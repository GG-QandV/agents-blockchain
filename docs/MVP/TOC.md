# Alchemy Documentation — Table of Contents

**426 files** · auto-generated

---

## 📦 Consolidated Templates

### Api Faq
`📄 00-consolidated/api-faq.md`

- Common Template
- Per-Chain Parameters
- Notes

### Api Overview
`📄 00-consolidated/api-overview.md`

- Common Template
- Per-Chain Parameters
- Notes

### Api Quickstart
`📄 00-consolidated/api-quickstart.md`

- Common Template
- Per-Chain Parameters
- Notes

### Utxo Overview
`📄 00-consolidated/utxo-overview.md`

- Common Template
- UTXO-based model
- What is a UTXO?
- Properties of UTXOs
- Account model vs UTXO model
- Querying UTXO data with Alchemy
- Per-Chain Parameters
- Notes

### Utxo Websockets
`📄 00-consolidated/utxo-websockets.md`

- Common Template
- subscribeNewBlock
  - Parameters
  - Response
  - Request
- subscribeNewTransaction
  - Parameters
  - Response
  - Request
  - Result
- subscribeFiatRates
  - Parameters
  - Response
  - Request
  - Result

---

## ⛓️ Chains

### Activity Log Webhook Subscriptions
`📄 01-chains/activity-log-webhook-subscriptions.md`

- How it works
- Create a subscription
- Authentication
- Payload
- Filtering events
- Testing your endpoint
- Managing subscriptions
- Delivery & reliability
- Next steps

### Add Alchemy RPC To Any Project using Cursor
`📄 01-chains/add-alchemy-rpc-to-any-project.md`

- Prerequisites
- One shot Cursor prompt
- How to call the proxy
- Quick verification checklist

### Arbitrum API Overview
`📄 01-chains/arbitrum--arbitrum-api-overview.md`

- Arbitrum APIs
- Related APIs

### Base API Overview
`📄 01-chains/base--base-api-overview.md`

- Base APIs
- Related APIs

### Berachain API Overview
`📄 01-chains/berachain--berachain-api-overview.md`

- Berachain APIs
- Related APIs

### Blockchain 101
`📄 01-chains/blockchain-101.md`

- 1. Bitcoin
- 2. Ethereum
- 3. Why is blockchain important?
- 4. Ethereum block details
- 5. Notable projects in the space

### Blockchain Basics
`📄 01-chains/blockchain-basics.md`

- Learn more about blockchains

### BNB Smart Chain API Overview
`📄 01-chains/bnb-smart-chain--bnb-smart-chain-api-overview.md`

- BNB Smart Chain APIs
- Related APIs

### debug_getRawBlock
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-get-raw-block.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_getRawHeader
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-get-raw-header.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_getRawReceipts
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-get-raw-receipts.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_traceBlockByHash
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-trace-block-by-hash.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_traceBlockByNumber
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-trace-block-by-number.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_traceCallMany
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-trace-call-many.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_traceCall
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-trace-call.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### debug_traceTransaction
`📄 01-chains/chains--debug-api--debug-api-endpoints--debug-trace-transaction.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### API Overview
`📄 01-chains/chains.md`

- What are the Chain APIs?
- Companion APIs
- Key capabilities
- Supported blockchains
  - Ethereum & EVM layer 1s
  - Layer 2 scaling solutions
    - Optimistic rollups
    - ZK rollups
    - Other L2s & sidechains
  - Alternative layer 1 blockchains
  - Emerging networks & testnets
- Getting started

### Set up Frontend for Solana Application
`📄 01-chains/hello-world-solana-application.md`

- Step 1: Get your Solana program ID
- Step 2: Create a Next.js app
- Step 3: Install Solana web3.js and set environment variables
- Step 4: Build a minimal client UI that pings your program
- Step 5: Run the Next.js app
- 🎉 Success

### Build & Deploy a "Hello World" Solana Program
`📄 01-chains/hello-world-solana-program.md`

- Overview
- Prerequisites
  - ✔ Rust & Cargo
  - ✔ Solana CLI
  - ✔ Node.js + Yarn or PNPM
  - ✔ Alchemy Solana RPC URL
- Step 1: Create a new Solana program
- Step 2: Add Solana dependencies
- Step 3: Write the Hello World program
- Step 4: Build the program
- Step 5: Create a Solana keypair
- Step 6: Set your network to devnet
- Step 7: Airdrop SOL
- Step 8: Deploy your program
- Step 9: Invoke program via Alchemy RPC

### How do Ethereum transactions work?
`📄 01-chains/how-ethereum-transactions-work.md`

- Intro
- Ethereum = A Transaction-Based State Machine
- What is a Transaction? (Ethereum)
  - Block & Transactions
  - Chain of States
- Chain of Blocks; A Blockchain!
  - Stack of Transactions
- Refresher on the Ethereum World State
    - Several Views of Ethereum World State
  - Wait, So An Account Can Be A Smart Contract?
- How Are The Account Public Addresses Determined?
  - Ok, Back to Transactions
    - Trivia: Can smart contract accounts initiate a transaction?
- Two Types of Transactions in Ethereum
  - 1. **Contract creation**: a special type of transaction that deploys a brand new smart contract
  - 2. Message call: a transaction initiated by an EOA that interacts with either another EOA or a smart contract
- Ethereum Transaction Architecture
- Blockchain = Globally Shared Transaction Database
- P2P Network
  - Transaction Object Example
  - How To Manually Construct Calldata
  - Conclusion
- Learn More About Ethereum Development

### How to Calculate Ethereum Miner Rewards
`📄 01-chains/how-to-calculate-ethereum-miner-rewards.md`

- Install environment tools
  - Ethers
  - Axios
  - Dotenv
- Create a Dotenv File
- Cost of all transactions in a block
- Sum the burned fees in a block
- Uncle and nephew rewards
  - nephew rewards
  - Uncle rewards
- Final miner reward calculation

### How to Enable Compression to Speed Up JSON-RPC Blockchain Requests
`📄 01-chains/how-to-enable-compression-to-speed-up-json-rpc-blockchain-requests.md`

  - What causes slow response times?
    - 1. Large Requests or Responses
    - 2. Complex Requests
- Compress RPC responses for faster requests
  - Compression support by chain
- How to Enable Gzip Compression on Node Requests
  - Step 1: Set up an Alchemy Account
  - Step 2: Create your app and API key
  - Step 3: Make a command-line node request with gzip enabled
- Test latency with gzip compression
  - Step 1: Create a curl-format file
  - Step 2: Test a request without gzip
  - Step 3: Test a request with gzip

### Get a wallet's cross-chain balance and display it
`📄 01-chains/how-to-get-crosschain-token-balances.md`

- Prerequisites
- Next steps

### How to Get On-chain Events on Ethereum
`📄 01-chains/how-to-get-on-chain-events.md`

- Step 1: Install Node and NPM
- Step 2: Create an Alchemy app
- Step 3: Create a node project
- Step 4: Get the event logs

### How to Get the Latest Block on Ethereum
`📄 01-chains/how-to-get-the-latest-block-on-ethereum.md`

- 1. Create a project directory
- 2. Install a web3 library
- 3. Create `index.js`
- 4. Run it using node

### How to Get Transaction History for an Address on Ethereum
`📄 01-chains/how-to-get-transaction-history-for-an-address-on-ethereum.md`

- How to query transaction history
  - Example: Getting Transactions Originating `From` An Address
    - Fetch
    - Node-Fetch
    - Axios
  - Example: Getting Recipient-based Transactions
    - JavaScript with Fetch (Recommended)
    - Node-Fetch
    - Axios
- How to process the API response
  - Modern Web3 Libraries (Recommended)
    - Parsing with Modern Web3 Library Responses
  - Node-Fetch
    - Parsing with `Node-Fetch` Responses
  - Axios
    - Parsing with `Axios` Responses
  - Raw API Response
    - Understanding API Response
- Printing out the `asset` and `value`
  - Modern Web3 Libraries (Recommended)
    - Saving response objects with Modern Web3 Libraries
  - Node-Fetch
    - Saving response objects with `Node-Fetch`
  - Axios
    - Saving response objects with `Axios`

### How to Send Transactions on Ethereum
`📄 01-chains/how-to-send-transactions-on-ethereum.md`

- The Basics
  - 1. Alchemy does not store your private keys
  - 2. What is a "signer"?
  - 3. Why do I need to sign my transactions?
  - 4. How do I protect my private key?
  - 5. What is the web3 library?
- Steps to Sending Your Transaction
  - 1. Create an Alchemy app on the Sepolia testnet
  - 2. Request Eth from the [Alchemy Sepolia faucet](https://www.alchemy.com/faucets/ethereum-sepolia)
  - 3. Create a new project directory and `cd` into it
  - 4. Install Viem/Ethers.js and dotenv
  - 5. Create the .env file
  - 6. Create `sendTx.js` file
  - 7. Run the code using `node sendTx.js`
  - 8. See your transaction in the Mempool
  - Transaction sent successfully

### How to simulate a transaction on Ethereum
`📄 01-chains/how-to-simulate-a-transaction-on-ethereum.md`

- Introduction
- Table of Contents
- Understanding the Transaction Object
- Simulating a Transaction with Alchemy
- Conclusion

### How to Verify a Message Signature on Ethereum
`📄 01-chains/how-to-verify-a-message-signature-on-ethereum.md`

- Prerequisites
  - Install Node.js
  - Install MetaMask
  - Install an IDE
  - Connect to Alchemy
- Setup Project Environment
- Install environment tools
  - Create a Dotenv File
- Verify Message Signatures

### Internal Playbook: Upgrading Ethereum Nodes
`📄 01-chains/internal-playbook-upgrading-ethereum-nodes.md`

- Why Upgrade Nodes?
- When To Upgrade Nodes?
- How To Upgrade Nodes?
- Calling All Alchemists 🧙

### How are Merkle trees used in blockchains?
`📄 01-chains/merkle-trees-in-blockchains.md`

  - Merkle Trees In Bitcoin
  - Merkle Proofs
  - Merkle Trees Use Cases
  - Logarithmic Scaling
  - Merkle Tree Vocabulary Summary
- Conclusion
- Learn More About Blockchain Data Structures

### On-chain Events
`📄 01-chains/on-chain-events.md`


### Polygon PoS API Overview
`📄 01-chains/polygon-pos--polygon-pos-api-overview.md`

- Polygon PoS APIs
- Related APIs

### Polygon zkEVM API Overview
`📄 01-chains/polygon-zkevm--polygon-zkevm-api-overview.md`

- Polygon zkEVM APIs
- Related APIs

### accountSubscribe
`📄 01-chains/reference--account-subscribe.md`


### getAssetProof
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-asset-proof.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetProofs
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-asset-proofs.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetSignatures
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-asset-signatures.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAsset
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-asset.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetsByAuthority
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-assets-by-authority.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetsByCreator
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-assets-by-creator.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetsByGroup
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-assets-by-group.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssetsByOwner
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-assets-by-owner.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getAssets
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-assets.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getNftEditions
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-nft-editions.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### getTokenAccounts
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--get-token-accounts.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### searchAssets
`📄 01-chains/reference--alchemy-das-apis-for-solana--solana-das-api-endpoints--search-assets.md`

- Parameters
- Result
- Example
  - Request
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### Alchemy DAS APIs for Solana NFTs and Fungible Tokens (Beta)
`📄 01-chains/reference--alchemy-das-apis-for-solana.md`

- Background
- Methods
- Docs & Sandbox
  - How to view parameters
  - How to test requests

### alchemy_minedTransactions
`📄 01-chains/reference--alchemy-minedtransactions.md`

  - Request
  - Result

### alchemy_pendingTransactions
`📄 01-chains/reference--alchemy-pendingtransactions.md`

  - Request
  - Result

### ApeChain API FAQ
`📄 01-chains/reference--apechain-api-faq.md`

- What is ApeChain?
- What is the ApeChain API?
- How can I get started using the ApeChain API?
- Is ApeChain EVM compatible?
- What API does ApeChain use?
- What is an ApeChain API key?
- Which libraries support ApeChain?
- What programming languages work with ApeChain?
- What does ApeChain use for gas?
- How can I participate in the ApeChain ecosystem?
- Is ApeChain secure?
- My question isn't here, where can I get help?

### ApeChain API Quickstart
`📄 01-chains/reference--apechain-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Arbitrum API FAQ
`📄 01-chains/reference--arbitrum-api-faq.md`

