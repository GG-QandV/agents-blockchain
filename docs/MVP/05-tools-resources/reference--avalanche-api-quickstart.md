> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Avalanche C-Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/avalanche-api-quickstart.md](https://www.alchemy.com/docs/reference/avalanche-api-quickstart.md)

# Avalanche C-Chain API Quickstart

> How to get started building on Avalanche using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Avalanche is an EVM-compatible blockchain known for its high throughput and low latency. Designed to support the needs of decentralized applications (dApps), Avalanche provides a scalable and efficient environment for deploying Ethereum-based applications with near-instant finality.

The Avalanche C-Chain API facilitates interaction with the Avalanche network through a collection of JSON-RPC methods. Given its compatibility with the Ethereum ecosystem, developers familiar with Ethereum's JSON-RPC APIs will find working with Avalanche both intuitive and straightforward.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create an Avalanche client connected to Alchemy and fetch the latest block number!

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
import { avalanche } from "viem/chains";

const client = createPublicClient({
  chain: avalanche,
  transport: http("https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

<CodeGroup>
```js 
const balance = await client.getBalance({ address: "0xab5801a7d398351b8be11c439e05c5b3259aec9b" });
console.log("Balance (AVAX):", Number(balance) / 1e18);
```
</CodeGroup>

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

# Avalanche APIs

For the full list of Avalanche APIs, see the [Avalanche API Endpoints](/docs/chains#avalanche-apis).