> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Lens API Quickstart

> Source: [https://www.alchemy.com/docs/reference/lens-api-quickstart.md](https://www.alchemy.com/docs/reference/lens-api-quickstart.md)

# Lens API Quickstart

> How to get started building on Lens using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Lens Network is a decentralized social media infrastructure built on Ethereum using zkSync's technology. It aims to create open and fair social spaces by leveraging hybrid scaling solutions for high-throughput, low-cost transactions. Lens empowers users with control over their social identity and connections, enabling seamless movement across applications and fostering an environment with reduced censorship and enhanced freedom of expression.

<Info>
  We currently only support Lens Sepolia testnet.
</Info>

The Lens API lets you interact with the Lens network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Lens client connected to Alchemy and fetch the latest block number!

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

const lensTestnet = defineChain({
  id: 37111,
  name: "Lens Sepolia Testnet",
  nativeCurrency: { name: "GRASS", symbol: "GRASS", decimals: 18 },
  rpcUrls: {
    default: { http: ["https://lens-sepolia.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"] },
  },
});

const client = createPublicClient({
  chain: lensTestnet,
  transport: http("https://lens-sepolia.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (GRASS):", Number(balance) / 1e18);
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

# Lens APIs

For the full list of Lens APIs, see the [Lens API Endpoints](/docs/chains#lens-apis).