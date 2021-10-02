# Redstone-rs

## Implementation of Redstone protocol

This is the offical implemention of the redstone protocol. It is written in rust. Protocol is subject to frequent change and as such no documention exists (however it is in the works) It is currently not ready for usage.

## Table of Contents

- [How to compile](#how-to-compile)
  - [Linux](#linux)
    - [Prerequisites](#prerequisites)
      - [Generic Linux](#generic-linux)
- [Todo](#To-Do)
- [Security Policy](#Security-Policy)

## How to compile

### Linux

#### Prerequisites

Rust makes abundant use of Rust's syntax extensions and other advanced, unstable features. Because of this, you will need to use a nightly version of Rust. If you already have a working installation of the latest Rust nightly, feel free to skip to the next section.
To install a nightly version of Rust, we recommend using rustup. Install rustup by following the instructions on its website. Once rustup is installed, configure Rust nightly as your default toolchain by running the command:

```
rustup default nightly
```

To build redstone you also need installed dependencies listed bellow:

```
openssl
```

```
Cmake
```

##### Generic Linux

Ensure you have the dependencies listed above.

```bash
git clone -b master --single-branch https://github.com/avrio-project/avrio-rs/
cd redstone-rs/src/node # for node
cd redstone-rs/src/wallet # for Wallet

cargo build --release
```

After the completion, the binaries will be in the `target/release` folder.

```bash
cd target
./redstone-node
# or wallet
./redstone-wallet
```

# To-Do

### core

- [ ] Finish p2p code.
- [ ] Handeler functions for the p2p code (eg when you recieve a block from a peer what do you do with it.
- [ ] Block enacting
- [X] Txn validation
- [X] Mempool (stores unvalidated txns) needs implmenting.
- [x] Accounts
- [ ] Validtor code (regarding the DpoS)
- [X] Fix POW for txns
- [ ] Smart Contract
- [ ] When transaction is executed add it to the db, so we can check if there is duplicate transaction

### Node

- [X] Json Api
- [X] Rpc
- [ ] CLI 
- [ ] Full Node

### Wallet

- [X] Wallet
- [X] Basic Wallet
- [X] Api for the node
- [X] Rpc
- [ ] Gui?!

# Security Policy

## Supported Versions

**NOTE**: There is currently NO supported version of redstone client

| Version | Supported |
| ------- | --------- |
| 0.0.1   | ❌        |

## Reporting a Vulnerability

If you the vulnerability is already publicy known or not explotable then please open an issue. (eg you can crash local nodes using x)
If it is a critical vulnerability that must be not known please contact us on redstonecrypto@gmail.com
