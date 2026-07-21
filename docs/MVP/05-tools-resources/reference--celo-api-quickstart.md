> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Celo Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/celo-api-quickstart.md](https://www.alchemy.com/docs/reference/celo-api-quickstart.md)

# Celo Chain API Quickstart

> How to get started building on Celo using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Celo is an EVM-compatible blockchain designed to provide financial tools and services to anyone with a mobile phone. Known for its focus on usability and inclusive finance, Celo offers a robust environment for deploying Ethereum-based applications with an emphasis on mobile-first experiences.

The Celo API lets you interact with the Celo network through a collection of JSON-RPC methods. If you're familiar with Ethereum's JSON-RPC APIs, working with Celo is intuitive and straightforward.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Celo client connected to Alchemy and fetch the latest block number!

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
import { celo } from "viem/chains";

const client = createPublicClient({
  chain: celo,
  transport: http("https://celo-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (CELO):", Number(balance) / 1e18);
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

# Celo APIs

For the full list of Celo APIs, see the [Celo API Endpoints](/docs/chains#celo-apis).