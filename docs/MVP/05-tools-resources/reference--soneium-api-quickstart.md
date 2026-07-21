> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Soneium API Quickstart

> Source: [https://www.alchemy.com/docs/reference/soneium-api-quickstart.md](https://www.alchemy.com/docs/reference/soneium-api-quickstart.md)

# Soneium API Quickstart

> How to get started building on Soneium using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Soneium is an innovative Ethereum Layer 2 blockchain platform developed by Sony Block Solutions Labs. It aims to integrate Web3 technologies into everyday life, creating an inclusive digital world where everyone can be a creator and innovator. Soneium offers a robust infrastructure for developers to build impactful decentralized applications (dApps) and digital assets, while providing users with a secure and user-friendly environment for interacting with blockchain technology.

The Soneium API lets you interact with the Soneium network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Soneium client connected to Alchemy and fetch the latest block number!

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
import { soneium } from "viem/chains";

const client = createPublicClient({
  chain: soneium,
  transport: http("https://soneium-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Soneium APIs

For the full list of Soneium APIs, see the [Soneium API Endpoints](/docs/chains#soneium-apis).