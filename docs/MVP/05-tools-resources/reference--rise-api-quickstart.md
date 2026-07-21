> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Rise API Quickstart

> Source: [https://www.alchemy.com/docs/reference/rise-api-quickstart.md](https://www.alchemy.com/docs/reference/rise-api-quickstart.md)

# Rise API Quickstart

> How to get started building on Rise using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

RISE is an Ethereum Layer-2 with a parallel EVM and a "based" hybrid rollup design that targets sub-5 ms confirmations and 100k+ TPS.

The Rise API lets you interact with the Rise network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the design will feel familiar.

Alchemy supports both Rise networks. Use the endpoint that matches the network you want to connect to:

| Network | RPC URL |
| ------- | ------- |
| Rise Mainnet | `https://rise-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY` |
| Rise Testnet | `https://rise-testnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY` |

## Send your first request on Alchemy

Let's use the [`viem`](https://www.npmjs.com/package/viem) package to create a Rise client connected to Alchemy and fetch the latest block number!

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

const rise = defineChain({
  id: 11155931,
  name: "Rise",
  nativeCurrency: { name: "ETH", symbol: "ETH", decimals: 18 },
  rpcUrls: {
    default: { http: ["https://rise-testnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"] },
  },
});

const client = createPublicClient({
  chain: rise,
  transport: http("https://rise-testnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY"),
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

# Rise APIs

For the full list of Rise APIs, see the [Rise API Endpoints](/docs/chains#rise-apis).