
# RedStone Documentation

- [TRANSACTION](#TRANSACTION)
- [NODE JSON API](#JSON-API)
- [NODE](#NODE)

## TRANSACTION


To make transaction cration easier we made own javascript lib! [LIB](https://github.com/RedStoneCoin/Redstone-js/)
Please check out redstone js, it has wallet creation, transaction sumbiter, hash, sign, verfy function in the js format compleatly offline and client side.

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
```
request: <node-ip>:8000/json_api/ping
respond: Pong!
```
#### Sumbit Transaction
[MORE INFO](#TRANSACTION)
**TAKES JSON REDSTONE TRANSACTION**
```
post request: request: <node-ip>:8000/json_api/submit_txn
response: Failure/Not Failure
```
#### Sumbit Transaction No Pow
Same thing just node is doing pow for transaction
```
post request: request: <node-ip>:8000/json_api/submit_txn_np
response: Failure/Not Failure
```
#### Get Transaction from mempool
- hash = hash or tx_id of your transaction
```
request: <node-ip>:8000/json_api/get_mem_tx/<hash>
response: Full transaction in json format if found
```
#### Get Account by address
- public_key = public key of your wallet not address!
```
request: <node-ip>:8000/json_api/get_acc/<public_key>
response: Full account details (Address Balance is smart-contract)
```
#### Create redstone keypair
Redstone keypairs are normal keypairs but to make things easier for future app developers we made it an api request!
*KEYPAIRS ARE CREATED ON SERVER SIDE FOR CLIENT SIDE CREATION PELEASE CHECK REDSTONE JS*
```
request: <node-ip>:8000/json_api/create_wallet
response: Public key, Private Key and Address
```
#### Sign
Signs a hash without wallet doing it
- pik = private key
- message = message you want to sign

```
request: <node-ip>:8000/json_api/sign/<pik>/<message>
response: singature in hex format
```
### Easy transaction send
Sends transaction without wallet doing crypto stuff


- pik = private key
- from = PUBLIC KEY!!!
- amount = float or not
- to = ADDRESS!!!
```
request: <node-ip>:8000/json_api/send_easy_transaction/<pik>/<from>/<amount>/<to>
```

#### Public key to Address
From public key retrives an address!
```
request: <node-ip>:8000/json_api/pkacc/<public_key>
returns: {address: address} 
```

## NODE
### NODE CLI

#### RPC
- port of the rpc
```
./Redstone-node --rpc <rpc_port>
```
#### TEST-NET
- enable test net
```
./Redstone-node --testnet
```
#### DISABLE API
- if used api will not be launched 
```
./Redstone-node --no_api
```
