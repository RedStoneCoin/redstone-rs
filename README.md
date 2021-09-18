# Redstone-rs

## Implementation of Redstone protocol
This is the offical implemention of the redstone protocol. It is written in rust. Protocol is subject to frequent change and as such no documention exists (however it is in the works) It is currently not ready for usage.

# To-Do List
### core
- [ ] Finish p2p code. 
- [ ] Handeler functions for the p2p code (eg when you recieve a block from a peer what do you do with it. 
- [ ] Block enacting 
- [x] Txn validation (99% will be done when blockchain lib is done)
- [x] Mempool (stores unvalidated txns) needs implmenting.
- [ ] Accounts
- [ ] Validtor code (regarding the DpoS) 
- [x] Fix POW for txns
- [ ] Smart Contract  
- [ ] When recived sync message from wallet provider insted of sending test blocks send every block from the db
### Node
- [x] Json Api
- [x] Rpc
- [ ] Full Node
### Wallet
- [x] Wallet
- [x] Basic Wallet
- [x] Api for the node
- [x] Rpc
- [x] Send sync message to the node via rpc
- [ ] Gui?!


# Security Policy

## Supported Versions

**NOTE**: There is currently NO supported version of redstone client

| Version | Supported          |
| ------- | ------------------ |
| 0.0.1   | :x: |              |


## Reporting a Vulnerability

If you the vulnerability is already publicy known or not explotable then please open an issue. (eg you can crash local nodes using x)
If it is a critical vulnerability that must be not known please contact us on redstonecrypto@gmail.com
