# alchemy_simulateAssetChangesBundle

> Source: [https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-asset-changes-bundle.md](https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-asset-changes-bundle.md)

# alchemy_simulateAssetChangesBundle

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Simulates multiple transactions sequentially and returns a combined list of asset changes.

Reference: https://www.alchemy.com/docs/data/simulation-apis/transaction-simulation-endpoints/alchemy-simulate-asset-changes-bundle

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactions | object[] | Yes | An array of 1–2 transaction objects to run in sequence. |

## Result

**Simulation result** (object[]): The asset changes resulting from the sequential simulation.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_simulateAssetChangesBundle",
  "params": [
    [
      {
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "data": "0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240"
      },
      {
        "from": "0x1234567890abcdef1234567890abcdef12345678",
        "to": "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
        "data": "0x00abcdef"
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
    "changes": [
      {
        "assetType": "ERC20",
        "changeType": "TRANSFER",
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
        "rawAmount": "1000000",
        "contractAddress": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "tokenId": null,
        "decimals": 6,
        "symbol": "USDC",
        "name": "USDC",
        "logo": "https://static.alchemyapi.io/images/assets/3408.png",
        "amount": "1"
      }
    ],
    "gasUsed": "0xb05c",
    "error": null
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
  "method": "alchemy_simulateAssetChangesBundle",
  "params": [
    [
      {
        "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
        "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "data": "0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240"
      },
      {
        "from": "0x1234567890abcdef1234567890abcdef12345678",
        "to": "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
        "data": "0x00abcdef"
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
    method: 'alchemy_simulateAssetChangesBundle',
    params: [
      [
        {
          from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045',
          to: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48',
          data: '0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240'
        },
        {
          from: '0x1234567890abcdef1234567890abcdef12345678',
          to: '0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de',
          data: '0x00abcdef'
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
    "method": "alchemy_simulateAssetChangesBundle",
    "params": [[
            {
                "from": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045",
                "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                "data": "0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240"
            },
            {
                "from": "0x1234567890abcdef1234567890abcdef12345678",
                "to": "0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de",
                "data": "0x00abcdef"
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateAssetChangesBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\",\n        \"data\": \"0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240\"\n      },\n      {\n        \"from\": \"0x1234567890abcdef1234567890abcdef12345678\",\n        \"to\": \"0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de\",\n        \"data\": \"0x00abcdef\"\n      }\n    ]\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateAssetChangesBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\",\n        \"data\": \"0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240\"\n      },\n      {\n        \"from\": \"0x1234567890abcdef1234567890abcdef12345678\",\n        \"to\": \"0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de\",\n        \"data\": \"0x00abcdef\"\n      }\n    ]\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_simulateAssetChangesBundle\",\n  \"params\": [\n    [\n      {\n        \"from\": \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\",\n        \"to\": \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\",\n        \"data\": \"0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240\"\n      },\n      {\n        \"from\": \"0x1234567890abcdef1234567890abcdef12345678\",\n        \"to\": \"0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de\",\n        \"data\": \"0x00abcdef\"\n      }\n    ]\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_simulateAssetChangesBundle
description: Simulates multiple transactions sequentially and returns a combined list of asset changes.
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
  name: Simulation result
  description: The asset changes resulting from the sequential simulation.
  schema:
    type: array
    items:
      type: object
      properties:
        changes:
          type: array
          items:
            type: object
            properties:
              assetType:
                type: string
              changeType:
                type: string
              from:
                type: string
              to:
                type: string
              rawAmount:
                type: string
              contractAddress:
                type: string
              tokenId:
                type:
                  - string
                  - 'null'
              decimals:
                type: integer
              symbol:
                type: string
              name:
                type: string
              logo:
                type: string
              amount:
                type: string
examples:
  - name: Bundle simulation example
    params:
      - name: transactions
        value:
          - from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045'
            to: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
            data: '0xa9059cbb000000000000000000000000fc43f5f9dd45258b3aff31bdbe6561d97e8b71de00000000000000000000000000000000000000000000000000000000000f4240'
          - from: '0x1234567890abcdef1234567890abcdef12345678'
            to: '0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de'
            data: '0x00abcdef'
    result:
      name: Simulation result
      value:
        changes:
          - assetType: ERC20
            changeType: TRANSFER
            from: '0xd8da6bf26964af9d7eed9e03e53415d37aa96045'
            to: '0xfc43f5f9dd45258b3aff31bdbe6561d97e8b71de'
            rawAmount: '1000000'
            contractAddress: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
            tokenId: null
            decimals: 6
            symbol: USDC
            name: USDC
            logo: https://static.alchemyapi.io/images/assets/3408.png
            amount: '1'
        gasUsed: '0xb05c'
        error: null
```
