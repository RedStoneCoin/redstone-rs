# Redstone-rs
![Github All Releases](https://img.shields.io/github/downloads/RedStoneCoin/redstone-rs/total.svg)
[![Join the chat at https://discordapp.com/invite/8ezwRUK](https://img.shields.io/discord/539606376339734558.svg?label=discord&logo=discord&logoColor=white)](https://discord.gg/JjThnVdASR)
[![GitHub contributors](https://img.shields.io/github/contributors-anon/Redstonecoin/redstone-rs?label=Contributors)](https://github.com/Redstonecoin/redstone-rs/graphs/contributors) [![GitHub issues](https://img.shields.io/github/issues/Redstonecoin/redstone-rs?label=Issues)](https://github.com/RedStoneCoin/redstone-rs/issues) ![GitHub stars](https://img.shields.io/github/stars/Redstonecoin/redstone-rs?label=Github%20Stars)

## Implementation of Redstone protocol
Redstone Project is a p2p custom blockchain implementation, comprised of a network of interconnected subchains providing improved speed and scalability over conventional cryptocurrenys.
This is the offical implemention of the redstone protocol. It is written in rust. Protocol is subject to frequent change and as such no complete documention exists (however it is in the works) It is currently not ready for usage.
[DOCS](https://github.com/RedStoneCoin/redstone-rs/blob/main/DOCS.MD)

## Table of Contents

- [How to compile](#how-to-compile)
  - [Linux](#linux)
    - [Prerequisites](#prerequisites)
      - [Generic Linux](#generic-linux)
- [Todo](https://pacific-philosophy-3dd.notion.site/38585e4797344b968e7cd9280ca714c7?v=5f039c4709b244c3a139572315a77ddf)

## How to compile

### Linux

#### Prerequisites

Redstone makes abundant use of Rust's syntax extensions and other advanced, unstable features. Because of this, you will need to use a nightly version of Rust. If you already have a working installation of the latest Rust nightly, feel free to skip to the next section.
To install a nightly version of Rust, we recommend using rustup. Install rustup by following the instructions on its website. Once rustup is installed, configure Rust nightly as your default toolchain by running the command:

```
rustup default nightly
```

To build redstone you also need installed dependencies listed bellow:

```
OpenSLL
GCC
Cmake
```

##### Generic Linux

Ensure you have the dependencies listed above.

```bash
git clone -b master --single-branch https://github.com/RedStoneCoin/redstone-rs/
cd redstone-rs/bin/node # for node
cd redstone-rs/bin/wallet # for wallet
cargo build --release
```
After the completion, the binaries will be in the `target/release` folder.


## Reporting a Vulnerability

If you the vulnerability is already publicy known or not explotable then please open an issue. (eg you can crash local nodes using x)
If it is a critical vulnerability that must be not known please contact us on redstonecrypto@gmail.com

# Contributors
- [Toni Dev](https://github.com/Toni-d-e-v) - Founder and primary developer of redstone
- [Leo Cornelius](https://github.com/leocornelius) - Core contributor to Redstone and secuirty advisary to the redstone team
- This could be you!

If you feel we have missed you out from this section please open an Issue or PR! Rest assured it is not intentional.
We welcome new contributors and team members. Please join our discord to get up to date, and feel free to open PRs and issues. Please be civil, developers are only human :)

# Thanks
A massive thanks to the amazing team at rust, for their groundbreaking contributions to the programing-scape!
We would also like to thank all of the projects, resources and individuals who aided us in learning about cryptocurrencies and hope our contributions are worth of your praise! Kudos to all OSS developers.