- How can I get started using the Arbitrum API?
- What type of Layer 2 solution is Arbitrum?
- Is Arbitrum EVM compatible?
- How do I add Arbitrum to MetaMask mainnet?
- What testnet should I use for Arbitrum?
- How do I build an app on Arbitrum?
- How do you bridge Arbitrum to Ethereum?
- What wallets can be used on Arbitrum?
- What does Arbitrum use for gas?
- What projects are on Arbitrum?
- How do you withdraw ETH from Arbitrum?
- What API does Arbitrum use?
- What is an Arbitrum API key?
- Which libraries support Arbitrum?
- What programming languages work with Arbitrum?
- What methods does Alchemy support for the Arbitrum API?
- My question isn't here, where can I get help?

### Arbitrum API Quickstart
`📄 01-chains/reference--arbitrum-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Arbitrum Nova Network Deprecation Notice
`📄 01-chains/reference--arbitrum-nova-deprecation-notice.md`

- ⚠️ Deprecation Notice

### Arbitrum vs. Ethereum API Differences
`📄 01-chains/reference--arbitrumethereum-differences.md`

- Blocks and Time
- Block hashes and randomness
- L1 Fees
- Blocks
  - Additional Fields
  - Existing Fields With Different Behavior
- Transactions
  - Transaction Types
  - Additional Fields
  - Existing Fields With Different Behavior
- Tx Receipts
  - Additional Fields

### Base API FAQ
`📄 01-chains/reference--base-api-faq.md`

- What is Base?
- What is the Base API?
- How can I get started using the Base API?
- Is Base EVM compatible?
- What API does Base use?
- What is a Base API key?
- Which libraries support Base?
- What programming languages work with Base?
- What does Base use for gas?
- What testnet should I use for Base?
- What methods does Alchemy support for the Base API?
- How can I bridge assets between Ethereum and Base?
- My question isn't here, where can I get help?

### Base API Quickstart
`📄 01-chains/reference--base-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Base Flashblocks API quickstart
`📄 01-chains/reference--base-flashblocks-api-quickstart.md`

- What are Flashblocks
- Supported networks
- Query Flashblocks with JSON-RPC
  - [eth\_getBlockByNumber](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-block-by-number)
    - Example Response
  - [eth\_getTransactionReceipt](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-receipt)
    - Example Response
  - [eth\_getBalance](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-balance)
    - Example Response
  - [eth\_getTransactionCount](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-count)
    - Example Response
  - [eth\_getTransactionByHash](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-by-hash)
    - Example Response
  - [eth\_call](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-call)
    - Example Response
  - [eth\_simulateV1](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-simulate-v-1)
    - Example Response
  - [eth\_estimateGas](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-estimate-gas)
    - Example Response
  - [eth\_getLogs](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-get-logs)
    - Example Response
  - `eth_subscribe` (WebSocket subscriptions)
    - Flashblocks subscription types
    - Subscription response
    - `eth_unsubscribe`

### Berachain API FAQ
`📄 01-chains/reference--berachain-api-faq.md`

- What is Berachain?
- What is the Berachain API?
- How can I get started using the Berachain API?
- Is Berachain EVM compatible?
- What API does Berachain use?
- What is a Berachain API key?
- Which libraries support Berachain?
- What programming languages work with Berachain?
- What does Berachain use for gas?
- What methods does Alchemy support for the Berachain API?
- My question isn't here, where can I get help?

### Berachain API Quickstart
`📄 01-chains/reference--berachain-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### BNB Smart Chain Quickstart
`📄 01-chains/reference--bnb-smart-chain-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### BNB Smart Chain FAQ
`📄 01-chains/reference--bnb-smart-chain-faq.md`

- What is BNB Smart Chain (BSC)?
- What is the BNB Smart Chain API?
- How can I get started using the BSC API?
- Is BSC EVM compatible?
- What API does BSC use?
- What is a BSC API key?
- Which libraries support BSC?
- What programming languages work with BSC?
- What does BSC use for gas?
- What methods does Alchemy support for the BSC API?
- My question isn't here, where can I get help?

### Debug API Quickstart
`📄 01-chains/reference--debug-api-quickstart.md`

- Introduction
- Debug API use cases
- Trace types
- Trace actions types
  - `CREATE`
    - Example response
  - `SUICIDE`
    - Example response
  - `CALL`
    - Example response
- Helpful Resources

### Ethereum API FAQ
`📄 01-chains/reference--ethereum-api-faq.md`

- What testnet should you use for Ethereum development?
- What API does Ethereum use?
- What is an Ethereum API key?
- Does Ethereum only use JSON-RPC?
- How does Alchemy's Ethereum API work?
- Can you use Python for Ethereum?
- How do I get the timestamp for a transaction?
- Contract vs wallet address
- What is the difference between `DATA` and `QUANTITY`?
- What is the default block parameter?
- What methods does Alchemy support for the Ethereum API?
- My question isn't here, where can I get help?

### Ethereum API Quickstart
`📄 01-chains/reference--ethereum-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Feature Support By Chain
`📄 01-chains/reference--feature-support-by-chain.md`


### logsSubscribe
`📄 01-chains/reference--logs-subscribe.md`


### MEV Protection
`📄 01-chains/reference--mev-protection.md`

- Why it matters
- How MEV protection works
- Supported networks
- Key benefits

### Monad API FAQ
`📄 01-chains/reference--monad-api-faq.md`

- What is Monad Chain?
- What is the Monad Chain API?
- How can I get started using the Monad Chain API?
- Is Monad Chain EVM compatible?
- What API does Monad Chain use?
- What is a Monad Chain API key?
- Which libraries support Monad Chain?
- What programming languages work with Monad Chain?
- What does Monad Chain use for gas?
- What methods does Alchemy support for the Monad Chain API?
- My question isn't here, where can I get help?

### Monad API Quickstart
`📄 01-chains/reference--monad-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### monadLogs
`📄 01-chains/reference--monadlogs.md`


### monadNewHeads
`📄 01-chains/reference--monadnewheads.md`


### newHeads
`📄 01-chains/reference--newheads.md`


### newPendingTransactions
`📄 01-chains/reference--newpendingtransactions.md`


### Operating your Rollup
`📄 01-chains/reference--operating-your-rollup.md`


### Polygon PoS API FAQ
`📄 01-chains/reference--polygon-pos-api-faq.md`

- How can I get started using the Polygon API?
- Is Polygon EVM compatible?
- What API does Polygon use?
- What is a Polygon API key?
- What programming languages work with Polygon?
- How do I add Polygon to MetaMask mainnet?
- What testnet should I use for Polygon?
- How do I build an app on Polygon?
- What wallets can be used on Polygon?
- What does Polygon use for gas?
- How do you bridge Polygon to Ethereum?
- How do you withdraw ETH from Polygon?
- What projects are on Polygon?
- What methods does Alchemy support for the Polygon API?
- My question isn't here, where can I get help?

### Polygon PoS API Quickstart
`📄 01-chains/reference--polygon-pos-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### What is the difference between Polygon zkEVM and Ethereum?
`📄 01-chains/reference--polygon-zkevm-and-ethereum-differences.md`

- JSON RPC Method Differences
- Additional `zkEVM_*` methods
- Opcodes
- Additions
- Other Minor Differences

### Polygon zkEVM API FAQ
`📄 01-chains/reference--polygon-zkevm-api-faq.md`

- How can I get started using the Polygon zkEVM API?
- Is Polygon zkEVM EVM compatible?
- What API does Polygon zkEVM use?
- What is a Polygon zkEVM API key?
- Which libraries support Polygon zkEVM?
- What programming languages work with Polygon zkEVM?
- What does Polygon zkEVM use for gas?
- What testnet should I use for Polygon zkEVM?
- Supported Polygon zkEVM methods
- My question isn't here, where can I get help?

### Polygon zkEVM API Quickstart
`📄 01-chains/reference--polygon-zkevm-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Polygon zkEVM Deprecation Notice
`📄 01-chains/reference--polygon-zkevm-deprecation-notice.md`

- ⚠️ Deprecation Notice
- What you need to do
- Need help?

### programSubscribe
`📄 01-chains/reference--program-subscribe.md`


### Robinhood Chain API FAQ
`📄 01-chains/reference--robinhood-chain-api-faq.md`

- What is Robinhood Chain?
- How do I get started with Robinhood Chain?
- What is the Robinhood Chain API?
- Is Robinhood Chain EVM compatible?
- What API does Robinhood Chain use?
- What methods are supported on Robinhood Chain?
- What is a Robinhood Chain API key?
- Which libraries support Robinhood Chain?

### Robinhood Chain API Quickstart
`📄 01-chains/reference--robinhood-chain-api-quickstart.md`

- What is Robinhood Chain?
- What is the Robinhood Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Rollups FAQ
`📄 01-chains/reference--rollups-faq.md`


### rootSubscribe
`📄 01-chains/reference--root-subscribe.md`


### signatureSubscribe
`📄 01-chains/reference--signature-subscribe.md`


### slotSubscribe
`📄 01-chains/reference--slot-subscribe.md`


### Solana API Quickstart
`📄 01-chains/reference--solana-api-quickstart.md`

- Send your first request on Alchemy
- Create a connection to Alchemy
- Create a wallet
- Check wallet balance
- Fetch account info
- Find Solana methods by category

### Solana Subscription API Endpoints
`📄 01-chains/reference--solana-subscription-api-endpoints.md`

- Accounts
- Transactions
- Cluster

---

## 📊 Data

### Transactions By Wallet (Beta)
`📄 02-data/data--beta-apis--beta-api-endpoints--beta-api-endpoints--get-transaction-history-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFT Collections By Wallet
`📄 02-data/data--portfolio-apis--portfolio-api-endpoints--portfolio-api-endpoints--get-nft-contracts-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFTs By Wallet
`📄 02-data/data--portfolio-apis--portfolio-api-endpoints--portfolio-api-endpoints--get-nfts-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Token Balances By Wallet
`📄 02-data/data--portfolio-apis--portfolio-api-endpoints--portfolio-api-endpoints--get-token-balances-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Tokens By Wallet
`📄 02-data/data--portfolio-apis--portfolio-api-endpoints--portfolio-api-endpoints--get-tokens-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Historical Token Prices
`📄 02-data/data--prices-api--prices-api-endpoints--prices-api-endpoints--get-historical-token-prices.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Token Prices By Address
`📄 02-data/data--prices-api--prices-api-endpoints--prices-api-endpoints--get-token-prices-by-address.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Token Prices By Symbol
`📄 02-data/data--prices-api--prices-api-endpoints--prices-api-endpoints--get-token-prices-by-symbol.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### alchemy_simulateAssetChangesBundle
`📄 02-data/data--simulation-apis--transaction-simulation-endpoints--alchemy-simulate-asset-changes-bundle.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_simulateAssetChanges
`📄 02-data/data--simulation-apis--transaction-simulation-endpoints--alchemy-simulate-asset-changes.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_simulateExecutionBundle
`📄 02-data/data--simulation-apis--transaction-simulation-endpoints--alchemy-simulate-execution-bundle.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_simulateExecution
`📄 02-data/data--simulation-apis--transaction-simulation-endpoints--alchemy-simulate-execution.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### Get Stellar account balances
`📄 02-data/data--stellar-data-api--stellar-data-api-endpoints--balances--get-stellar-balances.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get Stellar NFT holdings
`📄 02-data/data--stellar-data-api--stellar-data-api-endpoints--nf-ts--get-stellar-nfts.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get Stellar transfer history
`📄 02-data/data--stellar-data-api--stellar-data-api-endpoints--transfers--get-stellar-transfers.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### alchemy_getTokenAllowance
`📄 02-data/data--token-api--token-api-endpoints--alchemy-get-token-allowance.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_getTokenBalances
`📄 02-data/data--token-api--token-api-endpoints--alchemy-get-token-balances.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_getTokenMetadata
`📄 02-data/data--token-api--token-api-endpoints--alchemy-get-token-metadata.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_getAssetTransfers
`📄 02-data/data--transfers-api--transfers-endpoints--alchemy-get-asset-transfers.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### alchemy_getTransactionReceipts
`📄 02-data/data--utility-apis--transactions-receipts-endpoints--alchemy-get-transaction-receipts.md`

- Parameters
- Result
- Example
  - Request
  - Response
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- OpenRPC Method Specification

### Create a Variable
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--create-custom-webhook-variable.md`

