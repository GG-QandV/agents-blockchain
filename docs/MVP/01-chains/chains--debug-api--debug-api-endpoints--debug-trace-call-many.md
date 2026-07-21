# debug_traceCallMany

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call-many.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call-many.md)

# debug_traceCallMany

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Runs multiple eth_call requests within the context of the given block execution using the final state of the parent block as the base. Each bundle can contain multiple transactions and optional block overrides. Returns nested arrays of traces (outer array per bundle, inner array per transaction).

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call-many

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| Bundles | object[] | Yes | An array of call bundles. Each bundle is an object containing an array of transaction call objects and optional block header overrides. |
| Simulation Context | object | Yes | The simulation context specifying the block to simulate against. |
| Options | object | No | Options for the trace including tracer type and state overrides. |

## Result

**Call traces** (object[][]): Nested array of call traces. The outer array corresponds to each bundle and the inner array corresponds to each transaction within a bundle.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_traceCallMany",
  "params": [
    [
      {
        "transactions": [
          {
            "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
            "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "gas": "0xb59b",
            "value": "0x0",
            "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc"
          }
        ]
      },
      {
        "transactions": [
          {
            "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
            "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
            "value": "0x0",
            "data": "0x"
          }
        ]
      }
    ],
    {
      "blockNumber": "0x154bd82",
      "transactionIndex": -1
    },
    {
      "tracer": "callTracer"
    }
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": [
    [
      {
        "type": "CALL",
        "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
        "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
        "value": "0x0",
        "gas": "0xb59b",
        "gasUsed": "0x5d5a",
        "input": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc",
        "output": "0x"
      }
    ],
    [
      {
        "type": "CALL",
        "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
        "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
        "value": "0x0",
        "gas": "0x7fffffffffffadf7",
        "gasUsed": "0x0",
        "input": "0x",
        "output": "0x"
      }
    ]
  ],
  "id": 1
}
```

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/v2/docs-demo \
  --header 'Content-Type: application/json' \
  --data '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "debug_traceCallMany",
  "params": [
    [
      {
        "transactions": [
          {
            "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
            "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "gas": "0xb59b",
            "value": "0x0",
            "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc"
          }
        ]
      },
      {
        "transactions": [
          {
            "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
            "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
            "value": "0x0",
            "data": "0x"
          }
        ]
      }
    ],
    {
      "blockNumber": "0x154bd82",
      "transactionIndex": -1
    },
    {
      "tracer": "callTracer"
    }
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    jsonrpc: '2.0',
    id: 1,
    method: 'debug_traceCallMany',
    params: [
      [
        {
          transactions: [
            {
              from: '0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2',
              to: '0xdac17f958d2ee523a2206206994597c13d831ec7',
              gas: '0xb59b',
              value: '0x0',
              data: '0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc'
            }
          ]
        },
        {
          transactions: [
            {
              from: '0xe088776deabb472ffd2843e330e79c880a5f979e',
              to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1',
              value: '0x0',
              data: '0x'
            }
          ]
        }
      ],
      {blockNumber: '0x154bd82', transactionIndex: -1},
      {tracer: 'callTracer'}
    ]
  })
};

fetch('https://eth-mainnet.g.alchemy.com/v2/docs-demo', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "debug_traceCallMany",
    "params": [[{ "transactions": [
                    {
                        "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
                        "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
                        "gas": "0xb59b",
                        "value": "0x0",
                        "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc"
                    }
                ] }, { "transactions": [
                    {
                        "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
                        "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
                        "value": "0x0",
                        "data": "0x"
                    }
                ] }], {
            "blockNumber": "0x154bd82",
            "transactionIndex": -1
        }, { "tracer": "callTracer" }]
}
headers = {"Content-Type": "application/json"}

response = requests.post(url, json=payload, headers=headers)

print(response.text)
```

### Go

