> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Katana API Quickstart

> Source: [https://www.alchemy.com/docs/reference/katana-api-quickstart.md](https://www.alchemy.com/docs/reference/katana-api-quickstart.md)

# Katana API Quickstart

> How to get started building on Katana and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

*To use the Katana API, you need an Alchemy account. [Create a free account](https://dashboard.alchemy.com/signup) to get started.*

## What is Katana?

Katana is an EVM-compatible network purpose-built for DeFi, concentrating liquidity and directing chain-level yield back to users and applications to deliver deep, sustainable onchain markets.

## What is the Katana API?

The Katana API lets you interact with the Katana network through a set of JSON-RPC methods. If you've worked with Ethereum's JSON-RPC APIs, the interface will be familiar.

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
  mkdir katana-api-quickstart
  cd katana-api-quickstart
  npm init --yes
  ```

  ```shell yarn
  mkdir katana-api-quickstart
  cd katana-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `katana-api-quickstart` and initializes a Node.js project within it.

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

  const url = 'https://katana-mainnet.g.alchemy.com/v2/your-api-key';

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

Run your script to make a request to the Katana network:

<CodeGroup>
  ```shell shell
  node index.js
  ```
</CodeGroup>

You should see the latest block number from Katana in your console:

<CodeGroup>
  ```shell shell
  Latest Block: 0x...
  ```
</CodeGroup>

## Next steps

You've made your first request to the Katana network. Explore the JSON-RPC methods available on Katana and start building your dApps.