- Headers
- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Create webhook
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--create-webhook.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Delete a Variable
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--delete-custom-webhook-variable.md`

- Headers
- Path Parameters
- Code Examples
  - cURL
  - JavaScript

### Delete webhook
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--delete-webhook.md`

- Headers
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get Variable Elements
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--read-custom-webhook-variable.md`

- Headers
- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript

### Replace webhook addresses
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--replace-webhook-addresses.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get all webhooks
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--team-webhooks.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Update a Variable
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--update-custom-webhook-variable.md`

- Headers
- Path Parameters
- Code Examples
  - cURL
  - JavaScript

### Add and remove webhook addresses
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--update-webhook-addresses.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Update webhook NFT filters
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--update-webhook-nft-filters.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Update webhook
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--update-webhook.md`

- Headers
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get all addresses for an Address Activity webhook
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--webhook-addresses.md`

- Headers
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Get all webhook NFT filters
`📄 02-data/data--webhooks--webhooks-api-endpoints--notify-api-endpoints--webhook-nft-filters.md`

- Headers
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### DATA Network API Overview
`📄 02-data/data-network--data-network-api-overview.md`

- DATA Network APIs
- Related APIs

### Data APIs Overview
`📄 02-data/data.md`

- Components of the Data APIs
- When should you use the Data APIs?
- Data APIs use cases
  - 1. Developer tooling and debugging
  - 2. Help your users *write* transactions
  - 3. Help your users *read* transactions
- Summary
- Quick reference
  - Token, NFT, and Prices APIs
  - Webhooks

### Address Activity Webhook
`📄 02-data/reference--address-activity-webhook.md`


### Custom Webhook Filters
`📄 02-data/reference--custom-webhook-filters.md`

- Event/Log Filters
  - Specifying Addresses within Custom Webhook Event Filters
  - Specifying Topics within Custom Webhook Event Filters
- External Transaction Filters
  - Specifying `from` addresses within Custom Webhook Transaction Filters
  - Specifying `to` within Custom Webhook Transaction Filters
  - Specifying `from` & `to` within Custom Webhook Transaction Filters
- Internal Transaction (Debug trace calls) Filters (BETA)
  - Specifying `from` addresses within Custom Webhook Internal Transaction Filters
  - Specifying `to` addresses within Custom Webhook Internal Transaction Filters
  - Specifying `from` & `to` within Custom Webhook Internal Transaction Filters

### Custom Webhook Variables
`📄 02-data/reference--custom-webhook-variables.md`

  - Example `Address` Typed Variables
  - Example `Log Topic` Typed Variables
- How to Use Variables in Custom Webhooks
  - Single Variable Custom Webhooks
  - Multi-Variable Custom Webhooks
  - How many elements can I use within a Custom Webhook variable?

### Custom Webhook
`📄 02-data/reference--custom-webhook.md`


### Custom Webhooks GraphQL Examples
`📄 02-data/reference--custom-webhooks-example.md`

- Ethereum Custom Webhook GraphQL Templates:
- Polygon PoS Custom Webhook GraphQL Templates:
- Arbitrum Custom Webhook GraphQL Templates:
- Optimism Custom Webhook GraphQL Templates:

### NFT Activity Webhook
`📄 02-data/reference--nft-activity-webhook.md`


