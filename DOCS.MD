# RedStone Documentation

- [TRANSACTION](#TRANSACTION)
- [NODE JSON API](#JSON-API)
- [NODE](#NODE)

## TRANSACTION


To make transaction cration easier we made own javascript lib! [LIB](https://github.com/RedStoneCoin/Redstone-js/)

## JSON-API
Every json api request goes to the <node-ip>:8000/json_api/
There will be a lot more apis this is just start.

- ping
- submit_txn_v1
- submit_txn_np
- sign
- gettx (mempool)
- getacc
- create_wallet
- pkacc

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
#### Sumbit Transaction No Pow
Same thing just node is doing pow for transaction
```
post request: request: <node-ip>:8000/json_api/submit_txn_np
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
#### Sign
Signs a hash without wallet doing it
```
```
```
response: singature
```
### Easy transaction send
Sends transaction without wallet doing crypto stuff

```
pik = private key
from = PUBLIC KEY!!!
amount = float or not
to = ADDRESS!!!
request: <node-ip>:8000/json_api/send_easy_transaction/<pik>/<from>/<amount>/<to>
```

#### Public key to Address
From public key retrives an address!
```
request: <node-ip>:8000/json_api/pkacc/<publicc_key>
```
```
returns: {address: address} 
```

## NODE
### NODE CLI

#### RPC
```
./Redstone-node --rpc <rpc_port>
```
#### TEST-NET
```
./Redstone-node --testnet
```
#### DISABLE API
```
./Redstone-node --no_api
```
