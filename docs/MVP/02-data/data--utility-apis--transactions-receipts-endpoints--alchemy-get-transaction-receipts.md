# alchemy_getTransactionReceipts

> Source: [https://www.alchemy.com/docs/data/utility-apis/transactions-receipts-endpoints/alchemy-get-transaction-receipts.md](https://www.alchemy.com/docs/data/utility-apis/transactions-receipts-endpoints/alchemy-get-transaction-receipts.md)

# alchemy_getTransactionReceipts

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

An enhanced API that gets all transaction receipts for a given block by number or block hash.

Reference: https://www.alchemy.com/docs/data/utility-apis/transactions-receipts-endpoints/alchemy-get-transaction-receipts

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| blockNumber | string | No | The block number you want to get transaction receipts for, in hex. One of blockNumber or blockHash must be provided. |
| blockHash | string | No | The block hash you want to get transaction receipts for, in hex. One of blockNumber or blockHash must be provided. |

## Result

**Transaction receipts** (string or object): A list of transaction receipt objects for each transaction in this block.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_getTransactionReceipts",
  "params": [
    {
      "blockNumber": "0xF1D1C6"
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
    "receipts": [
      {
        "blockHash": "0x18760312114f3fdf11f9d5846245995835aa59994d5fc4203faee52d2f7eaabe",
        "blockNumber": "0xF1D1C6",
        "transactionIndex": "0x0",
        "transactionHash": "0x4c32db860a0b1fd4514b94c2c75a76d4924cebaebd62d2f622105e0fc863cfb2",
        "from": "0x0049d82d5a59bbdb24fe2e8eb4733ecf20df1886",
        "to": "0x66c4289252b6f1d8d7b241495e60f93c2820c9b1",
        "cumulativeGasUsed": "0x5208",
        "gasUsed": "0x5208",
        "contractAddress": null,
        "logs": [],
        "logsBloom": "0x0...",
        "root": null,
        "status": 1,
        "effectiveGasPrice": "0x1e8480",
        "type": "0x2"
      }
    ]
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
  "method": "alchemy_getTransactionReceipts",
  "params": [
    "string",
    "string"
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
    method: 'alchemy_getTransactionReceipts',
    params: ['string', 'string']
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
    "method": "alchemy_getTransactionReceipts",
    "params": ["string", "string"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTransactionReceipts\",\n  \"params\": [\n    \"string\",\n    \"string\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTransactionReceipts\",\n  \"params\": [\n    \"string\",\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTransactionReceipts\",\n  \"params\": [\n    \"string\",\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_getTransactionReceipts
description: An enhanced API that gets all transaction receipts for a given block by number or block hash.
x-compute-units: 250
params:
  - name: blockNumber
    required: false
    description: The block number you want to get transaction receipts for, in hex. One of blockNumber or blockHash must be provided.
    schema:
      title: hex encoded unsigned integer (any case)
      type: string
      pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
  - name: blockHash
    required: false
    description: The block hash you want to get transaction receipts for, in hex. One of blockNumber or blockHash must be provided.
    schema:
      title: 32 byte hex value (any case)
      type: string
      pattern: ^0[xX][0-9A-Fa-f]{64}$
result:
  name: Transaction receipts
  description: A list of transaction receipt objects for each transaction in this block.
  schema:
    oneOf:
      - title: Not Found (null)
        type: string
      - type: object
        properties:
          receipts:
            type: array
            description: Array of transaction receipt objects.
            items:
              type: object
              properties:
                blockHash:
                  title: 32 byte hex value (any case)
                  type: string
                  pattern: ^0[xX][0-9A-Fa-f]{64}$
                blockNumber:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                transactionIndex:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                transactionHash:
                  title: 32 byte hex value (any case)
                  type: string
                  pattern: ^0[xX][0-9A-Fa-f]{64}$
                from:
                  title: hex encoded address
                  type: string
                  pattern: ^0x[0-9a-fA-F]{40}$
                to:
                  title: hex encoded address
                  type: string
                  pattern: ^0x[0-9a-fA-F]{40}$
                cumulativeGasUsed:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                gasUsed:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                contractAddress:
                  title: hex encoded address
                  type: string
                  pattern: ^0x[0-9a-fA-F]{40}$
                logs:
                  type: array
                  items:
                    type: object
                    properties:
                      blockHash:
                        title: 32 byte hex value (any case)
                        type: string
                        pattern: ^0[xX][0-9A-Fa-f]{64}$
                      blockNumber:
                        title: hex encoded unsigned integer (any case)
                        type: string
                        pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                      transactionIndex:
                        title: hex encoded unsigned integer (any case)
                        type: string
                        pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                      address:
                        title: hex encoded address
                        type: string
                        pattern: ^0x[0-9a-fA-F]{40}$
                      logIndex:
                        title: hex encoded unsigned integer (any case)
                        type: string
                        pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                      data:
                        type: string
                      removed:
                        type: boolean
                      topics:
                        type: array
                        description: Zero to four 32-byte topics, any hex case.
                        items:
                          title: 32 byte hex value (any case)
                          type: string
                          pattern: ^0[xX][0-9A-Fa-f]{64}$
                      transactionHash:
                        title: 32 byte hex value (any case)
                        type: string
                        pattern: ^0[xX][0-9A-Fa-f]{64}$
                logsBloom:
                  title: 256 hex encoded bytes (any case)
                  type: string
                  pattern: ^0x[0-9a-fA-F]{512}$
                root:
                  title: 32 byte hex value (any case)
                  type: string
                  pattern: ^0[xX][0-9A-Fa-f]{64}$
                status:
                  type: integer
                  enum:
                    - 0
                    - 1
                  description: Transaction status; 0 = failure, 1 = success.
                effectiveGasPrice:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
                type:
                  title: hex encoded unsigned integer (any case)
                  type: string
                  pattern: ^0[xX]([1-9A-Fa-f]+[0-9A-Fa-f]*|0)$
examples:
  - name: alchemy_getTransactionReceipts example (by blockNumber)
    params:
      - name: blockNumber
        value:
          blockNumber: '0xF1D1C6'
    result:
      name: Transaction receipts
      value:
        receipts:
          - blockHash: '0x18760312114f3fdf11f9d5846245995835aa59994d5fc4203faee52d2f7eaabe'
            blockNumber: '0xF1D1C6'
            transactionIndex: '0x0'
            transactionHash: '0x4c32db860a0b1fd4514b94c2c75a76d4924cebaebd62d2f622105e0fc863cfb2'
            from: '0x0049d82d5a59bbdb24fe2e8eb4733ecf20df1886'
            to: '0x66c4289252b6f1d8d7b241495e60f93c2820c9b1'
            cumulativeGasUsed: '0x5208'
            gasUsed: '0x5208'
            contractAddress: null
            logs: []
            logsBloom: 0x0...
            root: null
            status: 1
            effectiveGasPrice: '0x1e8480'
            type: '0x2'
```
