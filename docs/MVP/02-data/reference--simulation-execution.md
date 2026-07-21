# Execution Simulation

> Source: [https://www.alchemy.com/docs/reference/simulation-execution.md](https://www.alchemy.com/docs/reference/simulation-execution.md)

# Execution Simulation

> Simulates a transaction and returns decoded execution traces and decoded logs.

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

<Info>
  Try it out - [alchemy_simulateExecution](https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution).
</Info>

## How does it work?

1. You give us a transaction (unsigned transaction object) 📦
2. We simulate your transaction on our infra ⚡
3. We return a complete list of decoded execution trace and logs 💸

We currently return the following info:

* gas used
* call trace
* logs
* ABI decoded