### computeRarity
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--compute-rarity.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getContractMetadataBatch
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contract-metadata-batch.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getContractMetadata
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contract-metadata.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getContractsForOwner
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contracts-for-owner.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getFloorPrice
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-floor-price.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getNFTsForCollection
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-nf-ts-for-collection.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getNFTs
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-nf-ts.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getNFTMetadataBatch
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-nft-metadata-batch.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getNFTMetadata
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-nft-metadata.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getNFTSales
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-nft-sales.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getOwnersForCollection
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-owners-for-collection.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getOwnersForToken
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-owners-for-token.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### getSpamContracts
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-spam-contracts.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### invalidateContract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--invalidate-contract.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### isAirdrop
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--is-airdrop.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### isHolderOfCollection
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--is-holder-of-collection.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### isSpamContract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--is-spam-contract.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### reportSpam
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--report-spam.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### searchContractMetadata
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--search-contract-metadata.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### summarizeNFTAttributes
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--summarize-nft-attributes.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Attribute Rarity By NFT
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--compute-rarity-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Collection Metadata By Slug
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--get-collection-metadata-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Contract Metadata By Address
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--get-contract-metadata-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFTs By Collection
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--get-nf-ts-for-collection-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFTs By Contract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--get-nf-ts-for-contract-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFT Metadata By Token ID
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--get-nft-metadata-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Invalidate Contract Cache
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--invalidate-contract-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Refresh NFT Metadata
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--refresh-nft-metadata-v-3.md`

- Path Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Search Contract Metadata
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--search-contract-metadata-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Attributes Summary By Contract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-metadata-endpoints--summarize-nft-attributes-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Collections By Owner
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--get-collections-for-owner-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Contracts By Owner
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--get-contracts-for-owner-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFTs By Owner
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--get-nf-ts-for-owner-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Owners By Contract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--get-owners-for-contract-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Owners By NFT
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--get-owners-for-nft-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Is Holder Of Contract
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-ownership-endpoints--is-holder-of-contract-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### Floor Prices By Slug
`📄 02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-sales-endpoints--get-floor-price-v-3.md`

- Path Parameters
- Query Parameters
- Code Examples
  - cURL
  - JavaScript
  - Python
  - Go
  - Java
  - C#
- Operation Specification

### NFT API Endpoints Overview
`📄 02-data/reference--nft-api-endpoints.md`


### Portfolio APIs
`📄 02-data/reference--portfolio-apis.md`

- **Why Portfolio APIs?**
- Feedback?

### Prices API FAQ
`📄 02-data/reference--prices-api-faq.md`

  - How are prices calculated?
  - What tokens do you support?
  - How are new tokens added?
  - Are there any rate limits?
  - Something else

### Prices API Quickstart
`📄 02-data/reference--prices-api-quickstart.md`

- Via HTTP requests
  - No additional installation required
  - Usage
  - Running the script
- Via Node Fetch
  - Installation
  - Usage
  - Running the script

### Asset Changes
`📄 02-data/reference--simulation-asset-changes.md`

- How does it work?
- What do you get back?
- Examples
- Ethereum
  - ETH - Transfer
  - ERC20 - Transfer
  - WETH - Wrap
  - WETH - Unwrap
- Polygon
  - MATIC - Transfer
  - ERC20 - Transfer
  - WMATIC - Wrap
  - WMATIC - Unwrap

### Bundle Simulation
`📄 02-data/reference--simulation-bundle.md`


### Transaction Simulation Examples
`📄 02-data/reference--simulation-examples.md`

- Ethereum
  - ETH - Transfer - `simulateAssetChanges`
  - ETH - Transfer - `simulateExecution`
  - ERC20 - Transfer
  - WETH - Wrap
  - WETH - Unwrap
- Polygon
  - MATIC - Transfer
  - ERC20 - Transfer
  - WMATIC - Wrap
  - WMATIC - Unwrap

### Execution Simulation
`📄 02-data/reference--simulation-execution.md`

- How does it work?

### Transaction Simulation FAQs
`📄 02-data/reference--simulation-faqs.md`

- What chains do you support?
- Wrapped tokens
- Do you support NFTs that predate ERC721?
- How can I get metadata for NFTs (ERC721, ERC155)?
- How can I determine if a simulation was successful?
- How can I get gas used?
  - `alchemy_simulateAssetChanges`
  - `alchemy_simulateExecution`
- Simulation Limits and Pricing

### Transaction Simulation
`📄 02-data/reference--simulation.md`


---

## 👛 Wallets

### Agent Wallets
`📄 03-wallets/agent-wallets.md`

- Capabilities
- Connect a session
- Examples
- Machine-readable contract
- Operational details
- Next steps

### How to Get All the Contracts Deployed by a Wallet
`📄 03-wallets/how-to-get-all-the-contracts-deployed-by-a-wallet.md`

- Install Node and npm
- Create an Alchemy App
- Create a node project
- Coding the Script
- Testing the Script

### How to Make Your Dapp Compatible With Smart Contract Wallets Using ERC-1271
`📄 03-wallets/how-to-make-your-dapp-compatible-with-smart-contract-wallets.md`

- Smart Contract Wallets
- ERC-4337
- Example script using `ethers.js`
- Libraries for SCW Signature Verification

### Bundler Sponsored Operations
`📄 03-wallets/reference--bundler-sponsored-operations.md`

- What is Bundler Sponsorship?
- Supported chains
- How it differs from paymaster sponsorship
- Policy setup
  - Required policy configuration
- Compute unit costs
- How to use bundler sponsorship
  - Example implementation
  - Key configuration points
- Requirements
- Benefits
- Limitations (beta)
- Related documentation

### Gas Limits
`📄 03-wallets/reference--gas-limits-for-eth_call-and-eth_estimategas.md`


---

## 🤖 Build with AI

### Alchemy Agent Skills
`📄 04-build-with-ai/alchemy-agent-skills.md`

- Install the skills
- What's included
- Use the `alchemy-api` skill
- Use the `agentic-gateway` skill
- How skills work
- Next steps

### Alchemy CLI
`📄 04-build-with-ai/alchemy-cli.md`

- Install
- Authenticate
  - Headless and remote environments
  - Selecting an app
- Your first commands
- Use it from AI agents and scripts
- Command reference
  - EVM
    - Data
    - Simulation
    - Transactions
  - Wallets and signing
  - Solana
  - Cross-chain
  - Webhook (Notify)
  - App (Admin API)
  - Usage
  - Auth and config
  - Utilities
- Global flags
- Configuration options
  - Webhook API key (Notify)
  - Check what's configured
- Shell completions
- Next steps

### Agent Authentication and Payment
`📄 04-build-with-ai/alchemy-for-agents.md`

- How it works
- The flow
- What your agent can access
- CLI wallet roles
- Install as an agent skill
- Use it from the CLI
- Watch it in action
- Next steps

### Alchemy MCP Server
`📄 04-build-with-ai/alchemy-mcp-server.md`

- Connect your client
- Getting started
- Available tools
  - Admin — Account & App Management (8 tools)
  - RPC — Onchain JSON-RPC (132 tools)
  - Data — REST APIs (28 tools)
- Supported chains
- Related tools

### Asset Changes - Explained
`📄 04-build-with-ai/asset-changes-explained.md`


### Build with AI
`📄 04-build-with-ai/build-with-ai.md`

- The four tools
  - Agent Skills
  - Alchemy CLI
  - MCP Server
  - Agent authentication and payment
- Pick the right tool
- Supported AI tools

### Alchemy Data APIs Explained using Cursor
`📄 04-build-with-ai/data-apis-with-cursor.md`

- Components of the Data API
  - Focus of this guide
- Video walkthrough
- How to decide which Data API is right for you
- Build an app with Cursor
  - Project setup
- #1. Transfers API - What moved in/out?
  - Transfers API conclusion
    - What the UI shows about the Transfers API:
    - What this demonstrates about the Transfers API:
- #2. Token API - What does a wallet own right now?
  - Token API conclusion
    - What the UI shows about the Token API:
    - What this demonstrates about the Token API:
- #3. Prices API - How much are wallet tokens worth?
  - Prices API conclusion
    - What the UI shows about the Prices API:
    - What this demonstrates about the Prices API:
- #4 Portfolio API - Just give me everything!
  - Portfolio API conclusion
    - What the UI shows about the Portfolio API:
    - What this demonstrates about the Portfolio API:
- Portfolio V1 vs Portfolio V2
- Wrap-up

### Kaia API Overview
`📄 04-build-with-ai/kaia--kaia-api-overview.md`

- Kaia APIs
- Related APIs

### OP Mainnet API Overview
`📄 04-build-with-ai/op-mainnet--op-mainnet-api-overview.md`

- OP Mainnet APIs
- Related APIs

---

## 🛠️ Tools & Resources

### What is a 51% attack?
`📄 05-tools-resources/51-percent-attack.md`

- Learn More About 51% Attacks

### Abstract API Overview
`📄 05-tools-resources/abstract--abstract-api-overview.md`

- Abstract APIs
- Related APIs

### OCSF Mapping
`📄 05-tools-resources/activity-log-ocsf-mapping.md`

- How to use this mapping
  - OCSF classes used below
- Authentication → Authentication (3002)
- Credential & account lifecycle → Account Change (3001)
- Role changes → User Access Management (3005)
- Apps, keys & app configuration → Entity Management (3004)
- Network & security configuration → Entity Management (3004)
- Billing & account administration → Account Change (3001)
- Wallet activity → Authentication (3002) / API Activity (6003)
- Notes

### Activity Log
`📄 05-tools-resources/activity-log.md`

  - Why it matters
- Accessing the Activity Log
- What's in the log
  - What gets logged
- Exporting to CSV
- Retention

### ADI API Overview
`📄 05-tools-resources/adi--adi-api-overview.md`

- ADI APIs
- Related APIs

### Alchemy Claude Plugin
`📄 05-tools-resources/alchemy-claude-plugin.md`

- Install the plugin
- Slash commands
- What's included
- Plugin vs. MCP server vs. skills
- Privacy and support
- Next steps

### Alchemy Quickstart Guide
`📄 05-tools-resources/alchemy-quickstart-guide.md`

- 📋 Steps to get started with Alchemy
- 💻 Start Building!

### Request Logs
`📄 05-tools-resources/alchemy-request-logs.md`

  - Why it matters
  - How to use it
    - Request Logs feature overview
    - Code samples
  - Best practices
    - Pro tips
    - Limits and retention
    - Common issues and solutions
    - Additional notes

### Alchemy Sandbox
`📄 05-tools-resources/alchemy-sandbox.md`


### Alchemy Subgraphs Deprecation Notice
`📄 05-tools-resources/alchemy-subgraphs--deprecation-notice.md`

- Migrate to Goldsky
- Why Goldsky?

### Set Up Alchemy with any Library via AI
`📄 05-tools-resources/alchemy-via-libraries.md`

- Many coding languages, many great web3 libraries
- Quickstart with AI
- Next Steps

### Anime API Overview
`📄 05-tools-resources/anime--anime-api-overview.md`

- Anime APIs
- Related APIs

### Arc API Overview
`📄 05-tools-resources/arc--arc-api-overview.md`

- Arc APIs
- Related APIs

### Astar API Overview
`📄 05-tools-resources/astar--astar-api-overview.md`

- Astar APIs
- Related APIs

### Avalanche P-Chain API Overview
`📄 05-tools-resources/avalanche-p--avalanche-p-api-overview.md`

- Avalanche P-Chain APIs

### Best Practices for Deploying a Smart Contract on EVM Mainnets
`📄 05-tools-resources/best-practices-for-deploying-a-smart-contract-on-evm-mainnets-1.md`


### Best Practices for Key Security and Management
`📄 05-tools-resources/best-practices-for-key-security-and-management.md`

- API Key Authentication Methods
- Best Option Based on Situation
- Key Rotation
- JWT Expiration Times
- Using Proper Permissions

### Best Practices When Using Alchemy
`📄 05-tools-resources/best-practices-when-using-alchemy.md`

- 1. Send Requests Concurrently
- 2. Avoid High Batch Cardinality
- 3. Retry with exponential backoff on failures
- 4. Send Requests over HTTPS, not WebSockets
- 5. Avoid Large Request / Response Sizes
- 6. Use gZip Compression to Speed Up Large Requests
- 7. Contact Us When Multiplying Your Capacity
- 8. Protecting your API Keys
- 8. API Authentication

### Bitcoin API Overview
`📄 05-tools-resources/bitcoin--bitcoin-api-overview.md`

- Bitcoin APIs

### Migrating UTXO data to Alchemy
`📄 05-tools-resources/bitcoin--utxo-migration-guide.md`

- Supported networks
- What Alchemy's UTXO API provides
- How to enable UTXO endpoints
- Migrating from QuickNode
  - Example: Get balance history
  - Understand the differences
- Migrating from BlockCypher
  - Example: Get address balance
  - Understand the differences
- Migrating from Blockdaemon
  - Example: Get address balance
  - Understand the differences
- Benchmarking results
- Start your migration to Alchemy
- Frequently asked questions

### UTXO WebSockets
`📄 05-tools-resources/bitcoin--utxo-websockets.md`

- subscribeNewBlock
  - Parameters
  - Response
  - Request
  - Result
- subscribeNewTransaction
  - Parameters
  - Response
  - Request
  - Result
- subscribeAddresses
  - Parameters
  - Response
  - Request
  - Result
- subscribeFiatRates
  - Parameters
  - Response
  - Request
  - Result

### UTXO Overview
`📄 05-tools-resources/bitcoin--utxo.md`

- UTXO-based model
- What is a UTXO?
- Properties of UTXOs
- Account model vs UTXO model
- Querying UTXO data with Alchemy

### What is the Bitcoin genesis block?
`📄 05-tools-resources/bitcoin-genesis-block.md`

- Learn More About Bitcoin

### Bitcoin Cash API Overview
`📄 05-tools-resources/bitcoincash--bitcoincash-api-overview.md`

- Bitcoin Cash APIs

### UTXO WebSockets
`📄 05-tools-resources/bitcoincash--utxo-websockets.md`

- subscribeNewBlock
  - Parameters
  - Response
  - Request
  - Result
- subscribeNewTransaction
  - Parameters
  - Response
  - Request
  - Result
- subscribeAddresses
  - Parameters
  - Response
  - Request
  - Result
- subscribeFiatRates
  - Parameters
  - Response
  - Request
  - Result

### UTXO Overview
`📄 05-tools-resources/bitcoincash--utxo.md`

- UTXO-based model
- What is a UTXO?
- Properties of UTXOs
- Account model vs UTXO model
- Querying UTXO data with Alchemy

### Blast API Overview
`📄 05-tools-resources/blast--blast-api-overview.md`

- Blast APIs
- Related APIs

### BOB API Overview
`📄 05-tools-resources/bob--bob-api-overview.md`

- BOB APIs
- Related APIs

### Botanix API Overview
`📄 05-tools-resources/botanix--botanix-api-overview.md`

- Botanix APIs
- Related APIs

### Building a MetaMask Snap from scratch
`📄 05-tools-resources/building-a-metamask-snap-from-scratch.md`


### Celo API Overview
`📄 05-tools-resources/celo--celo-api-overview.md`

- Celo APIs
- Related APIs

### Choosing a Web3 Network
`📄 05-tools-resources/choosing-a-web3-network.md`

- Layer 1 Networks supported by Alchemy
- Layer 2 Networks supported by Alchemy
- Ethereum
  - Ethereum Mainnet
  - Sepolia Testnet
- Polygon
  - Polygon Mainnet
  - Polygon Amoy Testnet
- Optimism
  - Optimism Mainnet
  - Optimism Sepolia Testnet
- Arbitrum
  - Arbitrum Mainnet
  - Arbitrum Sepolia Testnet
- Base
  - Base Mainnet
  - Base Sepolia Testnet
- Starknet
  - Starknet Mainnet
- Astar
  - Astar Mainnet
- Frax
  - Frax Mainnet
  - Frax Hoodi Testnet
- Zora
  - Zora Mainnet
  - Zora Sepolia Testnet
- Solana
  - Solana Mainnet
  - Solana Devnet
- Which Ethereum Testnet Should I use?

### Citrea API Overview
`📄 05-tools-resources/citrea--citrea-api-overview.md`

- Citrea APIs
- Related APIs

### Clankermon API Overview
`📄 05-tools-resources/clankermon--clankermon-api-overview.md`

- Clankermon APIs
- Related APIs

### Create an Alchemy API key
`📄 05-tools-resources/create-an-api-key.md`

- Prerequisites
- Verify it works
- Next steps

### How to create a JSON REST API for Ethereum
`📄 05-tools-resources/create-json-rest-api.md`

- Setting Up 📦
- The Server 💻
  - The GET Request
  - The POST Request
- Making the Request
- Challenge 1: Customize 🎨
- Challenge 2: DELETE Request ⚔️
- Challenge 3: PUT Request ⚔️
- Learn More About Ethereum

### Cronos API Overview
`📄 05-tools-resources/cronos--cronos-api-overview.md`

- Cronos APIs
- Related APIs

### CrossFi API Overview
`📄 05-tools-resources/crossfi--crossfi-api-overview.md`

- CrossFi APIs
- Related APIs

### Cryptography Basics
`📄 05-tools-resources/cryptography-basics.md`

- Learn More About Cryptography

### Dashboard Alerts
`📄 05-tools-resources/dashboard-alerts.md`


### Dashboard Roles
`📄 05-tools-resources/dashboard-roles.md`


### Dashboard SSO
`📄 05-tools-resources/dashboard-sso.md`

- Requirements
- Setup Instructions
  - 1. Create a new SAML application in your IdP
  - 2. Configure Attribute Mappings
  - 3. Share Metadata with Us
- Logging In

### Dashboard Tools Quickstart
`📄 05-tools-resources/dashboard-tools-quickstart.md`


### Debugging CORS problems for End-Users
`📄 05-tools-resources/debugging-cors-problems-for-end-users.md`

- Overview
- Examples
- Some causes and fixes
  - The user has an antivirus such as Bitdefender installed
  - The user's ISP or router is blocking the website
  - Your application uses a browser extension (unlikely)
- Submitting a persistent problem
- Setting up a CORS proxy

### Understanding Logs: Deep Dive into eth_getLogs
`📄 05-tools-resources/deep-dive-into-eth_getlogs.md`

- Making a Request to eth\_getLogs
- Deciphering the Response

### Choose Your Starting Point
`📄 05-tools-resources/get-started.md`

- 1. Chain APIs
- 2. Data APIs
- 3. Wallet APIs / Account Abstraction Infrastructure
- 4. Rollups

### Gnosis API Overview
`📄 05-tools-resources/gnosis--gnosis-api-overview.md`

- Gnosis APIs
- Related APIs

### What is a hashing algorithm?
`📄 05-tools-resources/hashing-algorithm.md`

  - Cryptographic Hashing Algorithms
- Learn More About Hashing

### How do Solidity structs work?
`📄 05-tools-resources/how-do-solidity-structs-work.md`

- Structs
  - Struct Use Cases: Library Record Keeping 📚
    - - For each book record I add to my `Library` smart contract, I want to keep track of its `title`, `author` and some sort of `id` for internal record-keeping
- Suggested Reading
- Conclusion
- Learn More About Ethereum Development

### How does Solidity work with the EVM?
`📄 05-tools-resources/how-does-solidity-work.md`

- Talk Bytecode To Me 🗣
- 🏁 Wrap Up
- Learn More About Solidity

### How do Solidity arrays work?
`📄 05-tools-resources/how-solidity-arrays-work.md`

  - What Are Dynamic & Fixed Arrays?
  - Storage Arrays
- Structs
  - Struct Use Cases: Library Record Keeping 📚
    - - For each book record I add to my `Library` smart contract, I want to keep track of its `title`, `author` and some sort of `id` for internal record-keeping
- Suggested Reading
- Conclusion
- Learn More About Ethereum Development

### How to Add Allowlists to Your Apps for Enhanced Security
`📄 05-tools-resources/how-to-add-allowlists-to-your-apps-for-enhanced-security.md`

- Introduction
- Restricting Access To Apps
  - 1. Allowlist Addresses
    - Steps to Test
  - 2. Allowlist Domains
    - Steps to Test
  - 3. Allowlist IPs
    - Steps to Test
- Conclusion

### How to check a transaction status from its hash
`📄 05-tools-resources/how-to-check-the-status-of-a-transaction-using-its-hash.md`

- Prerequisites
- How to interpret the result
- Next steps

### How to Create Access Keys
`📄 05-tools-resources/how-to-create-access-keys.md`

- Introduction to Access Keys
- Generating Access Keys
- Using Access Keys
  - Using as Path Param
  - Using as Auth Header

### How to Deploy a Smart Contract to the Sepolia Testnet
`📄 05-tools-resources/how-to-deploy-a-smart-contract-to-the-sepolia-testnet.md`

- Connecting to the Ethereum Network
- Create Your App and Obtain an API Key
- Setting Up Your Ethereum Account
- Adding Ether from a Sepolia Faucet
- Initiating Our Project
- Downloading and Setting Up Hardhat
- Creating a Hardhat Project
- Creating Project Folders
- Writing Contract
- Integrate Metamask and Alchemy with Your Project
- Installing Ethers.js
- Updating hardhat.config.js File
- Compiling Contract
- Writing Deploy Script
- Deploying Contract

### How to Get a Contract's First Transfer Event
`📄 05-tools-resources/how-to-get-a-contracts-first-transfer-event.md`

- Install Node.js
- Setup Project Environment
- Setup for API Calls
- Get Contract's First Transfer Event

### How to Get a Contract's Last Transfer Event
`📄 05-tools-resources/how-to-get-a-contracts-last-transfer-event.md`

- Install Node.js
- Setup Project Environment
- Using Native JavaScript
- Get Contract's Last Transfer Event
- Use Page Keys

### How to Get a Smart Contract's Balance in Solidity
`📄 05-tools-resources/how-to-get-a-smart-contracts-balance-in-solidity.md`


### How to Get Contract Deployment Transactions in a Block
`📄 05-tools-resources/how-to-get-contract-deployment-transactions-in-a-block.md`

- Step 1: Install Node and NPM
- Step 2: Create an Alchemy App
- Step 3: Create a Node Project
- Approach 1: The `to` address
  - Writing the script
  - Testing the Script
- Approach 2: OPCODES
  - Writing the script
  - Testing the Script
- Comparing the Two Approaches

### How to Get the Number of Transactions in a Block
`📄 05-tools-resources/how-to-get-the-number-of-transactions-in-a-block.md`

- 1. Create a project directory
- 2. Install HTTP client library
- 3. Create `index.js`
- 4. Run it using node
- 5. Converting HEX to Decimal:

### How to Query Transaction Details on Ethereum
`📄 05-tools-resources/how-to-get-transaction-details.md`

- Step 1: Install Node and NPM
- Step 2: Create an Alchemy app
- Step 3: Create a node project
- Step 4: Get the Transaction Receipt

### How to Handle Checksum Addresses
`📄 05-tools-resources/how-to-handle-checksum-addresses.md`

- What are checksum addresses?
- Handling checksum addresses with ethers
- Conclusion

### How to Implement Retries
`📄 05-tools-resources/how-to-implement-retries.md`


### How to Interact with ERC-20 tokens in Solidity
`📄 05-tools-resources/how-to-interact-with-erc-20-tokens-in-solidity.md`

- Step 1: Add a License and Specify the Solidity Version
- Step 2: Define the Interface
- Step 3: Define the Smart Contract

### How to Interact with ERC-721 Tokens in Solidity
`📄 05-tools-resources/how-to-interact-with-erc-721-tokens-in-solidity.md`

- Creating the Interaction Contract
- Interacting with other ERC-721 contracts
  - Step 1: Install Node and npm
  - Step 2: Create a Hardhat project
  - Step 3: Write the incorrect NFT smart contract
  - Step 4: Write the replacement NFT smart contract
  - Step 5: Simulate functionality locally
- Conclusion

### How to Interpret Binaries in Solidity
`📄 05-tools-resources/how-to-interpret-binaries-in-solidity.md`

- Why does Solidity encode smart contracts in binary?
- What is a Solidity ABI (application binary interface)? Why do you need one to read a smart contract?
- How to Interpret Call Data Binaries from Solidity
  - 1. Use the first 4 bytes of the call data to identify the method ID.
  - 2. Use the following 32 bytes to identify the first parameter
  - 3. Use the final 32 bytes to identify the second parameter
- How can I interpret event data binaries from Solidity?
- What tools should I use to decompile Solidity binaries?

### How to Modify State Variables
`📄 05-tools-resources/how-to-modify-state-variables.md`

- Step 1: Set Up Project Structure Using Hardhat
- Step 2: Create Smart Contract
- Step 3: Create Test
- Step 4: Run the Test
  - Extra Challenges:
- Learn More About Ethereum Development

### How to Read Data with JSON-RPC
`📄 05-tools-resources/how-to-read-data-with-json-rpc.md`

- How to read Ethereum data?
- What We Are Ultimately Trying To Build
- Core Concept: Ethereum Clients
- Core Concept: JSON-RPC
  - Visualization of API Standards: REST and JSON-RPC
  - JSON-RPC Request
  - JSON-RPC Response
  - JSON-RPC Tools
  - Suggested Reading
- 🏁 Conclusion
- Learn More About Ethereum Development

### How to Send Value from Within a Smart Contract Using Solidity
`📄 05-tools-resources/how-to-send-value-from-within-a-smart-contract-using-solidity.md`

- Step 1: Add a License and Specify the Solidity Version
- Step 2: Implement the Receive Function
- Step 3: Set the Value to Be Sent
- Step 4: Use `call` Function to Send Value

### How to set usage limits for your account
`📄 05-tools-resources/how-to-set-usage-limits-and-alerts-for-your-account.md`

  - 1. Accessing the Alchemy Dashboard
  - 2. Navigate to the billing section
  - 3. Configure auto-scale options
  - 4. Setting your preferences

### How to Subscribe to Mined Transactions via WebSocket Endpoints
`📄 05-tools-resources/how-to-subscribe-to-pending-transactions-via-websocket-endpoints.md`

- Step 0: Configure your developer environment
- Step 1: Open your Alchemy App
- Step 2: Get WebSocket URL from Alchemy App
- Step 3: Output Mined Transactions Using wscat
- Step 4: Create a Node project
- Step 5: Output Mined Transactions using WebSocket libraries
- Step 6: Filter Mined Transactions

### How to Subscribe to Pending Transactions via WebSocket Endpoints
`📄 05-tools-resources/how-to-subscribe-to-transactions-via-websocket-endpoints.md`

- Step 0: Configure your developer environment
- Step 1: Open your Alchemy App
- Step 2: Get WebSocket URL from Alchemy App
- Step 3: Output Pending Transactions Using wscat
- Step 4: Create a Node project
- Step 5: Output pending transactions via WebSockets
- Step 6: Filter Pending Transactions

### How to unit test a smart contract
`📄 05-tools-resources/how-to-unit-test-a-smart-contract.md`

- Guide Requirements
- Useful JS + Solidity Testing Resources
- Step 1: Hardhat Project Structure Setup
- Step 2: Add a `Faucet.sol` Contract File
- Step 3: Add Test File Structure
    - - A lot of the logic in the contract depends on the owner being set correctly in the constructor, so we'll want to test that.
- Step 4. Add Withdrawal Amount Test 🔎
    - - We don't want someone instantly draining all of our funds, so we should check that the `require` clause in the `withdraw()` function works as expected
- Step 5 - Challenge: Add Critical Function Tests ☢️
    - - **The `destroyFaucet()` function should only be called by the contract owner, as should the `withdrawAll` function.**
- Learn More About Ethereum Development

### How to make HTTP header-based API requests
`📄 05-tools-resources/how-to-use-api-keys-in-http-headers.md`

- Why use headers
- Prerequisites
- Quick cURL example
- Next steps

### How To Use JWTs For API Requests
`📄 05-tools-resources/how-to-use-jwts-for-api-requests.md`

- Introduction
- What are JWTs
- How do JWTs Work with Alchemy?
- Setting up the Project
- Generating a Public / Private Key Pair
- Setting up the Public Key in Alchemy Dashboard
- Generating the JWT
- Testing the JWT ( Optional )
- Making an API Request with JWT
- How do I avoid JWTs interfering with users experience connecting via Wallet Connect?
- Conclusion

### Humanity API Overview
`📄 05-tools-resources/humanity--humanity-api-overview.md`

- Humanity APIs
- Related APIs

### Hyperliquid API Overview
`📄 05-tools-resources/hyperliquid--hyperliquid-api-overview.md`

- Hyperliquid APIs
- Related APIs

### Injective API Overview
`📄 05-tools-resources/injective--injective-api-overview.md`

- Injective APIs
- Related APIs

### Ink API Overview
`📄 05-tools-resources/ink--ink-api-overview.md`

- Ink APIs
- Related APIs

### Integrating Historical Transaction Data into your dApp
`📄 05-tools-resources/integrating-historical-transaction-data-into-your-dapp.md`

- Overview
- Our Example
  - Problem Statement: 🐕
- Option 1: Building the dApp using Heroku
  - 1. Set Up Github Repo & Heroku
    - a) Make a clone of the existing [Github Repository](https://github.com/pileofscraps/alchemy_notify.git)
    - b) Install Heroku-CLI and verify/install dependencies
    - c) Initiate Heroku
  - 2. Create a [Free Alchemy Account](https://alchemy.com/?r=affiliate:edc790e6-b0b0-41ce-9c6b-25959178d827)
  - 3. Integrate Alchemy Transfers API
  - 4. Insert Alchemy API Key
  - 5. Deploy Heroku App!
- Option 2: Build the dApp From Scratch
  - 1-2. Complete [Steps 2 & 3](/docs/reference/transfers-api-quickstart#2-create-a-free-alchemy-account) from the Heroku Project.
  - 3. Configure your App with your Alchemy API Key
  - 4. Create Backend Processing Script
    - a) Install necessary dependencies
  - 5. Create dApp Dashboard Frontend
    - a) Create your `index.html` **file**
    - b) Create "Refresh Data" Button
    - c) Create UI elements for the Python script
    - d) Add Other HTML Elements
    - e) Deploy!
- **Conclusion**

### Integrating Simulation with 1 line of code
`📄 05-tools-resources/integrating-simulation-with-1-line-of-code.md`

- eth\_signTransaction
- ethers.js

### Jovay API Overview
`📄 05-tools-resources/jovay--jovay-api-overview.md`

- Jovay APIs
- Related APIs

### Katana API Overview
`📄 05-tools-resources/katana--katana-api-overview.md`

- Katana APIs
- Related APIs

### Lens API Overview
`📄 05-tools-resources/lens--lens-api-overview.md`

- Lens APIs
- Related APIs

### Linea API Overview
`📄 05-tools-resources/linea--linea-api-overview.md`

- Linea APIs
- Related APIs

### Litecoin API Overview
`📄 05-tools-resources/litecoin--litecoin-api-overview.md`

- Litecoin APIs

### UTXO WebSockets
`📄 05-tools-resources/litecoin--utxo-websockets.md`

- subscribeNewBlock
  - Parameters
  - Response
  - Request
  - Result
- subscribeNewTransaction
  - Parameters
  - Response
  - Request
  - Result
- subscribeAddresses
  - Parameters
  - Response
  - Request
  - Result
- subscribeFiatRates
  - Parameters
  - Response
  - Request
  - Result

### UTXO Overview
`📄 05-tools-resources/litecoin--utxo.md`

- UTXO-based model
- What is a UTXO?
- Properties of UTXOs
- Account model vs UTXO model
- Querying UTXO data with Alchemy

### Make your first Alchemy request
`📄 05-tools-resources/make-your-first-request.md`

- Prerequisites
- Next steps

### Mantle API Overview
`📄 05-tools-resources/mantle--mantle-api-overview.md`

- Mantle APIs
- Related APIs

### MegaETH API Overview
`📄 05-tools-resources/megaeth--megaeth-api-overview.md`

- MegaETH APIs
- Related APIs

### Metis API Overview
`📄 05-tools-resources/metis--metis-api-overview.md`

- Metis APIs
- Related APIs

### Mode API Overview
`📄 05-tools-resources/mode--mode-api-overview.md`

- Mode APIs
- Related APIs

### Monad API Overview
`📄 05-tools-resources/monad--monad-api-overview.md`

- Monad APIs
- Related APIs

### Moonbeam API Overview
`📄 05-tools-resources/moonbeam--moonbeam-api-overview.md`

- Moonbeam APIs
- Related APIs

### What are multi-signature contracts?
`📄 05-tools-resources/multi-sig-contracts.md`

- Multi-Sigs Overview
- Multi-Sig Utility
  - No Single Point of Failure
- Multi-Sig Contract Wallet Use Cases
- Gnosis Safe
- Suggested Reading
- Conclusion
- Learn More About Ethereum Development

### Mythos API Overview
`📄 05-tools-resources/mythos--mythos-api-overview.md`

- Mythos APIs
- Related APIs

### What are Patricia Merkle Tries?
`📄 05-tools-resources/patricia-merkle-tries.md`

- Intro
  - Review; Bitcoin: Block Architecture
  - First Look: Ethereum Block Architecture
    - Why are you showing me block architectures, aren't we covering trees?
  - Review: Merkle Trees in Bitcoin
  - Trees in Ethereum
  - Radix Trie
  - Patricia Merkle Trees
    - Patricia??
- Why Does Ethereum Use a Merkle Patricia Trie?
- Ethereum Block Header
- Ethereum: State Trie
    - Account Example
- Ethereum: Transaction Trie
    - Transaction Example
- Ethereum: Transaction Receipt Trie
    - Transaction Receipt Example
- Conclusion
- Learn More About Ethereum

### Pharos API Overview
`📄 05-tools-resources/pharos--pharos-api-overview.md`

- Pharos APIs
- Related APIs

### Plasma API Overview
`📄 05-tools-resources/plasma--plasma-api-overview.md`

- Plasma APIs
- Related APIs

### What is Proof of Work?
`📄 05-tools-resources/proof-of-work.md`

  - Why would you use Proof of Work?
  - How does Bitcoin use Proof-of-Work?
- Learn More About Proof-of-Work

### What is Public Key Cryptography?
`📄 05-tools-resources/public-key-cryptography.md`

- Cryptography Historically
- Personal Computing
  - Start Thought Experiment 🧠
  - End Thought Experiment 🧠
- RSA and ECDSA
- Learn More About Crypto

### Abstract API FAQ
`📄 05-tools-resources/reference--abstract-api-faq.md`

- What is Abstract?
- What is the Abstract API?
- How can I get started using Abstract?
- Is Abstract EVM compatible?
- What unique features does Abstract offer?
- How does Abstract's architecture work?
- How does Abstract ensure scalability and security?
- My question isn't here, where can I get help?

### Abstract API Quickstart
`📄 05-tools-resources/reference--abstract-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### ADI API FAQ
`📄 05-tools-resources/reference--adi-api-faq.md`

