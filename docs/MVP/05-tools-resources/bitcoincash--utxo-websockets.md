> ⚠️ **This page is a template variant.** The consolidated content is in [utxo-websockets](../00-consolidated/utxo-websockets.md).
> Below is the original chain-specific version.

# UTXO WebSockets

> Source: [https://www.alchemy.com/docs/bitcoincash/utxo-websockets.md](https://www.alchemy.com/docs/bitcoincash/utxo-websockets.md)

# UTXO WebSockets

> Stream block, transaction, address, and fiat rate updates from Bitcoin Cash over a persistent WebSocket connection.

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

The UTXO WebSockets API streams real-time events from Bitcoin Cash over a persistent connection, powered by [Trezor Blockbook](https://github.com/trezor/blockbook). Use it to push-subscribe to new blocks, new transactions, transactions affecting specific addresses, and fiat rate updates, with the same data shapes returned by the [UTXO REST API](/docs/bitcoincash/utxo).

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

# Supported Networks

* `bitcoincash-mainnet`
* `bitcoincash-testnet`

# Connection

Open a WebSocket connection using the standard Alchemy WebSocket URL for the network:

<CodeGroup>
  ```shell wscat
  wscat -c wss://bitcoincash-mainnet.g.alchemy.com/v2/{apiKey}
  ```
</CodeGroup>

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

# Message format

Every client message uses the following envelope:

```json
{
  "id": "1",
  "method": "<request or subscription name>",
  "params": { ... }
}
```

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

# Subscriptions

## subscribeNewBlock

Emits an event each time a new block is added to the chain.

### Parameters

* None.

### Response

* `id`: `string` - The id you sent on the request.
* `data`:
  * `height`: `number` - Block height.
  * `hash`: `string` - Block hash.

### Request

<CodeGroup>
  ```shell wscat
  // open the websocket
  wscat -c wss://bitcoincash-mainnet.g.alchemy.com/v2/{apiKey}

  // then send the subscription
  {"id":"1","method":"subscribeNewBlock","params":{}}
  ```
</CodeGroup>

### Result

<CodeGroup>
  ```json result
  {"id":"1","data":{"subscribed":true}}

  {
    "id":"1",
    "data":{
      "height":860730,
      "hash":"000000000000000000a2b67ec3ea0b3a3a3a3b3a3a3a3a3a3a3a3a3a3a3a3a3a"
    }
  }
  ```
</CodeGroup>

## subscribeNewTransaction

Emits every new transaction added to the blockchain, across all addresses.

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

### Parameters

* None.

### Response

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

### Request

<CodeGroup>
  ```shell wscat
  {"id":"2","method":"subscribeNewTransaction","params":{}}
  ```
</CodeGroup>

### Result

<CodeGroup>
  ```json result
  {"id":"2","data":{"subscribed":true}}

  {
    "id":"2",
    "data":{
      "txid":"<bch txid>",
      "version":2,
      "vin":[
        {
          "txid":"<input txid>",
          "vout":1,
          "n":0,
          "addresses":["bitcoincash:qpm2qsznhks23z7629mms6s4cwef74vcwvy22gdx0"],
          "isAddress":true,
          "value":"10106300"
        }
      ],
      "vout":[
        {
          "value":"175000",
          "n":0,
          "addresses":["bitcoincash:qq3vlashthktqpeppuv7trmw062vfqkfk5l6w0jcuv"],
          "isAddress":true
        }
      ],
      "blockHeight":-1,
      "confirmations":0,
      "value":"10063100",
      "valueIn":"10106300",
      "fees":"43200"
    }
  }
  ```
</CodeGroup>

## subscribeAddresses

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

### Parameters

* `addresses`: `string[]` - One or more addresses to watch. Sending a new `subscribeAddresses` request replaces the previous list. Both CashAddr (`bitcoincash:q...`) and legacy (`1...`) formats are accepted.
* `newBlockTxs`: `boolean` *(optional)* - When `true`, also emits an event when a watched transaction is included in a new block, not just when it enters the mempool. Defaults to `false`.

### Response

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

### Request

<CodeGroup>
  ```shell wscat
  {
    "id":"3",
    "method":"subscribeAddresses",
    "params":{
      "addresses":[
        "bitcoincash:qpm2qsznhks23z7629mms6s4cwef74vcwvy22gdx0",
        "bitcoincash:qq3vlashthktqpeppuv7trmw062vfqkfk5l6w0jcuv"
      ],
      "newBlockTxs":true
    }
  }
  ```
</CodeGroup>

### Result

<CodeGroup>
  ```json result
  {"id":"3","data":{"subscribed":true}}

  {
    "id":"3",
    "data":{
      "address":"bitcoincash:qpm2qsznhks23z7629mms6s4cwef74vcwvy22gdx0",
      "tx":{
        "txid":"<bch txid>",
        "vin":[
          {
            "n":0,
            "addresses":["bitcoincash:qpm2qsznhks23z7629mms6s4cwef74vcwvy22gdx0"],
            "isAddress":true,
            "value":"10106300"
          }
        ],
        "vout":[
          {
            "value":"175000",
            "n":0,
            "addresses":["bitcoincash:qq3vlashthktqpeppuv7trmw062vfqkfk5l6w0jcuv"],
            "isAddress":true
          }
        ],
        "blockHeight":-1,
        "confirmations":0,
        "value":"10063100",
        "fees":"43200"
      }
    }
  }
  ```
</CodeGroup>

## subscribeFiatRates

Emits an event when the fiat rate ticker for the chain's native asset updates.

### Parameters

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

### Response

* `id`: `string` - The id you sent on the request.
* `data`:
  * Initial: `{ "subscribed": true }`.
  * On each event: `{ "rates": { "<currency>": <number>, ... } }`.

### Request

<CodeGroup>
  ```shell wscat
  {"id":"4","method":"subscribeFiatRates","params":{"currency":"usd"}}
  ```
</CodeGroup>

### Result

<CodeGroup>
  ```json result
  {"id":"4","data":{"subscribed":true}}

  {"id":"4","data":{"rates":{"usd":312.45}}}
  ```
</CodeGroup>

# Request methods

In addition to subscriptions, the WebSocket connection accepts the same one-shot requests as the [UTXO REST API](/docs/chains/bitcoincash/utxo-api-endpoints/utxo-api-endpoints). They are useful when you have already opened a connection for subscriptions and want to fetch ad-hoc data without a separate HTTP round-trip.

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

# WebSocket limits

The following limits apply to Alchemy WebSocket connections:

* 100 concurrent connections per app on the FREE tier; 2,000 on all other tiers.
* 1,000 unique subscriptions per connection.

> 📄 **This content also appears in [UTXO WebSockets](05-tools-resources/bitcoin--utxo-websockets.md)** — see there for full details.

# See also

* [UTXO Overview](/docs/bitcoincash/utxo)
* [UTXO API Endpoints](/docs/chains/bitcoincash/utxo-api-endpoints/utxo-api-endpoints)
* [UTXO Migration Guide](/docs/bitcoin/utxo-migration-guide)