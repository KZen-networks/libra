<a href="https://developers.libra.org">
	<img width="200" src="./.assets/libra.png" alt="Libra Logo" />
</a>

<hr/>

[![CircleCI](https://circleci.com/gh/libra/libra.svg?style=shield)](https://circleci.com/gh/libra/libra)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE.md)

------
**This is a forked repository to support _two-party signing_ in Libra**.<br>
It allows a more secure key management paradigm to protect your Libra funds:<br>
we move away the concept of a single _seed_, and instead support the co-signing between the _client_ and a _server_, each with an independently generated secret share.  
Both shares are required to produce a valid signature in order to move funds, and a single key is never present on a single place.<br>

**Try out our demo:**<br>
Launch the server:
```bash
$ cd server
$ cargo build --release
$ ../target/release/server_exec
```
Client (just like [My First Transaction](https://developers.libra.org/docs/my-first-transaction)):
```bash
$ scripts/cli/start_cli_testnet.sh
```
... and follow the CLI help for creating accounts and transferring Libra. 

|![demo](https://raw.githubusercontent.com/KZen-networks/libra/master/libra-tss-demo.gif "Libra Two-Party Wallet Demo")|
|:--:|

------

Libra Core implements a decentralized, programmable database which provides a financial infrastructure that can empower billions of people.

## Note to Developers
* Libra Core is a prototype.
* The APIs are constantly evolving and designed to demonstrate types of functionality. Expect substantial changes before the release.
* We’ve launched a testnet that is a live demonstration of an early prototype of the Libra Blockchain software.

## Contributing

Read our [Contributing guide](https://developers.libra.org/docs/community/contributing). Find out what’s coming on our [blog](https://developers.libra.org/blog/2019/06/18/the-path-forward).

## Getting Started

### Learn About Libra
* [Welcome](https://developers.libra.org/docs/welcome-to-libra)
* [Libra Protocol: Key Concepts](https://developers.libra.org/docs/libra-protocol)
* [Life of a Transaction](https://developers.libra.org/docs/life-of-a-transaction)

### Try Libra Core
* [My First Transaction](https://developers.libra.org/docs/my-first-transaction)
* [Getting Started With Move](https://developers.libra.org/docs/move-overview)

### Technical Papers
* [The Libra Blockchain](https://developers.libra.org/docs/the-libra-blockchain-paper)
* [Move: A Language With Programmable Resources](https://developers.libra.org/docs/move-paper)
* [State Machine Replication in the Libra Blockchain](https://developers.libra.org/docs/state-machine-replication-paper)

### Blog
* [Libra: The Path Forward](https://developers.libra.org/blog/2019/06/18/the-path-forward/)

### Libra Codebase

* [Libra Core Overview](https://developers.libra.org/docs/libra-core-overview)
* [Admission Control](https://developers.libra.org/docs/crates/admission-control)
* [Bytecode Verifier](https://developers.libra.org/docs/crates/bytecode-verifier)
* [Consensus](https://developers.libra.org/docs/crates/consensus)
* [Crypto](https://developers.libra.org/docs/crates/crypto)
* [Execution](https://developers.libra.org/docs/crates/execution)
* [Mempool](https://developers.libra.org/docs/crates/mempool)
* [Move IR Compiler](https://developers.libra.org/docs/crates/ir-to-bytecode)
* [Move Language](https://developers.libra.org/docs/crates/move-language)
* [Network](https://developers.libra.org/docs/crates/network)
* [Storage](https://developers.libra.org/docs/crates/storage)
* [Virtual Machine](https://developers.libra.org/docs/crates/vm)


## Community

* Join us on the [Libra Discourse](https://community.libra.org).
* Get the latest updates to our project by signing up for our [newsletter](https://developers.libra.org/newsletter_form).

## License

Libra Core is licensed as [Apache 2.0](https://github.com/libra/libra/blob/master/LICENSE).
