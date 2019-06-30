// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! The following is a minimalist version of a hierarchical key derivation library for the
//! LibraWallet.
//!
//! Note that the Libra Blockchain makes use of ed25519 Edwards Digital Signature Algorithm
//! (EdDSA) and therefore, BIP32 Public Key derivation is not available without falling back to
//! a non-deterministic Schnorr signature scheme. As LibraWallet is meant to be a minimalist
//! reference implementation of a simple wallet, the following does not deviate from the
//! ed25519 spec. In a future iteration of this wallet, we will also provide an implementation
//! of a Schnorr variant over curve25519 and demonstrate our proposal for BIP32-like public key
//! derivation.
//!
//! Note further that the Key Derivation Function (KDF) chosen in the derivation of Child
//! Private Keys adheres to [HKDF RFC 5869](https://tools.ietf.org/html/rfc5869).

use byteorder::{ByteOrder, LittleEndian};
use crypto::{hmac::Hmac as CryptoHmac, pbkdf2::pbkdf2, sha3::Sha3};
use ed25519_dalek;
use libra_crypto::{hash::HashValue, hkdf::Hkdf};
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use std::{convert::TryFrom, ops::AddAssign, collections::HashMap};
use tiny_keccak::Keccak;
use types::account_address::AccountAddress;
use two_party_eddsa_client::api::*;
use hex::FromHex;

use crate::error::Result;

/// Master is a set of raw bytes that are used for child key derivation
pub struct Master([u8; 32]);
impl_array_newtype!(Master, u8, 32);
impl_array_newtype_show!(Master);
impl_array_newtype_encodable!(Master, u8, 32);

/// A child number for a derived key, used to derive a certain private key from the Master
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ChildNumber(pub(crate) u64);

impl ChildNumber {
    /// Constructor from u64
    pub fn new(child_number: u64) -> Self {
        Self(child_number)
    }

    /// Bump the ChildNumber
    pub fn increment(&mut self) {
        self.add_assign(Self(1));
    }
}

impl std::ops::AddAssign for ChildNumber {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0)
    }
}

impl std::convert::AsRef<u64> for ChildNumber {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl std::convert::AsMut<u64> for ChildNumber {
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

/// Derived private key.
// TODO: delete after debugging
#[derive(Debug, Clone)]
pub struct ExtendedPrivKey {
    client_shim: ClientShim,
    key_pair: KeyPair,
    aggregated_public_key: KeyAgg,
    id: String,
}

impl ExtendedPrivKey {
    pub fn new(client_shim: ClientShim, key_pair: KeyPair, aggregated_public_key: KeyAgg, id: String) -> Self {
        Self {
            client_shim,
            key_pair,
            aggregated_public_key,
            id,
        }
    }

    pub fn get_public(&self) -> ed25519_dalek::PublicKey {
        let public_key_bytes = hex::decode(self.aggregated_public_key.apk.bytes_compressed_to_big_int().to_hex()).unwrap();
        println!("ExtendedPrivKey::get_public - public_key_bytes = {:?}", public_key_bytes);
        ed25519_dalek::PublicKey::from_bytes(&public_key_bytes.as_slice())
            .expect("Error while creating public key from bytes")
    }

    /// Computes the sha3 hash of the PublicKey and attempts to construct a Libra AccountAddress
    /// from the raw bytes of the pubkey hash
    pub fn get_address(&self) -> Result<AccountAddress> {
        let public_key = self.get_public();
        let mut keccak = Keccak::new_sha3_256();
        let mut hash = [0u8; 32];
        keccak.update(&public_key.to_bytes());
        keccak.finalize(&mut hash);
        let addr = AccountAddress::try_from(&hash[..])?;
        Ok(addr)
    }


