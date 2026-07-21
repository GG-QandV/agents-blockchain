> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# BNB Smart Chain Quickstart

> Source: [https://www.alchemy.com/docs/reference/bnb-smart-chain-api-quickstart.md](https://www.alchemy.com/docs/reference/bnb-smart-chain-api-quickstart.md)

# BNB Smart Chain Quickstart

> How to get started building on BNB Smart Chain using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

BNB Smart Chain (BSC) is a high-performance blockchain developed by Binance, designed to offer low transaction costs and fast execution times. It is Ethereum Virtual Machine (EVM) compatible, which lets you deploy Ethereum-based applications seamlessly.

The BNB Smart Chain API lets you interact with the BSC network through a collection of JSON-RPC methods. If you're familiar with Ethereum's JSON-RPC APIs, working with BSC is intuitive and straightforward.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a BNB Smart Chain client connected to Alchemy and fetch the latest block number!

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
import { bsc } from "viem/chains";

const client = createPublicClient({
  chain: bsc,
  transport: http("https://bnb-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (BNB):", Number(balance) / 1e18);
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

# BNB Smart Chain APIs

For the full list of BNB Smart Chain APIs, see the [BNB Smart Chain API Endpoints](/docs/chains#bnb-smart-chain-apis).