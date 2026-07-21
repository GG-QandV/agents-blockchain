# alchemy_simulateExecutionBundle

> Source: [https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution-bundle.md](https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution-bundle.md)

# alchemy_simulateExecutionBundle

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Simulates multiple transactions sequentially and returns decoded execution traces and logs.

Reference: https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-execution-bundle

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactions | object[] | Yes | An array of 1–2 transaction objects to run in sequence. |

## Result

**Execution simulation** (object[]): The decoded execution traces and logs from simulating multiple transactions in a bundle.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_simulateExecutionBundle",
  "params": [
    [
      {
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
        "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000"
      },
      {
        "from": "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
        "to": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "data": "0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000"
      }
    ]
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
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
        "value": "0x0",
        "gas": "0x80000000000000c0",
        "gasUsed": "0x55a0",
        "input": "0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000",
        "error": "execution reverted"
      },
      {
        "type": "CALL",
        "from": "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
        "to": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "value": "0x0",
        "gas": "0x80000000000000c0",
        "gasUsed": "0x8792",
        "input": "0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000",
        "output": "0x0000000000000000000000000000000000000000000000000000000000000001"
      }
    ],
    "logs": [
      {
        "address": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "data": "0x0000000000000000000000000000000000000000000000000de0b6b3a7640000",
        "topics": [
          "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
          "0xdd7eef07982749410eceaa721cdc8ff3167fc9b"
        ]
      }
    ],
    "error": null,
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
  "method": "alchemy_simulateExecutionBundle",
  "params": [
    [
      {
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
        "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000"
      },
      {
        "from": "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
        "to": "0x6b175474e89094c44da98b954eedeac495271d0f",
        "data": "0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000"
      }
    ]
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
    method: 'alchemy_simulateExecutionBundle',
    params: [
      [
        {
          from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045',
          to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45',
          data: '0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000'
        },
        {
          from: '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406',
          to: '0x6b175474e89094c44da98b954eedeac495271d0f',
          data: '0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000'
        }
      ]
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
    "method": "alchemy_simulateExecutionBundle",
    "params": [[
            {
                "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
                "to": "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
                "data": "0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000"
            },
            {
                "from": "0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406",
                "to": "0x6b175474e89094c44da98b954eedeac495271d0f",
                "data": "0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000"
            }
        ]]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecutionBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n        \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\"\n      },\n      {\n        \"from\": \"0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406\",\n        \"to\": \"0x6b175474e89094c44da98b954eedeac495271d0f\",\n        \"data\": \"0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000\"\n      }\n    ]\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecutionBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n        \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\"\n      },\n      {\n        \"from\": \"0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406\",\n        \"to\": \"0x6b175474e89094c44da98b954eedeac495271d0f\",\n        \"data\": \"0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000\"\n      }\n    ]\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateExecutionBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45\",\n        \"data\": \"0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000\"\n      },\n      {\n        \"from\": \"0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406\",\n        \"to\": \"0x6b175474e89094c44da98b954eedeac495271d0f\",\n        \"data\": \"0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000\"\n      }\n    ]\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_simulateExecutionBundle
description: Simulates multiple transactions sequentially and returns decoded execution traces and logs.
x-compute-units: 4500
params:
  - name: transactions
    required: true
    description: An array of 1–2 transaction objects to run in sequence.
    schema:
      type: array
      minItems: 1
      maxItems: 2
      items:
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
result:
  name: Execution simulation
  description: The decoded execution traces and logs from simulating multiple transactions in a bundle.
  schema:
    type: array
    items:
      type: object
      properties:
        calls:
          type: array
          items:
            type: object
            properties:
              type:
                type: string
              from:
                type: string
              to:
                type: string
              value:
                type: string
              gas:
                type: string
              gasUsed:
                type: string
              input:
                type: string
              output:
                type: string
              error:
                type: string
        logs:
          type: array
          items:
            type: object
            properties:
              address:
                type: string
              data:
                type: string
              topics:
                type: array
                items:
                  type: string
examples:
  - name: Bundle simulation example
    params:
      - name: transactions
        value:
          - from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045'
            to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45'
            data: '0x5ae401dc00000000000000000000000000000000000000000000000000000000634ef02f0000000000000000000000000000000000000000000000000000000000000000'
          - from: '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406'
            to: '0x6b175474e89094c44da98b954eedeac495271d0f'
            data: '0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000'
    result:
      name: Execution simulation
      value:
        calls:
          - type: CALL
            from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045'
            to: '0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45'
            value: '0x0'
            gas: '0x80000000000000c0'
            gasUsed: '0x55a0'
            input: '0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000'
            error: execution reverted
          - type: CALL
            from: '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406'
            to: '0x6b175474e89094c44da98b954eedeac495271d0f'
            value: '0x0'
            gas: '0x80000000000000c0'
            gasUsed: '0x8792'
            input: '0xa9059cbb0000000000000000000000000dd7eef07982749410eceaa721cdc8ff3167fc9b0000000000000000000000000000000000000000000000000de0b6b3a7640000'
            output: '0x0000000000000000000000000000000000000000000000000000000000000001'
        logs:
          - address: '0x6b175474e89094c44da98b954eedeac495271d0f'
            data: '0x0000000000000000000000000000000000000000000000000de0b6b3a7640000'
            topics:
              - '0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef'
              - '0x48da0965ab2d2cbf1c17c09cfb5cbe67ad5b1406'
              - '0xdd7eef07982749410eceaa721cdc8ff3167fc9b'
        error: null
        revertReason: null
```
