> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# opBNB Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/opbnb-api-quickstart.md](https://www.alchemy.com/docs/reference/opbnb-api-quickstart.md)

# opBNB Chain API Quickstart

> How to get started building on opBNB using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

opBNB is an EVM-compatible blockchain designed to provide high performance, scalability, and security for decentralized applications (dApps). Known for its efficient transaction processing and low costs, opBNB offers a robust environment for deploying Ethereum-based applications.

The opBNB API lets you interact with the opBNB network through a collection of JSON-RPC methods. If you're familiar with Ethereum's JSON-RPC APIs, working with opBNB will feel natural.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create an opBNB client connected to Alchemy and fetch the latest block number!

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
import { opBNB } from "viem/chains";

const client = createPublicClient({
  chain: opBNB,
  transport: http("https://opbnb-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# opBNB APIs

For the full list of opBNB APIs, see the [opBNB API Endpoints](/docs/chains#opbnb-apis).