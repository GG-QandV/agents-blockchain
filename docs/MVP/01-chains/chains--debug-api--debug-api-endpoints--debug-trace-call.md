# debug_traceCall

> Source: [https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call.md](https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call.md)

# debug_traceCall

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Runs an eth_call within the context of the given block execution using the final state of parent block as the base.

Reference: https://www.alchemy.com/docs/chains/debug-api/debug-api-endpoints/debug-trace-call

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| Transaction Object | object | Yes | The transaction call object. |
| Block identifier | string or enum | Yes | Block hash, block number (in hex), or block tag. |
| Options | object | No | Options for the call including tracer and state overrides. |

## Result

**Call traces** (object[]): Array of call traces.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "debug_traceCall",
  "params": [
    {
      "type": "0x02",
      "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "gas": "0xb59b",
      "value": "0x0",
      "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc",
      "nonce": "0x8eaee",
      "accessList": [],
      "maxPriorityFeePerGas": "0xb2d05e00",
      "maxFeePerGas": "0x1ceb1eec4"
    },
    "0x154bd82",
    {
      "tracer": "callTracer",
      "stateOverrides": {
        "0xdac17f958d2ee523a2206206994597c13d831ec7": {
          "stateDiff": {
            "0x0000000000000000000000000000000000000000000000000000000000000007": "0x000000000000000000000000000000000000000000000000000000746a528800"
          }
        }
      }
    }
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "type": "CALL",
    "from": "0xe088776deabb472ffd2843e330e79c880a5f979e",
    "to": "0x70526cc7a6d6320b44122ea9d2d07670accc85a1",
    "value": "0x0",
    "gas": "0x7fffffffffffadf7",
    "gasUsed": "0x0",
    "input": "0x",
    "output": "0x"
  },
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
  "method": "debug_traceCall",
  "params": [
    {
      "type": "0x02",
      "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "gas": "0xb59b",
      "value": "0x0",
      "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc",
      "nonce": "0x8eaee",
      "accessList": [],
      "maxPriorityFeePerGas": "0xb2d05e00",
      "maxFeePerGas": "0x1ceb1eec4"
    },
    "string",
    {
      "tracer": {
        "tracer": "callTracer",
        "tracerConfig": {
          "onlyTopCall": false
        }
      },
      "stateOverrides": {
        "property1": {
          "balance": "string",
          "nonce": "string",
          "code": "string",
          "state": {
            "property1": "string"
          },
          "stateDiff": {
            "property1": "string"
          },
          "movePrecompileToAddress": "string"
        }
      }
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
    method: 'debug_traceCall',
    params: [
      {
        type: '0x02',
        from: '0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2',
        to: '0xdac17f958d2ee523a2206206994597c13d831ec7',
        gas: '0xb59b',
        value: '0x0',
        data: '0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc',
        nonce: '0x8eaee',
        accessList: [],
        maxPriorityFeePerGas: '0xb2d05e00',
        maxFeePerGas: '0x1ceb1eec4'
      },
      'string',
      {
        tracer: {tracer: 'callTracer', tracerConfig: {onlyTopCall: false}},
        stateOverrides: {
          property1: {
            balance: 'string',
            nonce: 'string',
            code: 'string',
            state: {property1: 'string'},
            stateDiff: {property1: 'string'},
            movePrecompileToAddress: 'string'
          }
        }
      }
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
    "method": "debug_traceCall",
    "params": [
        {
            "type": "0x02",
            "from": "0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2",
            "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
            "gas": "0xb59b",
            "value": "0x0",
            "data": "0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc",
            "nonce": "0x8eaee",
            "accessList": [],
            "maxPriorityFeePerGas": "0xb2d05e00",
            "maxFeePerGas": "0x1ceb1eec4"
        },
        "string",
        {
            "tracer": {
                "tracer": "callTracer",
                "tracerConfig": { "onlyTopCall": False }
            },
            "stateOverrides": { "property1": {
                    "balance": "string",
                    "nonce": "string",
                    "code": "string",
                    "state": { "property1": "string" },
                    "stateDiff": { "property1": "string" },
                    "movePrecompileToAddress": "string"
                } }
        }
    ]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCall\",\n  \"params\": [\n    {\n      \"type\": \"0x02\",\n      \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n      \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n      \"gas\": \"0xb59b\",\n      \"value\": \"0x0\",\n      \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\",\n      \"nonce\": \"0x8eaee\",\n      \"accessList\": [],\n      \"maxPriorityFeePerGas\": \"0xb2d05e00\",\n      \"maxFeePerGas\": \"0x1ceb1eec4\"\n    },\n    \"string\",\n    {\n      \"tracer\": {\n        \"tracer\": \"callTracer\",\n        \"tracerConfig\": {\n          \"onlyTopCall\": false\n        }\n      },\n      \"stateOverrides\": {\n        \"property1\": {\n          \"balance\": \"string\",\n          \"nonce\": \"string\",\n          \"code\": \"string\",\n          \"state\": {\n            \"property1\": \"string\"\n          },\n          \"stateDiff\": {\n            \"property1\": \"string\"\n          },\n          \"movePrecompileToAddress\": \"string\"\n        }\n      }\n    }\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCall\",\n  \"params\": [\n    {\n      \"type\": \"0x02\",\n      \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n      \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n      \"gas\": \"0xb59b\",\n      \"value\": \"0x0\",\n      \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\",\n      \"nonce\": \"0x8eaee\",\n      \"accessList\": [],\n      \"maxPriorityFeePerGas\": \"0xb2d05e00\",\n      \"maxFeePerGas\": \"0x1ceb1eec4\"\n    },\n    \"string\",\n    {\n      \"tracer\": {\n        \"tracer\": \"callTracer\",\n        \"tracerConfig\": {\n          \"onlyTopCall\": false\n        }\n      },\n      \"stateOverrides\": {\n        \"property1\": {\n          \"balance\": \"string\",\n          \"nonce\": \"string\",\n          \"code\": \"string\",\n          \"state\": {\n            \"property1\": \"string\"\n          },\n          \"stateDiff\": {\n            \"property1\": \"string\"\n          },\n          \"movePrecompileToAddress\": \"string\"\n        }\n      }\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"debug_traceCall\",\n  \"params\": [\n    {\n      \"type\": \"0x02\",\n      \"from\": \"0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2\",\n      \"to\": \"0xdac17f958d2ee523a2206206994597c13d831ec7\",\n      \"gas\": \"0xb59b\",\n      \"value\": \"0x0\",\n      \"data\": \"0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc\",\n      \"nonce\": \"0x8eaee\",\n      \"accessList\": [],\n      \"maxPriorityFeePerGas\": \"0xb2d05e00\",\n      \"maxFeePerGas\": \"0x1ceb1eec4\"\n    },\n    \"string\",\n    {\n      \"tracer\": {\n        \"tracer\": \"callTracer\",\n        \"tracerConfig\": {\n          \"onlyTopCall\": false\n        }\n      },\n      \"stateOverrides\": {\n        \"property1\": {\n          \"balance\": \"string\",\n          \"nonce\": \"string\",\n          \"code\": \"string\",\n          \"state\": {\n            \"property1\": \"string\"\n          },\n          \"stateDiff\": {\n            \"property1\": \"string\"\n          },\n          \"movePrecompileToAddress\": \"string\"\n        }\n      }\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: debug_traceCall
description: Runs an eth_call within the context of the given block execution using the final state of parent block as the base.
x-compute-units: 40
x-rate-limit-cus: 1000
params:
  - name: Transaction Object
    required: true
    description: The transaction call object.
    schema:
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
  - name: Block identifier
    required: true
    description: Block hash, block number (in hex), or block tag.
    schema:
      oneOf:
        - title: 32 byte hex value
          type: string
          pattern: ^0x[0-9a-f]{64}$
        - title: hex encoded bytes
          type: string
          pattern: ^0x[0-9a-f]*$
        - title: block tag
          type: string
          enum:
            - latest
            - earliest
            - pending
  - name: Options
    required: false
    description: Options for the call including tracer and state overrides.
    schema:
      type: object
      properties:
        tracer:
          title: Tracer
          type: object
          properties:
            tracer:
              type: string
              enum:
                - callTracer
                - prestateTracer
            tracerConfig:
              type: object
              properties:
                onlyTopCall:
                  type: boolean
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
                description: Optional. Move the precompile residing at this account's address to the address specified here.
                title: hex encoded address
                type: string
                pattern: ^0x[0-9a-fA-F]{40}$
result:
  name: Call traces
  description: Array of call traces.
  schema:
    type: array
    description: Array of block traces.
    items:
      type: object
      description: Array of block traces.
      properties:
        type:
          type: string
          description: CALL or CREATE
        from:
          type: string
          description: 20-byte address of the caller
        to:
          type: string
          description: 20-byte address of the recipient. Null when its a contract creation transaction.
        value:
          type: string
          description: Amount of value included in the transfer (in hex)
        gas:
          type: string
          description: Amount of gas provided for the call (in hex)
        gasUsed:
          type: string
          description: Amount of gas used during the call (in hex)
        input:
          type: string
          description: Call data
        output:
          type: string
          description: Return data
        error:
          type: string
          description: Error message, if any.
        revertReason:
          type: string
          description: Solidity revert reason, if any.
        calls:
          type: array
          description: Array of sub-calls made within the transaction.
          items:
            type: object
            description: Array of block traces.
            properties:
              type:
                type: string
                description: CALL or CREATE
              from:
                type: string
                description: 20-byte address of the caller
              to:
                type: string
                description: 20-byte address of the recipient. Null when its a contract creation transaction.
              value:
                type: string
                description: Amount of value included in the transfer (in hex)
              gas:
                type: string
                description: Amount of gas provided for the call (in hex)
              gasUsed:
                type: string
                description: Amount of gas used during the call (in hex)
              input:
                type: string
                description: Call data
              output:
                type: string
                description: Return data
              error:
                type: string
                description: Error message, if any.
              revertReason:
                type: string
                description: Solidity revert reason, if any.
examples:
  - name: debug_traceCall example
    params:
      - name: Transaction Object
        value:
          type: '0x02'
          from: '0xcdd37ada79f589c15bd4f8fd2083dc88e34a2af2'
          to: '0xdac17f958d2ee523a2206206994597c13d831ec7'
          gas: '0xb59b'
          value: '0x0'
          data: '0xa9059cbb000000000000000000000000b29d1cef6da3262df35fdee71a176c62687e747d000000000000000000000000000000000000000000000000000000001e3f0ccc'
          nonce: '0x8eaee'
          accessList: []
          maxPriorityFeePerGas: '0xb2d05e00'
          maxFeePerGas: '0x1ceb1eec4'
      - name: Block identifier
        value: '0x154bd82'
      - name: Options
        value:
          tracer: callTracer
          stateOverrides:
            '0xdac17f958d2ee523a2206206994597c13d831ec7':
              stateDiff:
                '0x0000000000000000000000000000000000000000000000000000000000000007': '0x000000000000000000000000000000000000000000000000000000746a528800'
    result:
      name: debug_traceCall response
      value:
        type: CALL
        from: '0xe088776deabb472ffd2843e330e79c880a5f979e'
        to: '0x70526cc7a6d6320b44122ea9d2d07670accc85a1'
        value: '0x0'
        gas: '0x7fffffffffffadf7'
        gasUsed: '0x0'
        input: 0x
        output: 0x
```
