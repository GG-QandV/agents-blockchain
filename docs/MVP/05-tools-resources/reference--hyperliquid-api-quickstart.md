> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# HyperEVM API Quickstart

> Source: [https://www.alchemy.com/docs/reference/hyperliquid-api-quickstart.md](https://www.alchemy.com/docs/reference/hyperliquid-api-quickstart.md)

# HyperEVM API Quickstart

> How to get started building on HyperEVM using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Check out the [Hyperliquid Alchemy Wallets Guide](https://www.alchemy.com/docs/wallets/recipes/hyperliquid-wallets) for enabling email and social login for user authentication and the ability to sign and send transactions.

HyperEVM is a high-performance, Ethereum-compatible execution layer developed by HyperLiquid that enables smart contracts to run natively alongside its decentralized trading engine, combining low-latency performance with full EVM compatibility.

The HyperEVM API lets you interact with the HyperEVM network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a HyperEVM client connected to Alchemy and fetch the latest block number!

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
import { hyperliquid } from "viem/chains";

const client = createPublicClient({
  chain: hyperliquid,
  transport: http("https://hyperliquid-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (HYPE):", Number(balance) / 1e18);
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

# HyperEVM APIs

For the full list of HyperEVM APIs, see the [HyperEVM API Endpoints](/docs/chains#hyperliquid-apis).