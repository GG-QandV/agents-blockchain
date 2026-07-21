> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Plasma API Quickstart

> Source: [https://www.alchemy.com/docs/reference/plasma-api-quickstart.md](https://www.alchemy.com/docs/reference/plasma-api-quickstart.md)

# Plasma API Quickstart

> How to get started building on Plasma using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Plasma is an EVM-compatible Layer-1 purpose-built for stablecoins, using PlasmaBFT (Fast HotStuff) and a native Bitcoin bridge to enable near-instant, zero-fee USD₮ transfers and confidential payments.

The Plasma API lets you interact with the Plasma network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Plasma client connected to Alchemy and fetch the latest block number!

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

const plasma = defineChain({
  id: 3338,
  name: "Plasma",
  nativeCurrency: { name: "ETH", symbol: "ETH", decimals: 18 },
  rpcUrls: {
    default: { http: ["https://plasma-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"] },
  },
});

const client = createPublicClient({
  chain: plasma,
  transport: http("https://plasma-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Plasma APIs

For the full list of Plasma APIs, see the [Plasma API Endpoints](/docs/chains#plasma-apis).