```go
package main

import (
	"fmt"
	"strings"
	"net/http"
	"io"
)

func main() {

	url := "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCallMany\",\n  \"params\": [\n    [\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n            \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n            \"gas\": \"0xb59b\",\n            \"value\": \"0x0\",\n            \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\"\n          }\n        ]\n      },\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xe088776deabb472ffd2843e330e79c880a5f979e\",\n            \"to\": \"0x70526cc7a6d6320b44122ea9d2d07670accc85a1\",\n            \"value\": \"0x0\",\n            \"data\": \"0x\"\n          }\n        ]\n      }\n    ],\n    {\n      \"blockNumber\": \"0x154bd82\",\n      \"transactionIndex\": -1\n    },\n    {\n      \"tracer\": \"callTracer\"\n    }\n  ]\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Content-Type", "application/json")

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/v2/docs-demo")
  .header("Content-Type", "application/json")
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCallMany\",\n  \"params\": [\n    [\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n            \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n            \"gas\": \"0xb59b\",\n            \"value\": \"0x0\",\n            \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\"\n          }\n        ]\n      },\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xe088776deabb472ffd2843e330e79c880a5f979e\",\n            \"to\": \"0x70526cc7a6d6320b44122ea9d2d07670accc85a1\",\n            \"value\": \"0x0\",\n            \"data\": \"0x\"\n          }\n        ]\n      }\n    ],\n    {\n      \"blockNumber\": \"0x154bd82\",\n      \"transactionIndex\": -1\n    },\n    {\n      \"tracer\": \"callTracer\"\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCallMany\",\n  \"params\": [\n    [\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n            \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n            \"gas\": \"0xb59b\",\n            \"value\": \"0x0\",\n            \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\"\n          }\n        ]\n      },\n      {\n        \"transactions\": [\n          {\n            \"from\": \"0xe088776deabb472ffd2843e330e79c880a5f979e\",\n            \"to\": \"0x70526cc7a6d6320b44122ea9d2d07670accc85a1\",\n            \"value\": \"0x0\",\n            \"data\": \"0x\"\n          }\n        ]\n      }\n    ],\n    {\n      \"blockNumber\": \"0x154bd82\",\n      \"transactionIndex\": -1\n    },\n    {\n      \"tracer\": \"callTracer\"\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_traceCallMany
description: Runs multiple eth_call requests within the context of the given block execution using the final state of the parent block as the base. Each bundle can contain multiple transactions and optional block overrides. Returns nested arrays of traces (outer array per bundle, inner array per transaction).
x-compute-units: 40
x-rate-limit-cus: 1000
params:
  - name: Bundles
    required: true
    description: An array of call bundles. Each bundle is an object containing an array of transaction call objects and optional block header overrides.
    schema:
      type: array
      items:
        type: object
        properties:
          transactions:
            type: array
            description: Array of transaction call objects to simulate within this bundle.
            items:
              title: Transaction Object
              type: object
              properties:
                from:
                  type: string
                  pattern: ^0[xX][0-9a-fA-F]{40}$
                to:
                  type: string
                  pattern: ^0[xX][0-9a-fA-F]{40}$
                gas:
                  type: string
                  pattern: ^0[xX]([1-9a-fA-F][0-9a-fA-F]*|0)$
                gasPrice:
                  type: string
                  pattern: ^0[xX]([1-9a-fA-F][0-9a-fA-F]*|0)$
                value:
                  type: string
                  pattern: ^0[xX]([1-9a-fA-F][0-9a-fA-F]*|0)$
                data:
                  type: string
                  pattern: ^0[xX][0-9a-fA-F]*$
          blockOverride:
            type: object
            description: Optional overrides for the block header affecting all transactions in this bundle.
            properties:
              blockNumber:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
              blockHash:
                type: object
                description: A mapping of block number to user-defined hash queryable via the BLOCKHASH opcode.
                additionalProperties:
                  title: 32 byte hex value
                  type: string
                  pattern: ^0x[0-9a-f]{64}$
              coinbase:
                title: hex encoded address
                type: string
                pattern: ^0x[0-9a-fA-F]{40}$
              timestamp:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
              difficulty:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
              gasLimit:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
              baseFee:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
  - name: Simulation Context
    required: true
    description: The simulation context specifying the block to simulate against.
    schema:
      type: object
      properties:
        blockNumber:
          description: Block number or tag to simulate against.
          oneOf:
            - title: hex encoded bytes
              type: string
              pattern: ^0x[0-9a-f]*$
            - title: block tag
              type: string
              enum:
                - latest
                - earliest
                - pending
        transactionIndex:
          type: integer
          description: The index of the transaction within the block to simulate after. Default is -1 (after the entire block).
  - name: Options
    required: false
    description: Options for the trace including tracer type and state overrides.
    schema:
      type: object
      properties:
        tracer:
          type: string
          enum:
            - callTracer
            - prestateTracer
          description: The type of tracer to use.
        stateOverrides:
          type: object
          description: Address-to-state mapping that allows overriding account state. Keys are 20-byte account addresses (hex). Each value provides per-account overrides applied before the call executes.
          additionalProperties:
            type: object
            description: Per-account state overrides, matching geth's `OverrideAccount` struct. Note that `state` and `stateDiff` are mutually exclusive; `state` replaces the account's entire storage, while `stateDiff` only patches the listed slots.
            properties:
              balance:
                description: Hex-encoded balance to set for the account.
                title: hex encoded 256 bit unsigned integer
                type: string
                pattern: ^0x([1-9a-f]+[0-9a-f]{0,31})|0$
              nonce:
                description: Hex-encoded nonce to set for the account.
                title: hex encoded 64 bit unsigned integer
                type: string
                pattern: ^0x([1-9a-f]+[0-9a-f]{0,15})|0$
              code:
                title: hex encoded bytes
                type: string
                pattern: ^0x[0-9a-f]*$
                description: Hex-encoded bytecode to inject as the account's code.
              state:
                type: object
                description: Replaces the account's entire storage. Map of 32-byte storage slot key to 32-byte storage value. Mutually exclusive with `stateDiff`.
                additionalProperties:
                  title: 32 hex encoded bytes
                  type: string
                  pattern: ^0x[0-9a-f]{64}$
              stateDiff:
                type: object
                description: Patches individual storage slots without clearing the rest of storage. Map of 32-byte storage slot key to 32-byte storage value. Mutually exclusive with `state`.
                additionalProperties:
                  title: 32 hex encoded bytes
                  type: string
                  pattern: ^0x[0-9a-f]{64}$
              movePrecompileToAddress:
                title: hex encoded address
                type: string
                pattern: ^0x[0-9a-fA-F]{40}$
                description: Optional. Move the precompile residing at this account's address to the address specified here.
        timeout:
          type: string
          description: A duration string that overrides the default timeout for the trace (e.g., "5s", "300ms").
result:
  name: Call traces
  description: Nested array of call traces. The outer array corresponds to each bundle and the inner array corresponds to each transaction within a bundle.
  schema:
    type: array
    items:
      type: array
      items:
        type: object
        description: A call trace object for a single transaction.
        properties:
          type:
            type: string
            description: The type of call (e.g., CALL, CREATE, DELEGATECALL).
          from:
            type: string
            description: 20-byte address of the caller.
          to:
            type: string
            description: 20-byte address of the recipient. Null for contract creation transactions.
          value:
            type: string
            description: Amount of value included in the transfer (in hex).
          gas:
            type: string
            description: Amount of gas provided for the call (in hex).
          gasUsed:
            type: string
            description: Amount of gas used during the call (in hex).
          input:
            type: string
            description: Call data.
          output:
            type: string
            description: Return data.
examples:
  - name: debug_traceCallMany example
    params:
      - name: Bundles
        value:
          - transactions:
              - from: '0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2'
                to: '0xdac17f958d2ee523a2206206994597c13d831ec7'
                gas: '0xb59b'
                value: '0x0'
                data: '0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc'
          - transactions:
              - from: '0xe088776deabb472ffd2843e330e79c880a5f979e'
                to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1'
                value: '0x0'
                data: 0x
      - name: Simulation Context
        value:
          blockNumber: '0x154bd82'
          transactionIndex: -1
      - name: Options
        value:
          tracer: callTracer
    result:
      name: debug_traceCallMany response
      value:
        - - type: CALL
            from: '0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2'
            to: '0xdac17f958d2ee523a2206206994597c13d831ec7'
            value: '0x0'
            gas: '0xb59b'
            gasUsed: '0x5d5a'
            input: '0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc'
            output: 0x
        - - type: CALL
            from: '0xe088776deabb472ffd2843e330e79c880a5f979e'
            to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1'
            value: '0x0'
            gas: '0x7fffffffffffadf7'
            gasUsed: '0x0'
            input: 0x
            output: 0x
```
