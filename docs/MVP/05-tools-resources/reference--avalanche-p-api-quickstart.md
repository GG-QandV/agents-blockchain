> ⚠️ **This page is a template variant.** The consolidated content is in [api-quickstart](../00-consolidated/api-quickstart.md).
> Below is the original chain-specific version.

# Avalanche P-Chain API Quickstart

> Source: [https://www.alchemy.com/docs/reference/avalanche-p-api-quickstart.md](https://www.alchemy.com/docs/reference/avalanche-p-api-quickstart.md)

# Avalanche P-Chain API Quickstart

> How to get started with the Avalanche Platform Chain (P-Chain) using Alchemy

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

> 📄 **This content also appears in [Choose Your Starting Point](05-tools-resources/get-started.md)** — see there for full details.

The Avalanche Platform Chain (P-Chain) is the metadata chain on Avalanche that coordinates validators, manages staking, and handles creation of subnets and blockchains. Unlike the Avalanche C-Chain (EVM), the Avalanche P-Chain uses the `platform.*` JSON-RPC API for validator set management, delegations, and subnet operations.

**The Avalanche P-Chain is now live on both mainnet and Fuji.** Use the same Alchemy base URLs as Avalanche C-Chain and append `/ext/bc/P` after your API key in the path.

## Getting started

### 1. Choose a package manager (npm or yarn)

> 📄 **This content also appears in [Aptos API Quickstart](05-tools-resources/reference--aptos-api-quickstart.md)** — see there for full details.

| npm | yarn |
| --- | ---- |
| Follow the [npm documentation](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). | Refer to [yarn's installation guide](https://classic.yarnpkg.com/lang/en/docs/install). |

### 2. Set up your project

Open your terminal and run:

<CodeGroup>
  ```text npm
  mkdir avalanche-p-api-quickstart
  cd avalanche-p-api-quickstart
  npm init --yes
  ```

  ```text yarn
  mkdir avalanche-p-api-quickstart
  cd avalanche-p-api-quickstart
  yarn init -y
  ```
</CodeGroup>

This creates a new directory and initializes a Node.js project.

### 3. Make your first request

Create an `index.js` file in your project directory. The following example uses native `fetch` (Node.js 18+), so no extra dependencies are needed. Replace `YOUR_ALCHEMY_API_KEY` with your Alchemy API key.

<CodeGroup>
  ```javascript index.js
  (async () => {
    const API_KEY = 'YOUR_ALCHEMY_API_KEY';
    const url = `https://avax-mainnet.g.alchemy.com/v2/${API_KEY}/ext/bc/P`;

    try {
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'platform.getHeight',
          params: {}
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${await response.text()}`);
      }

      const data = await response.json();
      if (data.error) {
        throw new Error(data.error.message || 'RPC error');
      }
      console.log('P-Chain height:', data.result?.height ?? data);
    } catch (error) {
      console.error('Error fetching P-Chain height:', error.message);
    }
  })();
  ```
</CodeGroup>

Run the script:

<CodeGroup>
  ```bash bash
  node index.js
  ```
</CodeGroup>

You should see the current P-Chain height in the console (e.g. `P-Chain height: 35000000`).

## Send your first request on Alchemy (curl)

You can also call the API with `curl` from the terminal. Example for **Mainnet** (`platform.getHeight`):

<CodeGroup>
```bash
curl -X POST https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getHeight","params":{}}'
```
</CodeGroup>

Example for **Fuji** (`platform.getTimestamp`):

<CodeGroup>
```bash
curl -X POST https://avax-fuji.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getTimestamp","params":{}}'
```
</CodeGroup>

## Get current validators

<CodeGroup>
```bash
curl -X POST https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getCurrentValidators","params":{}}'
```
</CodeGroup>

To filter by subnet or specific node IDs, pass optional params:

<CodeGroup>
```bash
curl -X POST https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getCurrentValidators","params":{"subnetID":"11111111111111111111111111111111LpoYY"}}'
```
</CodeGroup>

## Get min stake

<CodeGroup>
```bash
curl -X POST https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getMinStake","params":{"subnetID":"11111111111111111111111111111111LpoYY"}}'
```
</CodeGroup>

## Get subnets

<CodeGroup>
```bash
curl -X POST https://avax-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY/ext/bc/P \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"platform.getSubnets","params":{}}'
```
</CodeGroup>