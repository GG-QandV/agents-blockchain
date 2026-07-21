> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Shape API Quickstart

> Source: [https://www.alchemy.com/docs/reference/shape-api-quickstart.md](https://www.alchemy.com/docs/reference/shape-api-quickstart.md)

# Shape API Quickstart

> How to get started building on Shape using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Shape is a chain for creators built on top of Ethereum. It's an open space where everyone is free to create, from fine art to experimental projects, fostering a unique cultural ecosystem. Shape offers creators the opportunity to claim back 80% of sequencer fees users spend interacting with their contracts, making it an attractive platform for innovative content creation.

The Shape API lets you interact with the Shape network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Shape client connected to Alchemy and fetch the latest block number!

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
import { shape } from "viem/chains";

const client = createPublicClient({
  chain: shape,
  transport: http("https://shape-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Shape APIs

For the full list of Shape APIs, see the [Shape API Endpoints](/docs/chains#shape-apis).