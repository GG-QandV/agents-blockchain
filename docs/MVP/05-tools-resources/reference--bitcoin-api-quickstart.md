> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Bitcoin API Quickstart

> Source: [https://www.alchemy.com/docs/reference/bitcoin-api-quickstart.md](https://www.alchemy.com/docs/reference/bitcoin-api-quickstart.md)

# Bitcoin API Quickstart

> Get started building on Bitcoin and using the JSON-RPC API

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

# Introduction

The Bitcoin API gives you access to interact with the Bitcoin blockchain through a standard set of JSON-RPC methods. With this API, you can retrieve block and transaction data, inspect the mempool, broadcast transactions, and more.

Unlike EVM chains, Bitcoin uses a UTXO-based model and exposes its functionality via the JSON-RPC protocol. This quickstart will help you make your first request.

***

## What is the Bitcoin Chain API?

The Bitcoin Chain API allows applications to communicate with a Bitcoin node using the JSON-RPC protocol. Unlike Ethereum’s account-based model, Bitcoin relies on a UTXO (Unspent Transaction Output) model, which means that balances are tracked by outputs that are explicitly spent or unspent.

Alchemy’s Bitcoin API gives developers a consistent interface to query blockchain data, submit transactions, and monitor network activity. This includes:

* Retrieving raw or decoded transaction data
* Querying block headers and full blocks
* Monitoring mempool entries and states
* Submitting and testing raw transactions

If you’ve worked with Ethereum or other JSON-RPC-compatible chains, the structure will feel familiar, though the data returned may differ in structure and semantics.

***

## Getting started

### 1. Choose a package manager (npm or yarn)

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

### 2. Set up your project

Open your terminal and run the following commands:

<CodeGroup>
  ```text npm
  mkdir bitcoin-api-quickstart
  cd bitcoin-api-quickstart
  npm init --yes
  ```

  ```text yarn
  mkdir bitcoin-api-quickstart
  cd bitcoin-api-quickstart
  yarn init --yes
  ```
</CodeGroup>

This creates a new directory named `bitcoin-api-quickstart` and initializes a Node.js project within it.

### 3. Make your first request

Install Axios, a popular HTTP client, to make API requests:

<CodeGroup>
  ```bash bash
  npm install axios
  # Or with yarn
  # yarn add axios
  ```
</CodeGroup>

Create an `index.js` file in your project directory and paste the following code:

<CodeGroup>
  ```javascript javascript
  const axios = require('axios');

  const url = `https://bitcoin-mainnet.alchemy-blast.com/v2/${yourAPIKey}`;

  const payload = {
  jsonrpc: '2.0',
  id: 1,
  method: 'getblockcount',
  params: []
  };

  axios.post(url, payload)
  .then(response => {
  console.log('Current block height:', response.data.result);
  })
  .catch(error => {
  console.error('Error fetching block count:', error);
  });

  ```
</CodeGroup>

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

### 4. Run your script

To execute your script and make a request to the Bitcoin App, run:

<CodeGroup>
  ```bash bash
  node index.js
  ```
</CodeGroup>

You should see the current block number on Bitcoin App (in hexadecimal format) outputted to your console:

```shell
Current block height: 0x6d68e
```

## Next steps

You've made your first request to the Bitcoin App API. With this foundation, you can dive deeper into the [JSON-RPC methods available on Bitcoin App](/docs/bitcoin/bitcoin-api-overview) and start building your dApps on it!