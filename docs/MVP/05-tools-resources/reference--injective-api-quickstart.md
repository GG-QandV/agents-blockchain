> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Injective API Quickstart

> Source: [https://www.alchemy.com/docs/reference/injective-api-quickstart.md](https://www.alchemy.com/docs/reference/injective-api-quickstart.md)

# Injective API Quickstart

> How to get started building on Injective and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

*To use the Injective API, you need an Alchemy account. [Create a free account](https://dashboard.alchemy.com/signup) to get started.*

## What is Injective?

Injective is a high-performance Layer 1 blockchain built on the Cosmos SDK, purpose-built for decentralized finance (DeFi) applications such as decentralized exchanges, prediction markets, and lending protocols. It features full EVM compatibility, allowing developers to deploy Ethereum-based smart contracts while benefiting from fast finality and low transaction costs.

## What is the Injective API?

The Injective API lets you interact with Injective Mainnet or Injective Testnet through JSON-RPC. Mainnet chain ID is `0x6f0` (1776); testnet is `0x59f` (1439). If you've worked with Ethereum's JSON-RPC APIs, the interface will be familiar. For the full Node API method list, see the [Injective API overview](/docs/injective/injective-api-overview). Testnet-only methods document coverage on their own reference pages; use the [Debug API](/docs/reference/debug-api-quickstart) for `debug_*` RPC.

## Get started

### 1. Choose a package manager (npm or yarn)

Pick a package manager for your project's dependencies.

> 📄 **This content also appears in [Arc API Quickstart](05-tools-resources/reference--arc-api-quickstart.md)** — see there for full details.

  ```shell yarn
  # For yarn, refer to yarn's installation guide
  # https://classic.yarnpkg.com/lang/en/docs/install
  ```
</CodeGroup>

### 2. Set up your project

Run the following commands to create and initialize your project:

<CodeGroup>
  ```shell npm
  mkdir injective-api-quickstart
  cd injective-api-quickstart
  npm init --yes
  ```

  ```shell yarn
  mkdir injective-api-quickstart
  cd injective-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `injective-api-quickstart` and initializes a Node.js project within it.

### 3. Make your first request

Install Axios to make API requests:

<CodeGroup>
  ```shell npm
  npm install axios
  ```

  ```shell yarn
  yarn add axios
  ```
</CodeGroup>

Create an `index.js` file in your project directory and paste the following code:

<CodeGroup>
  ```javascript index.js
  const axios = require('axios');

  const url = "https://injective-mainnet.g.alchemy.com/v2/YOUR_API_KEY";

  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'eth_blockNumber',
    params: []
  };

  axios.post(url, payload)
    .then(response => {
      console.log('Latest Block:', response.data.result);
    })
    .catch(error => {
      console.error(error);
    });
  ```
</CodeGroup>

Replace `YOUR_API_KEY` with your Alchemy API key from the [Alchemy Dashboard](https://dashboard.alchemy.com/signup). For Injective Testnet, use `https://injective-testnet.g.alchemy.com/v2/YOUR_API_KEY` instead.

### 4. Run your script

Run your script to make a request to the Injective network:

<CodeGroup>
  ```shell shell
  node index.js
  ```
</CodeGroup>

You should see the latest block number from Injective in your console:

<CodeGroup>
  ```shell shell
  Latest Block: 0x...
  ```
</CodeGroup>

## Next steps

You've made your first request to the Injective network. Explore the JSON-RPC methods available on Injective and start building your dApps.