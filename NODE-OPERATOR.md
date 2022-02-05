# Getting started up and running as a Competitors Club Node Operator 

---
1. [Start Competitors Club Node](#start-competitors-club-node)
2. [Create a Node Key](#create-a-node-key)
3. [Create a Aura and Gran Keys](#create-a-aura-and-grand-key)
4. [Add Keys to Keystore](#add-keys-to-keystore)
* [Optional - Use Curl](#option-use-curl)
* [Optional - Verify Keys in the Keystore](#verify-keys-in-the-keystore)
5. [Limiting Resources](#limiting-resources)
6. [Next Step CSOV API](#next-step-scov-api)
7. [Reporting Issues](reporting-issues)
## Start a Competitors Club Node Using Docker Compose
---
You can use the following docker-compose.yml file:
```bash
version: '3.2'
services:
  cclub:
    container_name: competitors-club-node-0
    image: jesselurie6/competitors-club:v1.1.0
    ports:
      - 30333:30333 # p2p port
      - 9933:9933 # rpc port
      - 9944:9944 # ws port
    volumes:
      - /my/local/folder:/competitors-club
    command: [
      "--name", "<node-name>",
      "--node-key=<your-node-key>",
      "--base-path", "/competitors-club/<node-name>",
      "--bootnodes", "/dns/node0.competitors.club/tcp/30333/p2p/12D3KooWE8GWrMyemMVz2m6nJj2gqYGxzLcMwWzisBumRJqUmmhD",
      "--port","30333",
      "--ws-port","9944",
      "--rpc-port", "9933",
      "--unsafe-ws-external",
      "--unsafe-rpc-external",
      "--rpc-methods", "Unsafe",
      "--ws-external",
      "--rpc-external",
      "--rpc-cors", "all",
    ]
```
For more information about the flags
```bash
docker run jesselurie6/competitors-club:v1.1.0 --help
```
---
With following docker-compose.yml you can set up a node and use polkadot-js-apps as the front end on port 80. After starting the node use a browser and enter your Docker host ip in the url field: _<http://[YOUR_DOCKER_HOST_IP>_ and then go to settings -> Developer & copy/past and save the following:
``` bash
{
  "Address": "MultiAddress",
  "LookupSource": "MultiAddress",
  "ClassId": "u64",
  "PeerId": "[u8;32]",
  "ClassInfoOf": "ClassId",
  "BalanceOf": "Balance",
  "BalanceReservableOf": "BalanceOf",
  "Moment": "u64",
  "Place": {
    "spot": "u32",
    "payout": "Balance"
  },
  "Competitor": {
    "vie_id": "[u8;16]",
    "staked": "bool",
    "submitted_winner": "bool"
  },
  "Vie": {
    "operator": "AccountId",
    "stake": "Balance",
    "podium": "Vec<Place>",
    "date": "Moment",
    "competitors": "Vec<AccountId>",
    "memo": "Vec<u8>"
  },
  "VieOf": "Vie",
  "VieReq": {
    "stake": "Balance",
    "podium": "Vec<Place>",
    "competitors": "Vec<AccountId>",
    "memo": "Vec<u8>"
  },
  "VieRequestOf": "VieReq",
  "Participants": {
    "AccountId": "[u8;16]"
  },
  "Operators": {
    "AccountId": "[u8;16]"
  },
  "StandingReq": {
    "competitor": "AccountId",
    "spot": "u32"
  },
  "PodiumReq": {
    "champion": "AccountId",
    "podium": "Vec<StandingReq>"
  },
  "PodiumReqOf": "PodiumReq",
  "Trophy": {
    "trophy": "[u8; 16]",
    "competitors": "Vec<AccountId>",
    "stake": "Balance",
    "memo": "Vec<u8>",
    "time": "u64",
    "podium": "Vec<StandingReq<AccountId>>"
  },
  "TokenId": "u64",
  "TokenInfo": {
    "metadata": "Vec<u8>",
    "owner": "AccountId",
    "data": "Trophy"
  },
  "TokenInfoOf": "TokenInfo",
  "AssetId": "u32",
  "TAssetBalance": "Balance",
  "DepositBalance": "Balance",
  "DepositBalanceOf": "Balance",
  "AssetDetails": {
    "owner": "AccountId",
    "issuer": "AccountId",
    "admin": "AccountId",
    "freezer": "AccountId",
    "supply": "Balance",
    "deposit": "DepositBalance",
    "min_balance": "Balance",
    "is_sufficient": "bool",
    "accounts": "u32",
    "sufficients": "u32",
    "approvals": "u32",
    "is_frozen": "bool"
  },
  "AssetMetadata": {
    "name": "Vec<u8>",
    "symbol": "Vec<u8>",
    "decimals": "u8",
    "is_frozen": "bool"
  },
  "Approval": {
    "amount": "Balance",
    "deposit": "DepositBalance"
  }
}```
  ---
```bash
version: '3.2'
services:
  cclub:
    container_name: competitors-club-node0
    image: jesselurie6/competitors-club:v1.1.0
    ports:
      - 30333:30333 # p2p port
      - 9933:9933 # rpc port
      - 9944:9944 # ws port
    volumes:
      - /my/local/folder:/competitors-club
    command: [
       "--name", "<node-name>",
      "--node-key=<your-node-key>",
      "--base-path", "/competitors-club/<node-name>",
      "--bootnodes", "/dns/node0.competitors.club/tcp/30333/p2p/12D3KooWE8GWrMyemMVz2m6nJj2gqYGxzLcMwWzisBumRJqUmmhD",
      "--port","30333",
      "--ws-port","9944",
      "--rpc-port", "9933",
      "--unsafe-ws-external",
      "--unsafe-rpc-external",
      "--rpc-methods", "Unsafe",
      "--ws-external",
      "--rpc-external",
      "--rpc-cors", "all",
    ]
  walletui:
    container_name: walletui
    image: jacogr/polkadot-js-apps
    environment:
      - WS_URL=ws://localhost:9944
    ports:
      - 80:80
    links:
      - cclub
```
## Create a Node Key
---
Competitors club is a `permissioned network.` To join you need to generate a node key using [subkey](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#generating-node-keys).
**Upon installation run the following command then send your node key to jesseklurie@gmail.com. You wont be able to join the network or sync until this step is done.**
```bash
 subkey generate-node-key
 OUTPUT:
 12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 // this is PeerId.
c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a // This is node-key.
```
## Create Aura and Gran keys
---
You will need to generate `2 keys.` Every node will need to have its own keys.
Competitors Club utilized the same format as the polkadot network. Notice the flag --network polkadot
Generate a mnemonic, the sr25519 key and address associated with it.
This key will be used by `Aura` for `block production`.
```bash
# subkey command
subkey generate --scheme sr25519 --network polkadot
```
```bash
# subkey output
Secret phrase `infant salmon buzz patrol maple subject turtle cute legend song vital leisure` is account:
  Secret seed:      0xa2b0200f9666b743402289ca4f7e79c9a4a52ce129365578521b0b75396bd242
  Public key (hex): 0x0a11c9bcc81f8bd314e80bc51cbfacf30eaeb57e863196a79cccdc8bf4750d21
  Account ID:       0x0a11c9bcc81f8bd314e80bc51cbfacf30eaeb57e863196a79cccdc8bf4750d21
  SS58 Address:     5CHucvTwrPg8L2tjneVoemApqXcUaEdUDsCEPyE7aDwrtR8D
```
Now see the ed25519 key and address associated with the same mnemonic.
This key will be used by GRANDPA for block finalization.
```bash
# subkey command
subkey inspect --scheme ed25519 --network polkadot "infant salmon buzz patrol maple subject turtle cute legend song vital leisure"
```
```bash
# subkey output
Secret phrase `infant salmon buzz patrol maple subject turtle cute legend song vital leisure` is account:
  Secret seed:      0xa2b0200f9666b743402289ca4f7e79c9a4a52ce129365578521b0b75396bd242
  Public key (hex): 0x1a0e2bf1e0195a1f5396c5fd209a620a48fe90f6f336d89c89405a0183a857a3
  Account ID:       0x1a0e2bf1e0195a1f5396c5fd209a620a48fe90f6f336d89c89405a0183a857a3
  SS58 Address:     5CesK3uTmn4NGfD3oyGBd1jrp4EfRyYdtqL3ERe9SXv8jUHb
```
## Add Keys to Keystore
---
Once your node is running, you will again notice that no `blocks` are being `produced`. At this point, you need to `add` your `keys` into the keystore.
```bash
#INSERT AURA KEY
curl -X POST -d '{
"jsonrpc":"2.0",
"id":1,
"method":"author_insertKey",
"params": [
"aura",
"your mnemonic your mnemonic your mnemonic your mnemonic", \\mnemonic
"0x0a11c9bcc81f8bd314e80bc51cbfacf30eaeb57e863196a79cccdc8bf4750d21" \\public key
]
}' -H 'Content-Type: application/json' http://127.0.0.1:9939/author_insertKey
#INSERT GRAN KEY
curl -X POST -d '{
"jsonrpc":"2.0",
"id":1,
"method":"author_insertKey",
"params": [
"gran",
"your mnemonic your mnemonic your mnemonic your mnemonic", \\mnemonic
"0x1a0e2bf1e0195a1f5396c5fd209a620a48fe90f6f336d89c89405a0183a857a3" \\public key
]
}' -H 'Content-Type: application/json' http://127.0.0.1:9939/author_insertKey
```
If you enter the command and parameters correctly, the node will return a JSON response as follows.
```bash
{ "jsonrpc": "2.0", "result": null, "id": 1 }
```
## Verify Keys in the Keystore 
---
If you would like to check that your keys are now loaded, you can view the keystore files that should now exists for your node. These are found in the following location:
```bash
# The path stems from `--base-path` and ID from `chain_spec.rs` ID field.
# Keys are then in `chains/<chain ID>/keystore :
ls /tmp/node01/chains/mainnet/keystore
```
```bash
## list of keystore files:
617572619effc1668ca381c242885516ec9fa2b19c67b6684c02a8a3237b6862e5c8cd7e
6772616eb48004c6e1625282313b07d1c9950935e86894a2e4f21fb1ffee9854d180c781
# Read a keystore file (our demo seed 1 was used)
cat /tmp/node01/chains/mainnet/keystore/617572619effc1668ca381c242885516ec9fa2b19c67b6684c02a8a3237b6862e5c8cd7e
"clip organ olive upper oak void inject side suit toilet stick narrow"
```
Notice there are two keystores, as expected as we added two keys to our node.

## Limiting Resources
---
Chain syncing will utilize all available memory and CPU power your server has to offer, which can lead to crashing.
If running on a low resource VPS, use `--memory` and `--cpus` to limit the resources used. E.g. To allow a maximum of 512MB memory and 50% of 1 CPU, use `--cpus=".5" --memory="512m"`. Read more about limiting a container's resources [here](https://docs.docker.com/config/containers/resource_constraints).

## Reporting Issues
If you run into issues please email jesseklurie@gmail.com