    /// Libra specific sign function that is capable of signing an arbitrary HashValue
    /// NOTE: In Libra, we do not sign the raw bytes of a transaction, instead we sign the raw
    /// bytes of the sha3 hash of the raw bytes of a transaction. It is important to note that the
    /// raw bytes of the sha3 hash will be hashed again as part of the ed25519 signature algorithm.
    /// In other words: In Libra, the message used for signature and verification is the sha3 hash
    /// of the transaction. This sha3 hash is then hashed again using SHA512 to arrive at the
    /// deterministic nonce for the EdDSA.
    /// TODO: check if there's no need to do SHA3 here
    pub fn sign(&self, msg: HashValue) -> ed25519_dalek::Signature {
        let message = BigInt::from(msg.to_vec().as_slice());
        println!("ExtendedPrivKey::sign - message = {:?}", message);
        let signature = two_party_eddsa_client::api::sign(&self.client_shim, message, &self.key_pair, &self.aggregated_public_key, &self.id)
            .expect("Error while signing");
        println!("R = {:?}", signature.R.bytes_compressed_to_big_int().to_hex());
        let R = format!("{:0>64}", signature.R.bytes_compressed_to_big_int().to_hex());
        println!("R = {:?}", R);
        let s_src = hex::decode(format!("{:0>64}",signature.s.to_big_int().to_hex())).unwrap();
        println!("s_src = {:?}", s_src);
        let mut s_dst: [u8; 32] = [0; 32];
        for i in 0..32 {
            println!("{}: copying {:?}", i, s_src[31 - i]);
            s_dst[i] = s_src[31 - i];
        }
        println!("s_dst = {:?}", s_dst);
        let s = format!("{}", hex::encode(s_dst));
        println!("s = {}", s);

        let v = Vec::from_hex(format!("{}{}", R, s)).unwrap();
        println!("vec = {:x?}", v.as_slice());

        ed25519_dalek::Signature::from_bytes(v.as_slice()).unwrap()
    }
}

fn vector_as_u8_32_array(vector: Vec<u8>) -> [u8; 32] {
    let mut arr = [0u8;32];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}

/// Wrapper struct from which we derive child keys
pub struct KeyFactory {
    client_shim: ClientShim,
    children: HashMap<u64, ExtendedPrivKey>,
}

impl KeyFactory {

    pub fn new() -> Result<Self> {
        let client_shim = ClientShim::new("http://localhost:8000".to_string());
        let children = HashMap::new();

        Ok(Self {
            client_shim,
            children
        })
    }

    pub fn private_child(&mut self, child_number: ChildNumber) -> Result<ExtendedPrivKey> {
        match self.children.get(child_number.as_ref()) {
            Some(extended_priv_key) => {
                println!("found child! {:?}", extended_priv_key);
                Ok(extended_priv_key.clone())
            },
            None => {
                println!("generating a child key!");
                let (key_pair, aggregated_public_key, id) = two_party_eddsa_client::api::generate_key(&self.client_shim).unwrap();
                let extended_priv_key = ExtendedPrivKey {
                    client_shim: self.client_shim.clone(),
                    key_pair,
                    aggregated_public_key,
                    id
                };
                println!("generated: {:?}", extended_priv_key);
                self.children.insert(child_number.as_ref().clone(), extended_priv_key.clone());
                Ok(extended_priv_key.clone())
            }
        }
    }
}

#[test]
fn assert_default_child_number() {
    assert_eq!(ChildNumber::default(), ChildNumber(0));
}

#[test]
fn test_key_derivation() {
    let data = hex::decode("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f").unwrap();
    let mnemonic = Mnemonic::from("legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will").unwrap();
    assert_eq!(
        mnemonic.to_string(),
        Mnemonic::mnemonic(&data).unwrap().to_string()
    );
    let seed = Seed::new(&mnemonic, "LIBRA");

    let key_factory = KeyFactory::new(&seed).unwrap();
    assert_eq!(
        "16274c9618ed59177ca948529c1884ba65c57984d562ec2b4e5aa1ee3e3903be",
        hex::encode(&key_factory.master())
    );

    // Check child_0 key derivation.
    let child_private_0 = key_factory.private_child(ChildNumber(0)).unwrap();
    assert_eq!(
        "358a375f36d74c30b7f3299b62d712b307725938f8cc931100fbd10a434fc8b9",
        hex::encode(&child_private_0.private_key.to_bytes()[..])
    );

    // Check determinism, regenerate child_0.
    let child_private_0_again = key_factory.private_child(ChildNumber(0)).unwrap();
    assert_eq!(
        hex::encode(&child_private_0.private_key.to_bytes()[..]),
        hex::encode(&child_private_0_again.private_key.to_bytes()[..])
    );

    // Check child_1 key derivation.
    let child_private_1 = key_factory.private_child(ChildNumber(1)).unwrap();
    assert_eq!(
        "a325fe7d27b1b49f191cc03525951fec41b6ffa2d4b3007bb1d9dd353b7e56a6",
        hex::encode(&child_private_1.private_key.to_bytes()[..])
    );

    let mut child_1_again = ChildNumber(0);
    child_1_again.increment();
    assert_eq!(ChildNumber(1), child_1_again);

    // Check determinism, regenerate child_1, but by incrementing ChildNumber(0).
    let child_private_1_from_increment = key_factory.private_child(child_1_again).unwrap();
    assert_eq!(
        "a325fe7d27b1b49f191cc03525951fec41b6ffa2d4b3007bb1d9dd353b7e56a6",
        hex::encode(&child_private_1_from_increment.private_key.to_bytes()[..])
    );
}