- What is ADI?
- How do I get started with ADI?
- What is the ADI API?
- Is ADI EVM compatible?
- What API does ADI use?
- What methods are supported on ADI?
- What is an ADI API key?
- Which libraries support ADI?
- What is the native currency of ADI?
- My question isn’t here, where can I get help?

### ADI API Quickstart
`📄 05-tools-resources/reference--adi-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### App Management API Overview
`📄 05-tools-resources/reference--admin-api--overview.md`

- What you can do
- Authentication

### Admin API Quickstart
`📄 05-tools-resources/reference--admin-api--quickstart.md`

- Get started
  - Prerequisites
- Set up Environment Variables
- Get chain network options
- 1. Create an app
- 2. Get app info
- 3. Update app configuration
  - Update app name and description
  - Update network allowlist (optional)
  - Update origin allowlist (optional)
  - Update IP allowlist (optional)
- 4. Delete the app

### Anime API FAQ
`📄 05-tools-resources/reference--anime-api-faq.md`

- What is Anime?
- What is the Anime API?
- How can I get started using the Anime API?
- Is Anime EVM compatible?
- What API does Anime use?
- What is a Anime API key?
- Which libraries support Anime?
- What programming languages work with Anime?
- What does Anime use for gas?
- What testnet should I use for Anime?
- What methods does Alchemy support for the Anime API?
- My question isn't here, where can I get help?

### Anime API Quickstart
`📄 05-tools-resources/reference--anime-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Aptos API FAQ
`📄 05-tools-resources/reference--aptos-api-faq.md`

