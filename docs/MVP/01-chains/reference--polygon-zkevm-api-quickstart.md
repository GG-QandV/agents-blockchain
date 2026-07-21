> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Polygon zkEVM API Quickstart

> Source: [https://www.alchemy.com/docs/reference/polygon-zkevm-api-quickstart.md](https://www.alchemy.com/docs/reference/polygon-zkevm-api-quickstart.md)

# Polygon zkEVM API Quickstart

> How to get started building on Polygon zkEVM using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

<Warning title="Polygon zkEVM deprecation">
  Polygon Labs is [shutting down the Polygon zkEVM network on July 1, 2026](https://polygon.technology/polygon-zkevm). Alchemy will fully deprecate support for **Polygon zkEVM Mainnet** (`polygonzkevm-mainnet`) and the **Cardona** testnet (`polygonzkevm-cardona`) on **July 1, 2026**. See the [Polygon zkEVM Deprecation Notice](/docs/reference/polygon-zkevm-deprecation-notice) for details.
</Warning>

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Polygon zkEVM is a decentralized Ethereum Layer 2 network that uses cryptographic zero-knowledge proofs to offer validity and quick finality to offchain transactions. Emulating the Ethereum Virtual Machine (EVM), zkEVM allows transparent deployment of existing Ethereum smart contracts while enhancing scalability, security, and transaction throughput. With zkEVM, you can build decentralized applications with quick finality and improved performance, all within the Ethereum ecosystem.

The Polygon zkEVM API is a collection of JSON-RPC methods that let you interact with the Polygon zkEVM network. Using the endpoints provided by the API, you can access up-to-date network data and submit transactions.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Polygon zkEVM client connected to Alchemy and fetch the latest block number!

<CodeGroup>
  ```text npm
  npm install --save viem
  ```

  ```text yarn
  yarn add viem
  ```
</CodeGroup>

## Create a client connected to Alchemy

<CodeGroup>
```js
import { createPublicClient, http } from "viem"; 
import { polygonZkEvm } from "viem/chains";

const client = createPublicClient({
  chain: polygonZkEvm,
  transport: http("https://polygonzkevm-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
});
```
</CodeGroup>

Now that you've created a client connected to Alchemy, you can continue with some basics:

## Get the latest block number

<CodeGroup>
```js
const blockNumber = await client.getBlockNumber();
console.log("Current block number:", blockNumber);
```
</CodeGroup>

## Get an address balance

> 📄 **This content also appears in [Abstract API Quickstart](05-tools-resources/reference--abstract-api-quickstart.md)** — see there for full details.

## Read block data

<CodeGroup>
```js
const block = await client.getBlock({
  blockNumber: blockNumber, // from previous example
});
console.log(block);
```
</CodeGroup>

## Fetch a transaction by hash

<CodeGroup> 
```js 
const tx = await client.getTransaction({ hash: "0xYOUR_TX_HASH" });
console.log(tx);
```
</CodeGroup>

## Fetch a transaction receipt

<CodeGroup>
```js
const receipt = await client.getTransactionReceipt({
  hash: "0xYOUR_TX_HASH"
});
console.log(receipt);
```
</CodeGroup>

# Polygon zkEVM APIs

For the full list of Polygon zkEVM APIs, see the [Polygon zkEVM API Endpoints](/docs/chains#polygon-zkevm-apis).