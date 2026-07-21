> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Kaia API Quickstart

> Source: [https://www.alchemy.com/docs/reference/kaia-api-quickstart.md](https://www.alchemy.com/docs/reference/kaia-api-quickstart.md)

# Kaia API Quickstart

> How to get started building on Kaia and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

*To use the Kaia API, you need an Alchemy account. [Create a free account](https://dashboard.alchemy.com/signup) to get started.*

## What is Kaia?

Kaia is a public, EVM-compatible Layer 1 blockchain created from the merger of Klaytn and Finschia, designed for high performance with one-second block times, low transaction fees, and instant finality to power large-scale Web3 and stablecoin applications.

## What is the Kaia API?

The Kaia API lets you interact with the Kaia network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the interface will be familiar.

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
  mkdir kaia-api-quickstart
  cd kaia-api-quickstart
  npm init --yes
  ```

  ```shell yarn
  mkdir kaia-api-quickstart
  cd kaia-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `kaia-api-quickstart` and initializes a Node.js project within it.

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

  const url = 'https://kaia-mainnet.g.alchemy.com/v2/your-api-key';

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

Replace `your-api-key` with your actual Alchemy API key from the [Alchemy Dashboard](https://dashboard.alchemy.com/signup).

### 4. Run your script

Run your script to make a request to the Kaia network:

<CodeGroup>
  ```shell shell
  node index.js
  ```
</CodeGroup>

You should see the latest block number from Kaia in your console:

<CodeGroup>
  ```shell shell
  Latest Block: 0x...
  ```
</CodeGroup>

## Next steps

You've made your first request to the Kaia network. Explore the JSON-RPC methods available on Kaia and start building your dApps.