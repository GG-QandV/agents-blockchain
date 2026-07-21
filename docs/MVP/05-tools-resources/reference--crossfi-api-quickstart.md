> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# CrossFi API Quickstart

> Source: [https://www.alchemy.com/docs/reference/crossfi-api-quickstart.md](https://www.alchemy.com/docs/reference/crossfi-api-quickstart.md)

# CrossFi API Quickstart

> How to get started building on CrossFi using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

CrossFi is a blockchain platform built to enable seamless cross-chain asset transfers and decentralized finance (DeFi) applications. With a focus on interoperability, security, and scalability, CrossFi provides developers and users with a powerful ecosystem for deploying decentralized applications (dApps) that can operate across multiple blockchain networks.

The CrossFi API lets you interact with the CrossFi network through a collection of JSON-RPC methods. If you're familiar with Ethereum's JSON-RPC APIs, working with CrossFi is intuitive and straightforward.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a CrossFi client connected to Alchemy and fetch the latest block number!

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
import { createPublicClient, http, defineChain } from "viem";

const crossfi = defineChain({
  id: 4158,
  name: "CrossFi",
  nativeCurrency: { name: "XFI", symbol: "XFI", decimals: 18 },
  rpcUrls: {
    default: { http: ["https://crossfi-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"] },
  },
});

const client = createPublicClient({
  chain: crossfi,
  transport: http("https://crossfi-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (XFI):", Number(balance) / 1e18);
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

# CrossFi APIs

For the full list of CrossFi APIs, see the [CrossFi API Endpoints](/docs/chains#crossfi-apis).