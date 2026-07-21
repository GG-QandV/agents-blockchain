# alchemy_getTokenBalances

> Source: [https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-balances.md](https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-balances.md)

# alchemy_getTokenBalances

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Returns ERC-20 token balances for a given address.

Reference: https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-balances

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string | Yes | A 20-byte wallet address.  |
| tokenSpec | enum or string[] | No | A token specification: - The string "erc20" - "NATIVE_TOKEN"  - "DEFAULT_TOKENS" (deprecated) - An array of token contract addresses.  |
| options | object | No | Optional pagination options.  |

## Result

**Token Balances** (object): An object containing the queried address and an array of token balance objects.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_getTokenBalances",
  "params": [
    "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
    [
      "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
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
    "address": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
    "tokenBalances": [
      {
        "contractAddress": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "tokenBalance": "0x0000000000000000000000000000000000000000000000000000000000000000"
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
  "method": "alchemy_getTokenBalances",
  "params": [
    "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
    [
      "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
    ],
    {
      "pageKey": "string",
      "maxCount": 100
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
    method: 'alchemy_getTokenBalances',
    params: [
      '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045',
      ['0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'],
      {pageKey: 'string', maxCount: 100}
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
    "method": "alchemy_getTokenBalances",
    "params": [
        "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
        ["0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"],
        {
            "pageKey": "string",
            "maxCount": 100
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenBalances\",\n  \"params\": [\n    \"0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045\",\n    [\n      \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    ],\n    {\n      \"pageKey\": \"string\",\n      \"maxCount\": 100\n    }\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenBalances\",\n  \"params\": [\n    \"0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045\",\n    [\n      \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    ],\n    {\n      \"pageKey\": \"string\",\n      \"maxCount\": 100\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenBalances\",\n  \"params\": [\n    \"0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045\",\n    [\n      \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n    ],\n    {\n      \"pageKey\": \"string\",\n      \"maxCount\": 100\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_getTokenBalances
description: Returns ERC-20 token balances for a given address.
x-compute-units: 20
params:
  - name: address
    required: true
    description: |
      A 20-byte wallet address.
    schema:
      type: string
      title: Address
      default: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
  - name: tokenSpec
    required: false
    description: |
      A token specification: - The string "erc20" - "NATIVE_TOKEN"  - "DEFAULT_TOKENS" (deprecated) - An array of token contract addresses.
    schema:
      oneOf:
        - type: string
          title: Token Specification
          enum:
            - erc20
            - DEFAULT_TOKENS
            - NATIVE_TOKEN
          default: erc20
        - type: array
          title: Array of Contract Addresses
          items:
            type: string
            pattern: ^0[xX][0-9a-fA-F]{40}$
  - name: options
    required: false
    description: |
      Optional pagination options.
    schema:
      type: object
      title: Options
      properties:
        pageKey:
          type: string
          description: Used for pagination if more results are available.
        maxCount:
          type: integer
          description: Maximum number of token balances to return per call (capped at 100).
          default: 100
result:
  name: Token Balances
  description: An object containing the queried address and an array of token balance objects.
  schema:
    type: object
    properties:
      address:
        description: Address for which token balances were returned.
        title: hex encoded address
        type: string
        pattern: ^0x[0-9a-fA-F]{40}$
      tokenBalances:
        type: array
        description: Array of token balance objects. Exactly one of tokenBalance or error is non-null.
        items:
          type: object
          properties:
            contractAddress:
              description: The ERC-20 contract address.
              title: hex encoded address
              type: string
              pattern: ^0x[0-9a-fA-F]{40}$
            tokenBalance:
              type: string
              description: Hex-encoded string of the token balance, or null if error is present.
examples:
  - name: Single token balance example
    params:
      - name: address
        value: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
      - name: tokenSpec
        value:
          - '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
    result:
      name: Token Balances
      value:
        address: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
        tokenBalances:
          - contractAddress: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
            tokenBalance: '0x0000000000000000000000000000000000000000000000000000000000000000'
```
