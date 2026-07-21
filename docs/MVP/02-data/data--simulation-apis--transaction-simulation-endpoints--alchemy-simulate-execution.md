# alchemy_simulateExecution

> Source: [https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution.md](https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution.md)

# alchemy_simulateExecution

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Simulates a single transaction's execution and returns decoded traces/logs.

Reference: https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| format | enum | Yes | Specifies whether to return a 'NESTED' or 'FLAT' call structure. |
| transaction | object | Yes | Transaction object to simulate. |
| blockTag | enum | Yes | The block context to use: 'latest', 'safe', 'finalized', or 'earliest'. |

## Result

**Execution simulation** (object): The decoded execution traces and logs produced by simulating the transaction.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_simulateExecution",
  "params": [
    "FLAT",
    {
      "from": "0x0dd7eef07982749410eceaa721cdc8ff3167fc9b",
      "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
      "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000",
      "gas": "0x22710",
      "gasPrice": "0x3b9aca00"
    },
    "latest"
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "calls": [
      {
        "type": "CALL",
        "from": "0x0dd7eef07982749410eceaa721cdc8ff3167fc9b",
        "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
        "value": "0x0",
        "gas": "0x22710",
        "gasUsed": "0x1ab62",
        "input": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000",
        "output": "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "decoded": {
          "authority": "ETHERSCAN",
          "methodName": "multicall",
          "inputs": [
            {
              "name": "deadline",
              "type": "uint256",
              "value": "1666117679"
            },
            {
              "name": "data",
              "type": "bytes[]",
              "value": "0x04e45aaf000000000000000000000000..."
            }
          ],
          "outputs": [
            {
              "name": "",
              "type": "bytes[]",
              "value": "0x000000000000000000000000000000000000000000..."
            }
          ]
        }
      }
    ],
    "logs": [
      {
        "address": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "data": "0x0000000000000000000000000000000000000000000000001bc097e5badd8f46",
        "topics": [
          "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
          "0xdd7eef07982749410eceaa721cdc8ff3167fc9b"
        ],
        "decoded": {
          "authority": "ETHERSCAN",
          "eventName": "Transfer",
          "inputs": [
            {
              "name": "src",
              "type": "address",
              "value": "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
              "indexed": true
            },
            {
              "name": "dst",
              "type": "address",
              "value": "0x0dd7eef07982749410eceaa721cdc8ff3167fc9b",
              "indexed": true
            },
            {
              "name": "wad",
              "type": "uint256",
              "value": "1999765247490887494",
              "indexed": false
            }
          ]
        }
      }
    ],
    "error": "",
    "revertReason": null
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
  "method": "alchemy_simulateExecution",
  "params": [
    "FLAT",
    {
      "from": "0x0dd7eef07982749410eceaa721cdc8ff3167fc9b",
      "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
      "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000",
      "gas": "0x22710",
      "gasPrice": "0x3b9aca00"
    },
    "latest"
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
    method: 'alchemy_simulateExecution',
    params: [
      'FLAT',
      {
        from: '0x0dd7eef07982749410eceaa721cdc8ff3167fc9b',
        to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45',
        data: '0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000',
        gas: '0x22710',
        gasPrice: '0x3b9aca00'
      },
      'latest'
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
    "method": "alchemy_simulateExecution",
    "params": [
        "FLAT",
        {
            "from": "0x0dd7eef07982749410eceaa721cdc8ff3167fc9b",
            "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
            "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000",
            "gas": "0x22710",
            "gasPrice": "0x3b9aca00"
        },
        "latest"
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecution\",\n  \"params\": [\n    \"FLAT\",\n    {\n      \"from\": \"0x0dd7eef07982749410eceaa721cdc8ff3167fc9b\",\n      \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n      \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\",\n      \"gas\": \"0x22710\",\n      \"gasPrice\": \"0x3b9aca00\"\n    },\n    \"latest\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecution\",\n  \"params\": [\n    \"FLAT\",\n    {\n      \"from\": \"0x0dd7eef07982749410eceaa721cdc8ff3167fc9b\",\n      \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n      \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\",\n      \"gas\": \"0x22710\",\n      \"gasPrice\": \"0x3b9aca00\"\n    },\n    \"latest\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecution\",\n  \"params\": [\n    \"FLAT\",\n    {\n      \"from\": \"0x0dd7eef07982749410eceaa721cdc8ff3167fc9b\",\n      \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n      \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\",\n      \"gas\": \"0x22710\",\n      \"gasPrice\": \"0x3b9aca00\"\n    },\n    \"latest\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_simulateExecution
description: Simulates a single transaction's execution and returns decoded traces/logs.
x-compute-units: 2500
params:
  - name: format
    required: true
    description: Specifies whether to return a 'NESTED' or 'FLAT' call structure.
    schema:
      type: string
      enum:
        - NESTED
        - FLAT
  - name: transaction
    required: true
    description: Transaction object to simulate.
    schema:
      type: object
      required:
        - to
      properties:
        from:
          title: hex encoded address
          type: string
          pattern: ^0x[0-9a-fA-F]{40}$
        to:
          title: hex encoded address
          type: string
          pattern: ^0x[0-9a-fA-F]{40}$
        data:
          title: hex encoded bytes (any case)
          type: string
          pattern: ^0x[0-9a-fA-F]*$
        gas:
          title: hex encoded unsigned integer
          type: string
          pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
        gasPrice:
          title: hex encoded unsigned integer
          type: string
          pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
        maxFeePerGas:
          title: hex encoded unsigned integer
          type: string
          pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
        maxPriorityFeePerGas:
          title: hex encoded unsigned integer
          type: string
          pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
  - name: blockTag
    required: true
    description: 'The block context to use: ''latest'', ''safe'', ''finalized'', or ''earliest''.'
    schema:
      type: string
      enum:
        - latest
        - safe
        - finalized
        - earliest
result:
  name: Execution simulation
  description: The decoded execution traces and logs produced by simulating the transaction.
  schema:
    title: SimulateExecutionResult
    type: object
    required:
      - calls
      - logs
      - error
    properties:
      calls:
        type: array
        items:
          title: Call
          type: object
          required:
            - type
            - from
            - to
            - gas
            - gasUsed
            - input
            - output
            - decoded
          properties:
            type:
              title: CallType
              type: string
              enum:
                - CALL
                - STATICCALL
                - DELEGATECALL
            from:
              title: hex encoded address
              type: string
              pattern: ^0x[0-9a-fA-F]{40}$
              description: Address the transaction is sent from
            to:
              title: hex encoded address
              type: string
              pattern: ^0x[0-9a-fA-F]{40}$
              description: Address the transaction is directed to
            value:
              description: Amount in wei to transfer from sender to recipient
              anyOf:
                - title: hex encoded unsigned integer
                  type: string
                  pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
                - type: 'null'
            gas:
              title: hex encoded unsigned integer
              type: string
              pattern: ^0x([1-9a-f]+[0-9a-f]*|0)$
              description: Hex-encoded gas limit
            gasUsed:
              title: hex encoded bytes (any case)
              type: string
              pattern: ^0x[0-9a-fA-F]*$
              description: Hex-encoded gas actually used by this call
            input:
              title: hex encoded bytes (any case)
              type: string
              pattern: ^0x[0-9a-fA-F]*$
              description: Hex-encoded call data
            output:
              type: string
              description: Hex-encoded return data
            decoded:
              anyOf:
                - title: DecodedCall
                  type: object
                  required:
                    - methodName
                    - inputs
                    - outputs
                    - authority
                  properties:
                    methodName:
                      type: string
                    inputs:
                      type: array
                      items:
                        title: DecodedCallParam
                        type: object
                        required:
                          - name
                          - type
                          - value
                        properties:
                          name:
                            type: string
                          type:
                            type: string
                          value:
                            anyOf:
                              - title: hex encoded bytes (any case)
                                type: string
                                pattern: ^0x[0-9a-fA-F]*$
                              - type: string
                            description: The decoded field value
                    outputs:
                      type: array
                      items:
                        title: DecodedCallParam
                        type: object
                        required:
                          - name
                          - type
                          - value
                        properties:
                          name:
                            type: string
                          type:
                            type: string
                          value:
                            anyOf:
                              - title: hex encoded bytes (any case)
                                type: string
                                pattern: ^0x[0-9a-fA-F]*$
                              - type: string
                            description: The decoded field value
                    authority:
                      title: DecodingAuthority
                      type: string
                      enum:
                        - ETHERSCAN
                - type: 'null'
      logs:
        type: array
        items:
          title: Log
          type: object
          required:
            - topics
            - address
            - data
            - decoded
          properties:
            topics:
              type: array
              items:
                title: hex encoded bytes (any case)
                type: string
                pattern: ^0x[0-9a-fA-F]*$
            address:
              title: hex encoded address
              type: string
              pattern: ^0x[0-9a-fA-F]{40}$
            data:
              title: hex encoded bytes (any case)
              type: string
              pattern: ^0x[0-9a-fA-F]*$
            decoded:
              anyOf:
                - title: DecodedLog
                  type: object
                  required:
                    - eventName
                    - inputs
                    - authority
                  properties:
                    eventName:
                      type: string
                    inputs:
                      type: array
                      items:
                        title: DecodedLogInput
                        type: object
                        required:
                          - name
                          - type
                          - value
                          - indexed
                        properties:
                          name:
                            type: string
                          type:
                            type: string
                          value:
                            anyOf:
                              - title: hex encoded bytes (any case)
                                type: string
                                pattern: ^0x[0-9a-fA-F]*$
                              - type: string
                          indexed:
                            type: boolean
                    authority:
                      title: DecodingAuthority
                      type: string
                      enum:
                        - ETHERSCAN
                - type: 'null'
      error:
        type:
          - string
          - 'null'
        description: If the transaction would revert, error explains the reason
      revertReason:
        type: string
        nullable: true
        description: The reason why a transaction would revert. Provides details about potential transaction failure before execution.
examples:
  - name: Uniswap swap example
    params:
      - name: format
        value: FLAT
      - name: transaction
        value:
          from: '0x0dd7eef07982749410eceaa721cdc8ff3167fc9b'
          to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45'
          data: '0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000'
          gas: '0x22710'
          gasPrice: '0x3b9aca00'
      - name: blockTag
        value: latest
    result:
      name: Execution simulation
      value:
        calls:
          - type: CALL
            from: '0x0dd7eef07982749410eceaa721cdc8ff3167fc9b'
            to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45'
            value: '0x0'
            gas: '0x22710'
            gasUsed: '0x1ab62'
            input: '0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000'
            output: '0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'
            decoded:
              authority: ETHERSCAN
              methodName: multicall
              inputs:
                - name: deadline
                  type: uint256
                  value: '1666117679'
                - name: data
                  type: bytes[]
                  value: 0x04e45aaf000000000000000000000000...
              outputs:
                - name: ''
                  type: bytes[]
                  value: 0x000000000000000000000000000000000000000000...
        logs:
          - address: '0x6b175474e89094c44da98b954eedeac495271d0f'
            data: '0x0000000000000000000000000000000000000000000000001bc097e5badd8f46'
            topics:
              - '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef'
              - '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406'
              - '0xdd7eef07982749410eceaa721cdc8ff3167fc9b'
            decoded:
              authority: ETHERSCAN
              eventName: Transfer
              inputs:
                - name: src
                  type: address
                  value: '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406'
                  indexed: true
                - name: dst
                  type: address
                  value: '0x0dd7eef07982749410eceaa721cdc8ff3167fc9b'
                  indexed: true
                - name: wad
                  type: uint256
                  value: '1999765247490887494'
                  indexed: false
        error: ''
        revertReason: null
```
