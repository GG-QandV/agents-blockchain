# Transaction Simulation

> Source: [https://www.alchemy.com/docs/reference/simulation.md](https://www.alchemy.com/docs/reference/simulation.md)

# Transaction Simulation

> Discover Alchemy's Transaction Simulation APIs for predicting the precise impact of a transaction before it reaches the blockchain.

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

With our Simulation APIs, you can know the exact impact of a transaction ***before*** it hits the blockchain.

The Transaction Simulation APIs help prevent unwanted hacks or theft, so you know exactly what a transaction will do by simulating it before you approve it in your wallet. This means identifying exactly what assets will be transferred, what logs will be emitted, what internal calls will be made, etc., before it even happens.

[**Asset Changes**](/docs/reference/simulation-asset-changes)<br/>
Simulates a transaction and returns a full list of asset changes.

[**Execution Simulation**](/docs/reference/simulation-execution)<br/>
Simulates a transaction and returns decoded execution traces and decoded logs.

[**Bundle Simulation**](/docs/reference/simulation-bundle)<br/>
Simulate multiple transactions sequentially.

<Info>
  Check the [Chains](https://dashboard.alchemy.com/signup) page for details about product and chain support!

  > 📄 **This content also appears in [logs](05-tools-resources/reference--logs.md)** — see there for full details.

For practical examples of these APIs you can check out the [Transaction Simulation examples](/docs/reference/simulation-examples) page and for frequently asked questions you can check out the [Transaction Simulation FAQ](/docs/reference/simulation-faqs) page.