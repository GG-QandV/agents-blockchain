# Utxo Websockets

**3 chains** · template-based (1 doc replaces 5 per-chain files)

## Common Template

The following content is **identical** across all chains:

# UTXO WebSockets

# UTXO WebSockets

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

# Supported Networks

# Connection

Open a WebSocket connection using the standard Alchemy WebSocket URL for the network:

# Message format

Every client message uses the following envelope:

```json
{
  "id": "1",
  "method": "<request or subscription name>",
  "params": { ... }
}
```

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

// then send the subscription
  {"id":"1","method":"subscribeNewBlock","params":{}}
  ```
</CodeGroup>

### Result

<CodeGroup>
  ```json result
  {"id":"1","data":{"subscribed":true}}

## subscribeNewTransaction

Emits every new transaction added to the blockchain, across all addresses.

### Parameters

* None.

### Response

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

## subscribeAddresses

### Parameters

### Response

### Request

### Result

<CodeGroup>
  ```json result
  {"id":"3","data":{"subscribed":true}}

## subscribeFiatRates

Emits an event when the fiat rate ticker for the chain's native asset updates.

### Parameters

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

# Request methods

# WebSocket limits

The following limits apply to Alchemy WebSocket connections:

* 100 concurrent connections per app on the FREE tier; 2,000 on all other tiers.
* 1,000 unique subscriptions per connection.

# See also

## Per-Chain Parameters

| Chain | Title | Rpc Url |
|-------|---|---|
| Bitcoin/Utxo Websockets | UTXO WebSockets | https://www.alchemy.com/docs/bitcoin/utxo-websockets... |
| Bitcoincash/Utxo Websockets | UTXO WebSockets | https://www.alchemy.com/docs/bitcoincash/utxo-websoc... |
| Litecoin/Utxo Websockets | UTXO WebSockets | https://www.alchemy.com/docs/litecoin/utxo-websocket... |

## Notes

- Template applies to all 3 chains with per-chain variable substitution only
- Per-chain values stored in `utxo-websockets_data.json`
- Chains with notable unique content (beyond name substitution):
  - Bitcoin/Utxo Websockets: ~221 unique lines
  - Bitcoincash/Utxo Websockets: ~185 unique lines
  - Litecoin/Utxo Websockets: ~185 unique lines

---
Source: 00-consolidated/utxo-websockets.md
