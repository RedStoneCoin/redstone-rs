# Redstone-rs

## Implementation of Redstone protocol
Redstone Project is a p2p custom blockchain implementation, where we have a network made of subchains for better speeds and scalability.
This is the offical implemention of the redstone protocol. It is written in rust. Protocol is subject to frequent change and as such no documention exists (however it is in the works) It is currently not ready for usage.

[DOCS](https://github.com/RedStoneCoin/redstone-rs/blob/main/DOCS.MD)

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
**To build GUI wallet on debian**:
```
sudo apt-get install xorg-dev
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
sudo apt-get install libavutil-dev
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

```
GCC
Cmake
```
** Fedora **
``
sudo yum groupinstall "X Software Development" && yum install pango-devel libXinerama-devel libX11-devel libXinerama-devel-1.1.4-9.fc35
pango-devel
``

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
cd target/debug
./redstone-node
# or wallet
./redstone-wallet
```

# To-Do

### core

- [ ] Finish p2p code.
- [ ] Handeler functions for the p2p code (eg when you recieve a block from a peer what do you do with it.
- [ ] Block enacting, Save block get block.
- [ ] Txn validation
- [ ] Validtor code (regarding the DpoS)
- [ ] Smart Contract
- [ ] When transaction is executed add it to the db, so we can check if there is duplicate transaction

### Node

- [X] Json Api:
- - [ ] Get Block
- - [ ] Get Transaction
- [ ] CLI 
- [ ] Full Node

## Wallet
- [x] DONE
 

# Security Policy

## Supported Versions

**NOTE**: There is currently NO supported version of redstone client

| Version | Supported |
| ------- | --------- |
| 0.0.1   | ❌        |

## Reporting a Vulnerability

If you the vulnerability is already publicy known or not explotable then please open an issue. (eg you can crash local nodes using x)
If it is a critical vulnerability that must be not known please contact us on redstonecrypto@gmail.com
