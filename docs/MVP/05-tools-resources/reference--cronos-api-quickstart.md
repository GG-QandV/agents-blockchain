> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Cronos API Quickstart

> Source: [https://www.alchemy.com/docs/reference/cronos-api-quickstart.md](https://www.alchemy.com/docs/reference/cronos-api-quickstart.md)

# Cronos API Quickstart

> How to get started building on Cronos using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Cronos is a public, open-source, EVM-compatible blockchain launched by Crypto.com in 2021. Built with the Cosmos SDK and Ethermint, Cronos aims to deliver low fees, fast finality, and full compatibility with Ethereum tooling, using `$CRO` as its native gas token.

The Cronos API lets you interact with the Cronos network through a set of JSON-RPC methods. If you have worked with Ethereum's JSON-RPC APIs, the interface will feel familiar.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Cronos client connected to Alchemy and fetch the latest block number.

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
import { cronos } from "viem/chains";

const client = createPublicClient({
  chain: cronos,
  transport: http("https://cronos-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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
console.log("Balance (CRO):", Number(balance) / 1e18);
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

# Cronos APIs

For the full list of Cronos APIs, see the [Cronos API Endpoints](/docs/chains#cronos-apis).