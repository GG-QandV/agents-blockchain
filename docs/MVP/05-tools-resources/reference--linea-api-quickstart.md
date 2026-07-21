> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Linea Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/linea-api-quickstart.md](https://www.alchemy.com/docs/reference/linea-api-quickstart.md)

# Linea Chain API Quickstart

> How to get started building on Linea using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Linea is an EVM-compatible blockchain designed to provide scalability and efficiency for decentralized applications (dApps). Known for its high throughput and low transaction costs, Linea offers a robust environment for deploying Ethereum-based applications.

The Linea API lets you interact with the Linea network through a collection of JSON-RPC methods. If you're familiar with Ethereum's JSON-RPC APIs, working with Linea will feel natural.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Linea client connected to Alchemy and fetch the latest block number!

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
import { linea } from "viem/chains";

const client = createPublicClient({
  chain: linea,
  transport: http("https://linea-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Linea APIs

For the full list of Linea APIs, see the [Linea API Endpoints](/docs/chains#linea-apis).