- What is Aptos?
- What is the Aptos API?
- How can I get started using the Aptos API?
- Does Aptos support smart contracts?
- What API standard does Aptos use?
- What is an Aptos API key?
- Which libraries can I use with the Aptos API?
- What programming languages are compatible with the API?
- What is used for fees in Aptos?
- My question isn't listed here — where can I get help?

### Aptos API Quickstart
`📄 05-tools-resources/reference--aptos-api-quickstart.md`

- What is the Aptos Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Arc API FAQ
`📄 05-tools-resources/reference--arc-api-faq.md`

- What is Arc?
- How do I get started with Arc?
- What is the Arc API?
- Is Arc EVM compatible?
- What API does Arc use?
- What methods are supported on Arc?
- What is a Arc API key?
- Which libraries support Arc?

### Arc API Quickstart
`📄 05-tools-resources/reference--arc-api-quickstart.md`

- Introduction
- What is the Arc API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Astar API FAQ
`📄 05-tools-resources/reference--astar-api-faq.md`

- What is the Astar Network?
- What is the Astar API?
- How can I get started using the Astart API?
- What type of scaling solution is Astar?
- Is Astar EVM compatible?
- How do I add Astar to MetaMask?
- What testnet should I use for Astar?
- How do I build an app on Astar?
- How do you bridge Astar to Ethereum?
- What wallets can be used with Astar?
- What does Astar use for gas?
- What projects are on Astar?
- How do you withdraw ETH from Astar?
- What API does Astar use?
- What is an Astar API key?
- What libraries support Astar?
- What programming languages work with Astar?
- What methods does Alchemy support for the Astar API?
- My question isn't here, where can I get help?

### Astar API Quickstart
`📄 05-tools-resources/reference--astar-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Avalanche C-Chain API FAQ
`📄 05-tools-resources/reference--avalanche-api-faq.md`

- What is Avalanche?
- What is the Avalanche API?
- How can I get started using the Avalanche API?
- Is Avalanche EVM compatible?
- What API does Avalanche use?
- What is an Avalanche API key?
- Which libraries support Avalanche?
- What programming languages work with Avalanche?
- What does Avalanche use for gas?
- What methods does Alchemy support for the Avalanche API?
- My question isn't here, where can I get help?

### Avalanche C-Chain API Quickstart
`📄 05-tools-resources/reference--avalanche-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Avalanche P-Chain API FAQ
`📄 05-tools-resources/reference--avalanche-p-api-faq.md`

- What is the Avalanche P-Chain?
- What is the Avalanche P-Chain API?
- How can I get started using the Avalanche P-Chain API?
- Is the Avalanche P-Chain EVM compatible?
- What API does the Avalanche P-Chain use?
- What is an Avalanche API key?
- Which libraries support the Avalanche P-Chain?
- What programming languages work with the Avalanche P-Chain?
- What does the Avalanche P-Chain use for gas?
- Supported Avalanche P-Chain methods
- My question isn't here, where can I get help?

### Avalanche P-Chain API Quickstart
`📄 05-tools-resources/reference--avalanche-p-api-quickstart.md`

- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
- Send your first request on Alchemy (curl)
- Get current validators
- Get min stake
- Get subnets

### Batch Requests
`📄 05-tools-resources/reference--batch-requests.md`

- What is a batch request?
- What is the batch requests limit over HTTP?
- What is the batch request limit over WebSockets?
- Unsupported batch request APIs
- How do you make a batch request?
    - Single eth\_blockNumber request
    - Batch eth\_blockNumber request
- How do you make batch requests over REST?
- Handling errors in batch requests

### Best Practices for Using WebSockets in Web3
`📄 05-tools-resources/reference--best-practices-for-using-websockets-in-web3.md`

- How WebSockets differ from HTTP
- How can I set up a WebSocket connection?
- Subscribe to Ethereum blockchain updates
- Keep subscription scope narrow
- Subscribe to transaction updates
- When to use HTTPS over WebSockets
- 1. Silent failures
- 2. Load balancing
- 3. Retries
- 4. HTTP status codes
- 5. gZip Compression
- Conclusion

### Bitcoin API FAQ
`📄 05-tools-resources/reference--bitcoin-api-faq.md`

- What is Bitcoin?
- What is the Bitcoin API?
- How can I get started using the Bitcoin API?
- Does Bitcoin support smart contracts?
- What API standard does Bitcoin use?
- What is a Bitcoin API key?
- Which libraries can I use with the Bitcoin API?
- What programming languages are compatible with the API?
- What is used for fees in Bitcoin?
- What methods does Alchemy support for the Bitcoin API?
- My question isn't listed here — where can I get help?

### Bitcoin API Quickstart
`📄 05-tools-resources/reference--bitcoin-api-quickstart.md`

- What is the Bitcoin Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Bitcoin Cash API FAQ
`📄 05-tools-resources/reference--bitcoincash-api-faq.md`

- What is Bitcoin Cash?
- What is the Bitcoin Cash API?
- How can I get started using the Bitcoin Cash API?
- Does Bitcoin Cash support smart contracts?
- What API standard does Bitcoin Cash use?
- What is a Bitcoin Cash API key?
- Which libraries can I use with the Bitcoin Cash API?
- What programming languages are compatible with the API?
- What is used for fees in Bitcoin Cash?
- What methods does Alchemy support for the Bitcoin Cash API?
- My question isn't listed here, where can I get help?

### Bitcoin Cash API Quickstart
`📄 05-tools-resources/reference--bitcoincash-api-quickstart.md`

- What is the Bitcoin Cash Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Blast Chain API FAQ
`📄 05-tools-resources/reference--blast-api-faq.md`

- What is Blast Chain?
- What is the Blast Chain API?
- How can I get started using the Blast Chain API?
- Is Blast Chain EVM compatible?
- What API does Blast Chain use?
- What is a Blast Chain API key?
- Which libraries support Blast Chain?
- What programming languages work with Blast Chain?
- What does Blast Chain use for gas?
- What methods does Alchemy support for the Blast Chain API?
- My question isn't here, where can I get help?

### Blast Chain API Quickstart
`📄 05-tools-resources/reference--blast-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Bob API FAQ
`📄 05-tools-resources/reference--bob-api-faq.md`

- What is Bob?
- How do I get started with Bob?
- What is the Bob API?
- Is Bob EVM compatible?
- What API does Bob use?
- What methods are supported on Bob?
- What is a Bob API key?
- Which libraries support Bob?

### BOB API Quickstart
`📄 05-tools-resources/reference--bob-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Botanix API FAQ
`📄 05-tools-resources/reference--botanix-api-faq.md`

- What is Botanix?
- How do I get started with Botanix?
- What is the Botanix API?
- Is Botanix EVM compatible?
- What API does Botanix use?
- What is a Botanix API key?
- Which libraries support Botanix?
- What programming languages work with Botanix?
- What does Botanix use for gas?
- What methods does Alchemy support for the Botanix API?
- My question isn't here, where can I get help?

