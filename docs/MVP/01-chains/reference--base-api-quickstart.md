> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Base API Quickstart

> Source: [https://www.alchemy.com/docs/reference/base-api-quickstart.md](https://www.alchemy.com/docs/reference/base-api-quickstart.md)

# Base API Quickstart

> How to get started building on Base using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

Base offers a secure, affordable, and developer-centric Ethereum L2 solution. Designed with the vision of making blockchain more accessible, Base emerges from Coinbase's incubation, with clear intentions to move towards increased decentralization in the future. Its goal is to establish an inclusive, global cryptoeconomy. Notably, Base is developed with Optimism, leveraging the MIT-licensed OP Stack. This collaboration highlights Base's commitment to open-source principles and being in sync with the developer community's needs and aspirations.

The Base API is a collection of JSON-RPC methods that enable developers to interact with the Base network. Notably, it shares the same set of JSON-RPC methods as Optimism. For those already acquainted with [Optimism's API](/docs/reference/optimism-api-endpoints), diving into Base will feel both familiar and effortless.

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Base client connected to Alchemy and fetch the latest block number!

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
import { base } from "viem/chains";

const client = createPublicClient({
  chain: base,
  transport: http("https://base-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Base APIs

For the full list of Base APIs, see the [Base API Endpoints](/docs/chains#base-apis).