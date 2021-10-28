# RedStone Documentation

- [TRANSACTION](#TRANSACTION)
- [NODE JSON API](#JSON-API)

## TRANSACTION

HASH FUNCTION = ***BLAKE 2***
- Url must be redstone node json api
- Sender is senders ***PUBLIC KEY*** not address
- reciver is recivers ***ADDRESS***
- amount is int amount
- type_flag is allways 0 for send
- payload = hex encoded payload
- nonce = number in transaction that is hashed with transaction and hash starts with 4 zeros there pow
- pow = hash
- hash = hash
- signature = signature
How to generate POW?
Rust example:
```rust
        let mut rng = rand::thread_rng(); // rand generator
        for nonce_attempt in 0..=u64::MAX { 
            let nonce_attempt = rng.gen::<u64>(); // nonce attempt to be random
            self.nonce = nonce_attempt; // nonce = txc nonce
            let pow = self.hash_item(); // pow = hash transaction
            if pow.starts_with("0000") // if hash starts with '0000'
            {
                self.pow = self.hash_item();  // we found it pow = out hash
                self.hash = self.hash_item(); // our hash = hash
                self.nonce = nonce_attempt; // our nonce = nonce attempt
                println!("Found solution for , nonce {}, hash {}, hash value {}",self.nonce,self.hash,pow);
                break; // break loop
            }
            
        }
```
To make transaction cration easyier we made own javascript lib! [LIB]("https://github.com/RedStoneCoin/Redstone-js/")

## JSON-API
Every json api request goes to the <node-ip>:8000/json_api/
There will be a lot more apis this is just start.

- ping
- submit_txn_v1
- gettx (mempool)
- getacc
- create_wallet

#### PING

request: <node-ip>:8000/json_api/ping
respond: Pong!

#### Sumbit Transaction
```
post request: request: <node-ip>:8000/json_api/submit_txn
```
[MORE INFO](#TRANSACTION)
** TAKES JSON REDSTONE TRANSACTION **
```
response: Failure/Not Failure
```

#### Get Transaction from mempool
```
request: <node-ip>:8000/json_api/get_mem_tx/<hash>
```
```
response: Full transaction in json format if found
```
#### Get Account by address
request: <node-ip>:8000/json_api/get_acc/<public_key>
response: Full account details (Address Balance is smart-contract)
#### Create redstone keypair
Redstone keypairs are normal keypairs but to make things easier for future app developers we made it an api request!
```
request: <node-ip>:8000/json_api/create_wallet
```
```
response: Public key, Private Key and Address
```