### Botanix API Quickstart
`📄 05-tools-resources/reference--botanix-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Tutorial: Bridging Assets
`📄 05-tools-resources/reference--bridge-contract-address.md`


### Celo Chain API FAQ
`📄 05-tools-resources/reference--celo-api-faq.md`

- What is Celo Chain?
- What is the Celo Chain API?
- How can I get started using the Celo Chain API?
- Is Celo Chain EVM compatible?
- What API does Celo Chain use?
- What is a Celo Chain API key?
- What testnet should I use for Celo?
- Which libraries support Celo Chain?
- What programming languages work with Celo Chain?
- What does Celo Chain use for gas?
- What methods does Alchemy support for the Celo Chain API?
- My question isn't here, where can I get help?

### Celo Chain API Quickstart
`📄 05-tools-resources/reference--celo-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Citrea API FAQ
`📄 05-tools-resources/reference--citrea-api-faq.md`

- What is Citrea?
- How do I get started with Citrea?
- What is the Citrea API?
- Is Citrea EVM compatible?
- What API does Citrea use?
- What methods are supported on Citrea?
- What is a Citrea API key?
- Which libraries support Citrea?

### Citrea API Quickstart
`📄 05-tools-resources/reference--citrea-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Clankermon API FAQ
`📄 05-tools-resources/reference--clankermon-api-faq.md`

- What is Clankermon?
- How do I get started with Clankermon?
- What is the Clankermon API?
- Is Clankermon EVM compatible?
- What API does Clankermon use?
- What methods are supported on Clankermon?
- What is a Clankermon API key?
- Which libraries support Clankermon?

### Clankermon API Quickstart
`📄 05-tools-resources/reference--clankermon-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Compute Unit Costs
`📄 05-tools-resources/reference--compute-unit-costs.md`

- What are Compute Units and Throughput Compute Units?

### Compute Units
`📄 05-tools-resources/reference--compute-units.md`


### Cronos API FAQ
`📄 05-tools-resources/reference--cronos-api-faq.md`

- What is Cronos?
- What is the Cronos API?
- How can I get started using the Cronos API?
- Is Cronos EVM compatible?
- What API does Cronos use?
- What is a Cronos API key?
- Which libraries support Cronos?
- What programming languages work with Cronos?
- What does Cronos use for gas?
- What methods does Alchemy support for the Cronos API?
- My question isn't here, where can I get help?

### Cronos API Quickstart
`📄 05-tools-resources/reference--cronos-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### CrossFi API FAQ
`📄 05-tools-resources/reference--crossfi-api-faq.md`

- What is CrossFi?
- What is the CrossFi API?
- How can I get started using the CrossFi API?
- Is CrossFi EVM compatible?
- What API does CrossFi use?
- What is a CrossFi API key?
- Which libraries support CrossFi?
- What programming languages work with CrossFi?
- What does CrossFi use for gas?
- What methods does Alchemy support for the CrossFi API?
- My question isn't here, where can I get help?

### CrossFi API Quickstart
`📄 05-tools-resources/reference--crossfi-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Customizations & Integrations
`📄 05-tools-resources/reference--customizations-integrations.md`

  - Supernode (Node API)
  - Embedded Wallets
  - Webhooks
  - Token API
  - Gas Manager

### DATA Network API FAQ
`📄 05-tools-resources/reference--data-network-api-faq.md`

- What is DATA Network?
- What is the DATA Network API?
- How can I get started using the DATA Network API?
- Is DATA Network EVM compatible?
- What API does DATA Network use?
- What is a DATA Network API key?
- Which libraries support DATA Network?
- What programming languages work with DATA Network?
- What does DATA Network use for gas?
- What methods does Alchemy support for the DATA Network API?
- My question isn't here, where can I get help?

### DATA Network API Quickstart
`📄 05-tools-resources/reference--data-network-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Degen API FAQ
`📄 05-tools-resources/reference--degen-api-faq.md`

- What is Degen?
- How do I get started with Degen?
- What is the Degen API?
- Is Degen EVM compatible?
- What API does Degen use?
- What methods are supported on Degen?
- What is a Degen API key?
- Which libraries support Degen?

### Degen API Quickstart
`📄 05-tools-resources/reference--degen-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Dogecoin API FAQ
`📄 05-tools-resources/reference--dogecoin-api-faq.md`

- What is Dogecoin?
- What is the Dogecoin API?
- How can I get started using the Dogecoin API?
- Does Dogecoin support smart contracts?
- What API standard does Dogecoin use?
- What is a Dogecoin API key?
- Which libraries can I use with the Dogecoin API?
- What programming languages are compatible with the API?
- What is used for fees in Dogecoin?
- What methods does Alchemy support for the Dogecoin API?
- My question isn't listed here, where can I get help?

### Dogecoin API Quickstart
`📄 05-tools-resources/reference--dogecoin-api-quickstart.md`

- What is the Dogecoin Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request

### Error Reference
`📄 05-tools-resources/reference--error-reference.md`

- How to solve ENOTFOUND getAddrInfo errors
- How to solve `ECONNRESET` errors?

### Fantom Network Deprecation Notice
`📄 05-tools-resources/reference--fantom-deprecation-notice.md`

- ⚠️ Deprecation Notice

### Flow API FAQ
`📄 05-tools-resources/reference--flow-evm-api-faq.md`

- What is Flow?
- What is the Flow API?
- How can I get started using the Flow API?
- Is Flow EVM-compatible?
- What API does Flow use?
- What is a Flow API key?
- What programming languages work with Flow?
- Which libraries support Flow?
- What does Flow use for gas?
- What methods does Alchemy support for the Flow API?
- Where can I get additional help?

### Flow EVM API Quickstart
`📄 05-tools-resources/reference--flow-evm-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Frax API FAQ
`📄 05-tools-resources/reference--frax-api-faq.md`

- What is Frax?
- How do I get started with Frax?
- What is the Frax API?
- Is Frax EVM compatible?
- What API does Frax use?
- What methods are supported on Frax?
- What is a Frax API key?
- Which libraries support Frax?

### Fraxtal API Quickstart
`📄 05-tools-resources/reference--frax-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Geist Network Deprecation Notice
`📄 05-tools-resources/reference--geist-deprecation-notice.md`

- ⚠️ Deprecation Notice
- Migration options
- Next steps

### Gensyn API FAQ
`📄 05-tools-resources/reference--gensyn-api-faq.md`

- What is Gensyn?
- How do I get started with Gensyn?
- What is the Gensyn API?
- Is Gensyn EVM compatible?
- What API does Gensyn use?
- What methods are supported on Gensyn?
- What is a Gensyn API key?
- Which libraries support Gensyn?

### Gensyn API Quickstart
`📄 05-tools-resources/reference--gensyn-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Gnosis Chain API FAQ
`📄 05-tools-resources/reference--gnosis-api-faq.md`

- What is Gnosis Chain?
- What is the Gnosis Chain API?
- How can I get started using the Gnosis Chain API?
- Is Gnosis Chain EVM compatible?
- What API does Gnosis Chain use?
- What is a Gnosis Chain API key?
- Which libraries support Gnosis Chain?
- What programming languages work with Gnosis Chain?
- What does Gnosis Chain use for gas?
- What methods does Alchemy support for the Gnosis Chain API?
- My question isn't here, where can I get help?

### Gnosis Chain API Quickstart
`📄 05-tools-resources/reference--gnosis-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Humanity API FAQ
`📄 05-tools-resources/reference--humanity-api-faq.md`

- What is Humanity?
- How do I get started with Humanity?
- What is the Humanity API?
- Is Humanity EVM compatible?
- What API does Humanity use?
- What methods are supported on Humanity?
- What is a Humanity API key?
- Which libraries support Humanity?

### Humanity API Quickstart
`📄 05-tools-resources/reference--humanity-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Hyperevm API FAQ
`📄 05-tools-resources/reference--hyperliquid-api-faq.md`

- What is HyperEVM?
- How do I get started with HyperEVM?
- What is the HyperEVM API?
- Is HyperEVM EVM compatible?
- What API does HyperEVM use?
- What is a HyperEVM API key?
- Which libraries support HyperEVM?
- What programming languages work with HyperEVM?
- What does HyperEVM use for gas?
- What methods does Alchemy support for the HyperEVM API?
- My question isn't here, where can I get help?

### HyperEVM API Quickstart
`📄 05-tools-resources/reference--hyperliquid-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Injective API FAQ
`📄 05-tools-resources/reference--injective-api-faq.md`

- What is Injective?
- How do I get started with Injective?
- What is the Injective API?
- Is Injective EVM compatible?
- What API does Injective use?
- What methods are supported on Injective?
- What is an Injective API key?
- Which libraries support Injective?
- Where can I get more help?

### Injective API Quickstart
`📄 05-tools-resources/reference--injective-api-quickstart.md`

- What is Injective?
- What is the Injective API?
- Get started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Ink API FAQ
`📄 05-tools-resources/reference--ink-api-faq.md`

- What is Ink Chain?
- What is the Ink Chain API?
- How can I get started using the Ink API?
- Is Ink EVM compatible?
- What API does Ink use?
- What is an Ink API key?
- Which libraries support Ink?
- What programming languages work with Ink?
- What does Ink use for gas?
- What methods does Kraken support for the Ink API?
- My question isn't here, where can I get help?

### Ink API Quickstart
`📄 05-tools-resources/reference--ink-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Jovay API FAQ
`📄 05-tools-resources/reference--jovay-api-faq.md`

- What is Jovay?
- How do I get started with Jovay?
- What is the Jovay API?
- Is Jovay EVM compatible?
- What API does Jovay use?
- What methods are supported on Jovay?
- What is a Jovay API key?
- Which libraries support Jovay?

### Jovay API Quickstart
`📄 05-tools-resources/reference--jovay-api-quickstart.md`

- What is Jovay?
- What is the Jovay API?
- Get started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Kaia API FAQ
`📄 05-tools-resources/reference--kaia-api-faq.md`

- What is Kaia?
- How do I get started with Kaia?
- What is the Kaia API?
- Is Kaia EVM compatible?
- What API does Kaia use?
- What methods are supported on Kaia?
- What is a Kaia API key?
- Which libraries support Kaia?

### Kaia API Quickstart
`📄 05-tools-resources/reference--kaia-api-quickstart.md`

- What is Kaia?
- What is the Kaia API?
- Get started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Katana API FAQ
`📄 05-tools-resources/reference--katana-api-faq.md`

- What is Katana?
- How do I get started with Katana?
- What is the Katana API?
- Is Katana EVM compatible?
- What API does Katana use?
- What methods are supported on Katana?
- What is a Katana API key?
- Which libraries support Katana?

### Katana API Quickstart
`📄 05-tools-resources/reference--katana-api-quickstart.md`

- What is Katana?
- What is the Katana API?
- Get started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Lens API FAQ
`📄 05-tools-resources/reference--lens-api-faq.md`

- What is Lens Network?
- What is the Lens Protocol?
- How does Lens Network differ from traditional social media?
- What technology does Lens Network use?
- What are the phases of Lens Network's rollout?
- What is Validium?
- What is Volition?
- How will Lens Network improve user experience?
- Will the new Lens Network be cross-chain compatible?
- Impact on existing Lens users and apps
- How does Lens Network address the blockchain "trilemma"?
- How can developers get involved with Lens Network?
- My question isn't here, where can I get help?

### Lens API Quickstart
`📄 05-tools-resources/reference--lens-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Linea Chain API FAQ
`📄 05-tools-resources/reference--linea-api-faq.md`

- What is Linea Chain?
- What is the Linea Chain API?
- How can I get started using the Linea Chain API?
- Is Linea Chain EVM compatible?
- What API does Linea Chain use?
- What is a Linea Chain API key?
- Which libraries support Linea Chain?
- What programming languages work with Linea Chain?
- What does Linea Chain use for gas?
- What methods does Alchemy support for the Linea Chain API?
- My question isn't here, where can I get help?

