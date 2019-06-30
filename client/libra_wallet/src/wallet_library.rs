// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! The following document is a minimalist version of Libra Wallet. Note that this Wallet does
//! not promote security as the mnemonic is stored in unencrypted form. In future iterations,
//! we will be realesing more robust Wallet implementations. It is our intention to present a
//! foundation that is simple to understand and incrementally improve the LibraWallet
//! implementation and it's security guarantees throughout testnet. For a more robust wallet
//! reference, the authors suggest to audit the file of the same name in the rust-wallet crate.
//! That file can be found here:
//!
//! https://github.com/rust-bitcoin/rust-wallet/blob/master/wallet/src/walletlibrary.rs

use crate::{
    error::*,
    io_utils,
    key_factory::{ChildNumber, KeyFactory},
};
use libra_crypto::hash::CryptoHash;
use proto_conv::{FromProto, IntoProto};
use protobuf::Message;
use std::{collections::HashMap, path::Path};
use types::{
    account_address::AccountAddress,
    proto::transaction::SignedTransaction as ProtoSignedTransaction,
    transaction::{RawTransaction, RawTransactionBytes, SignedTransaction},
};

/// WalletLibrary contains all the information needed to recreate a particular wallet
pub struct WalletLibrary {
    key_factory: KeyFactory,
    addr_map: HashMap<AccountAddress, ChildNumber>,
    key_leaf: ChildNumber,
}

impl WalletLibrary {
    /// Constructor that generates a Mnemonic from OS randomness and subsequently instantiates an
    /// empty WalletLibrary from that Mnemonic
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            key_factory: KeyFactory::new().unwrap(),
            addr_map: HashMap::new(),
            key_leaf: ChildNumber(0),
        }
    }

    /// Function that writes the wallet Mnemonic to file
    /// NOTE: This is not secure, and in general the Mnemonic would need to be decrypted before it
    /// can be written to file; otherwise the encrypted Mnemonic should be written to file
    pub fn write_recovery(&self, output_file_path: &Path) -> Result<()> {
        io_utils::write_recovery(&self, &output_file_path)?;
        Ok(())
    }

    /// Recover wallet from input_file_path
    pub fn recover(input_file_path: &Path) -> Result<WalletLibrary> {
        let wallet = io_utils::recover(&input_file_path)?;
        Ok(wallet)
    }

    /// Get the current ChildNumber in u64 format
    pub fn key_leaf(&self) -> u64 {
        self.key_leaf.0
    }

    /// Function that iterates from the current key_leaf until the supplied depth
    pub fn generate_addresses(&mut self, depth: u64) -> Result<()> {
        let current = self.key_leaf.0;
        if current > depth {
            return Err(WalletError::LibraWalletGeneric(
                "Addresses already generated up to the supplied depth".to_string(),
            ));
        }
        while self.key_leaf != ChildNumber(depth) {
            let _ = self.new_address();
        }
        Ok(())
    }

    /// Function that allows to get the address of a particular key at a certain ChildNumber
    pub fn new_address_at_child_number(
        &mut self,
        child_number: ChildNumber,
    ) -> Result<AccountAddress> {
        let child = self.key_factory.private_child(child_number)?;
        child.get_address()
    }

    /// Function that generates a new key and adds it to the addr_map and subsequently returns the
    /// AccountAddress associated to the PrivateKey, along with it's ChildNumber
    pub fn new_address(&mut self) -> Result<(AccountAddress, ChildNumber)> {
        let child = self.key_factory.private_child(self.key_leaf)?;
        let address = child.get_address()?;
        let child = self.key_leaf;
        self.key_leaf.increment();
        match self.addr_map.insert(address, child) {
            Some(_) => Err(WalletError::LibraWalletGeneric(
                "This address is already in your wallet".to_string(),
            )),
            None => Ok((address, child)),
        }
    }

    /// Returns a list of all addresses controlled by this wallet that are currently held by the
    /// addr_map
    pub fn get_addresses(&self) -> Result<Vec<AccountAddress>> {
        let mut ret = Vec::with_capacity(self.addr_map.len());
        let rev_map = self
            .addr_map
            .iter()
            .map(|(&k, &v)| (v.as_ref().to_owned(), k.to_owned()))
            .collect::<HashMap<_, _>>();
        for i in 0..self.addr_map.len() as u64 {
            match rev_map.get(&i) {
                Some(account_address) => {
                    ret.push(*account_address);
                }
                None => {
                    return Err(WalletError::LibraWalletGeneric(format!(
                        "Child num {} not exist while depth is {}",
                        i,
                        self.addr_map.len()
                    )))
                }
            }
        }
        Ok(ret)
    }

    /// Simple public function that allows to sign a Libra RawTransaction with the PrivateKey
    /// associated to a particular AccountAddress. If the PrivateKey associated to an
    /// AccountAddress is not contained in the addr_map, then this function will return an Error
    pub fn sign_txn(
        &mut self,
        addr: &AccountAddress,
        txn: RawTransaction,
    ) -> Result<SignedTransaction> {
        if let Some(child) = self.addr_map.get(addr) {
            let raw_bytes = txn.into_proto().write_to_bytes()?;
            let txn_hashvalue = RawTransactionBytes(&raw_bytes).hash();

            let child_key = self.key_factory.private_child(child.clone())?;
            let signature = child_key.sign(txn_hashvalue);
            let public_key = child_key.get_public();

            let mut signed_txn = ProtoSignedTransaction::new();
            signed_txn.set_raw_txn_bytes(raw_bytes.to_vec());
            signed_txn.set_sender_public_key(public_key.to_bytes().to_vec());
            signed_txn.set_sender_signature(signature.to_bytes().to_vec());

            Ok(SignedTransaction::from_proto(signed_txn)?)
        } else {
            Err(WalletError::LibraWalletGeneric(
                "Well, that address is nowhere to be found... This is awkward".to_string(),
            ))
        }
    }
}