### Linea Chain API Quickstart
`📄 05-tools-resources/reference--linea-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Litecoin API FAQ
`📄 05-tools-resources/reference--litecoin-api-faq.md`

- What is Litecoin?
- What is the Litecoin API?
- How can I get started using the Litecoin API?
- Does Litecoin support smart contracts?
- What API standard does Litecoin use?
- What is a Litecoin API key?
- Which libraries can I use with the Litecoin API?
- What programming languages are compatible with the API?
- What is used for fees in Litecoin?
- What methods does Alchemy support for the Litecoin API?
- My question isn't listed here, where can I get help?

### Litecoin API Quickstart
`📄 05-tools-resources/reference--litecoin-api-quickstart.md`

- What is the Litecoin Chain API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request

### logs
`📄 05-tools-resources/reference--logs.md`


### Lumia Network Deprecation Notice
`📄 05-tools-resources/reference--lumia-deprecation-notice.md`

- ⚠️ Deprecation Notice

### Mantle Chain API FAQ
`📄 05-tools-resources/reference--mantle-api-faq.md`

- What is Mantle Chain?
- What is the Mantle Chain API?
- How can I get started using the Mantle Chain API?
- Is Mantle Chain EVM compatible?
- What API does Mantle Chain use?
- What is a Mantle Chain API key?
- Which libraries support Mantle Chain?
- What programming languages work with Mantle Chain?
- What does Mantle Chain use for gas?
- What methods does Alchemy support for the Mantle Chain API?
- My question isn't here, where can I get help?

### Mantle Chain API Quickstart
`📄 05-tools-resources/reference--mantle-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### MegaETH API FAQ
`📄 05-tools-resources/reference--megaeth-api-faq.md`

- What is MegaETH?
- How do I get started with MegaETH?
- What is the MegaETH API?
- Is MegaETH EVM compatible?
- What API does MegaETH use?
- What methods are supported on MegaETH?
- What is a MegaETH API key?
- Which libraries support MegaETH?

### MegaETH API Quickstart
`📄 05-tools-resources/reference--megaeth-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Metis Chain API FAQ
`📄 05-tools-resources/reference--metis-api-faq.md`

- What is Metis Chain?
- What is the Metis Chain API?
- What networks are supported for Metis Chain?
- How can I get started using the Metis Chain API?
- Is Metis Chain EVM compatible?
- What API does Metis Chain use?
- What is a Metis Chain API key?
- Which libraries support Metis Chain?
- What programming languages work with Metis Chain?
- What does Metis Chain use for gas?
- What methods does Alchemy support for the Metis Chain API?
- My question isn't here, where can I get help?

### Metis Chain API Quickstart
`📄 05-tools-resources/reference--metis-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Mode API FAQ
`📄 05-tools-resources/reference--mode-api-faq.md`

- What is Mode?
- How do I get started with Mode?
- What is the Mode API?
- Is Mode EVM compatible?
- What API does Mode use?
- What methods are supported on Mode?
- What is a Mode API key?
- Which libraries support Mode?

### Mode API Quickstart
`📄 05-tools-resources/reference--mode-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Moonbeam API FAQ
`📄 05-tools-resources/reference--moonbeam-api-faq.md`

- What is Moonbeam?
- How do I get started with Moonbeam?
- What is the Moonbeam API?
- Is Moonbeam EVM compatible?
- What API does Moonbeam use?
- What methods are supported on Moonbeam?
- What is a Moonbeam API key?
- Which libraries support Moonbeam?

### Moonbeam API Quickstart
`📄 05-tools-resources/reference--moonbeam-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Mythos API FAQ
`📄 05-tools-resources/reference--mythos-api-faq.md`

- What is Mythos?
- How do I get started with Mythos?
- What is the Mythos API?
- Is Mythos EVM compatible?
- What API does Mythos use?
- What methods are supported on Mythos?
- What is a Mythos API key?
- Which libraries support Mythos?

### Mythos API Quickstart
`📄 05-tools-resources/reference--mythos-api-quickstart.md`

- What is Mythos Chain?
- What is the Mythos API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### OP Mainnet Flashblocks API quickstart
`📄 05-tools-resources/reference--op-mainnet-flashblocks-api-quickstart.md`

- What are Flashblocks?
- Getting started
- Flashblocks-enabled API endpoints
  - [eth\_getBlockByNumber](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-block-by-number)
    - Example Response
  - [eth\_getTransactionReceipt](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-receipt)
    - Example Response
  - [eth\_getBalance](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-balance)
    - Example Response
  - [eth\_getTransactionCount](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-count)
    - Example Response
  - [eth\_getTransactionByHash](https://www.alchemy.com/docs/node/op-mainnet/op-mainnet-api-endpoints/eth-get-transaction-by-hash)
    - Example Response
  - [eth\_call](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-call)
    - Example Response
  - [eth\_simulateV1](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-simulate-v-1)
    - Example Response
  - [eth\_estimateGas](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-estimate-gas)
    - Example Response
  - [eth\_getLogs](https://www.alchemy.com/docs/chains/op-mainnet/op-mainnet-api-endpoints/eth-get-logs)
    - Example Response

### opBNB Chain API FAQ
`📄 05-tools-resources/reference--opbnb-api-faq.md`

- What is opBNB?
- What is the opBNB API?
- Can I use opBNB on free tier?
- How can I get started using the opBNB API?
- Is opBNB EVM compatible?
- What API does opBNB use?
- What is an opBNB API key?
- Which libraries support opBNB?
- What programming languages work with opBNB?
- What does opBNB use for gas?
- What methods does Alchemy support for the opBNB API?
- My question isn't here, where can I get help?

### opBNB Chain API Quickstart
`📄 05-tools-resources/reference--opbnb-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Pay As You Go Pricing FAQ
`📄 05-tools-resources/reference--pay-as-you-go-pricing-faq.md`


### Pharos API FAQ
`📄 05-tools-resources/reference--pharos-api-faq.md`

- What is Pharos?
- How do I get started with Pharos?
- What is the Pharos API?
- Is Pharos EVM compatible?
- What API does Pharos use?
- What methods are supported on Pharos?
- What is a Pharos API key?
- Which libraries support Pharos?

### Pharos API Quickstart
`📄 05-tools-resources/reference--pharos-api-quickstart.md`

- What is Pharos?
- What is the Pharos API?
- Getting started
  - 1. Choose a package manager (npm or yarn)
  - 2. Set up your project
  - 3. Make your first request
  - 4. Run your script
- Next steps

### Plasma API FAQ
`📄 05-tools-resources/reference--plasma-api-faq.md`

- What is Plasma?
- How do I get started with Plasma?
- What is the Plasma API?
- Is Plasma EVM compatible?
- What API does Plasma use?
- What methods are supported on Plasma?
- What is a Plasma API key?
- Which libraries support Plasma?

### Plasma API Quickstart
`📄 05-tools-resources/reference--plasma-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Pricing Plans
`📄 05-tools-resources/reference--pricing-plans.md`

- Feature comparison
- Account Abstraction (AA) Pricing
- Throughput (CUPS)

### Rise API FAQ
`📄 05-tools-resources/reference--rise-api-faq.md`

- What is Rise?
- How do I get started with Rise?
- What is the Rise API?
- Is Rise EVM compatible?
- What API does Rise use?
- What methods are supported on Rise?
- What is a Rise API key?
- Which libraries support Rise?

### Rise API Quickstart
`📄 05-tools-resources/reference--rise-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Ronin API FAQ
`📄 05-tools-resources/reference--ronin-api-faq.md`

- What is Ronin?
- How do I get started with Ronin?
- What is the Ronin API?
- Is Ronin EVM compatible?
- What API does Ronin use?
- What methods are supported on Ronin?
- What is a Ronin API key?
- Which libraries support Ronin?

### Ronin API Quickstart
`📄 05-tools-resources/reference--ronin-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Rootstock API FAQ
`📄 05-tools-resources/reference--rootstock-api-faq.md`

- What is Rootstock?
- What is the Rootstock API?
- How can I get started using the Rootstock API?
- Is Rootstock EVM compatible?
- What API does Rootstock use?
- What is an Rootstock API key?
- Which libraries support Rootstock?
- What programming languages work with Rootstock?
- What does Rootstock use for gas?
- How does Rootstock relate to Bitcoin?
- What is the Powpeg mechanism?
- My question isn't here, where can I get help?

### Rootstock API Quickstart
`📄 05-tools-resources/reference--rootstock-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Scroll Chain API FAQ
`📄 05-tools-resources/reference--scroll-api-faq.md`

- What is Scroll Chain?
- What is the Scroll Chain API?
- How can I get started using the Scroll Chain API?
- Is Scroll Chain EVM compatible?
- What API does Scroll Chain use?
- What is a Scroll Chain API key?
- Which libraries support Scroll Chain?
- What programming languages work with Scroll Chain?
- What does Scroll Chain use for gas?
- What methods does Alchemy support for the Scroll Chain API?
- My question isn't here, where can I get help?

### Scroll Chain API Quickstart
`📄 05-tools-resources/reference--scroll-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Sei API FAQ
`📄 05-tools-resources/reference--sei-api-faq.md`

- What is Sei?
- What is the Sei API?
- How can I get started using the Sei API?
- Is Sei EVM compatible?
- What API does Sei use?
- What is a Sei API key?
- Which libraries support Sei?
- What programming languages work with Sei?
- What does Sei use for gas?
- What methods does Alchemy support for the Sei API?
- My question isn't here, where can I get help?

### Sei API Quickstart
`📄 05-tools-resources/reference--sei-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Settlus API FAQ
`📄 05-tools-resources/reference--settlus-api-faq.md`

- What is Settlus?
- How do I get started with Settlus?
- What is the Settlus API?
- Is Settlus EVM compatible?
- What API does Settlus use?
- What methods are supported on Settlus?
- What is a Settlus API key?
- Which libraries support Settlus?

### Settlus API Quickstart
`📄 05-tools-resources/reference--settlus-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Shape API FAQ
`📄 05-tools-resources/reference--shape-api-faq.md`

- What is Shape?
- What is the Shape API?
- How can I get started using the Shape API?
- Is Shape EVM compatible?
- What is Gasback?
- How does Shape relate to Ethereum?
- What programming languages work with Shape?
- Is Shape secure?
- My question isn't here, where can I get help?

### Shape API Quickstart
`📄 05-tools-resources/reference--shape-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Soneium API FAQ
`📄 05-tools-resources/reference--soneium-api-faq.md`

- What is Soneium?
- What is the Soneium API?
- How can I get started using the Soneium API?
- Is Soneium EVM compatible?
- What API does Soneium use?
- What is a Soneium API key?
- Which libraries support Soneium?
- What programming languages work with Soneium?
- What does Soneium use for gas?
- What methods does Alchemy support for the Soneium API?
- My question isn't here, where can I get help?

### Soneium API Quickstart
`📄 05-tools-resources/reference--soneium-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

### Sonic Chain API FAQ
`📄 05-tools-resources/reference--sonic-api-faq.md`

- What is Sonic Chain?
- What is the Sonic Chain API?
- How can I get started using the Sonic Chain API?
- Is Sonic Chain EVM compatible?
- What API does Sonic Chain use?
- What is a Sonic Chain API key?
- Which libraries support Sonic Chain?
- What programming languages work with Sonic Chain?
- What does Sonic Chain use for gas?
- What methods does Alchemy support for the Sonic Chain API?
- My question isn't here, where can I get help?

### Sonic Chain API Quickstart
`📄 05-tools-resources/reference--sonic-api-quickstart.md`

- Send your first request on Alchemy
- Create a client connected to Alchemy
- Get the latest block number
- Get an address balance
- Read block data
- Fetch a transaction by hash
- Fetch a transaction receipt

---

## Stats

- Files: 426
- Headings (h2+): 4342
- 📦 Consolidated Templates: 5 files
- ⛓️ Chains: 91 files
- 📊 Data: 88 files
- 👛 Wallets: 5 files
- 🤖 Build with AI: 9 files
- 🛠️ Tools & Resources: 224 files
