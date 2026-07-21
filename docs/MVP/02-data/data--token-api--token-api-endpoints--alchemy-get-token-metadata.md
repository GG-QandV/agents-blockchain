# alchemy_getTokenMetadata

> Source: [https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-metadata.md](https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-metadata.md)

# alchemy_getTokenMetadata

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

Returns metadata for a given token contract (name, symbol, decimals, logo).

Reference: https://www.alchemy.com/docs/data/token-api/token-api-endpoints/alchemy-get-token-metadata

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | A single 20-byte token contract address. |

## Result

**Token Metadata** (object): Object with name, symbol, decimals, and an optional logo URL.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_getTokenMetadata",
  "params": [
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "name": "USD Coin",
    "symbol": "USDC",
    "decimals": 6,
    "logo": "https://static.alchemyapi.io/images/assets/3408.png"
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
  "method": "alchemy_getTokenMetadata",
  "params": [
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
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
    method: 'alchemy_getTokenMetadata',
    params: ['0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48']
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
    "method": "alchemy_getTokenMetadata",
    "params": ["0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenMetadata\",\n  \"params\": [\n    \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenMetadata\",\n  \"params\": [\n    \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getTokenMetadata\",\n  \"params\": [\n    \"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_getTokenMetadata
description: Returns metadata for a given token contract (name, symbol, decimals, logo).
x-compute-units: 10
params:
  - name: contractAddress
    required: true
    description: A single 20-byte token contract address.
    schema:
      title: hex encoded address
      type: string
      pattern: ^0x[0-9a-fA-F]{40}$
result:
  name: Token Metadata
  description: Object with name, symbol, decimals, and an optional logo URL.
  schema:
    type: object
    properties:
      name:
        type: string
        description: Token's name, or null if not found.
      symbol:
        type: string
        description: Token's symbol, or null if not found.
      decimals:
        type: number
        description: Number of decimals the token uses, or null if not found.
      logo:
        type: string
        description: URL of the token's logo image, or null if none available.
examples:
  - name: USDC metadata example
    params:
      - name: contractAddress
        value: '0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'
    result:
      name: Token Metadata
      value:
        name: USD Coin
        symbol: USDC
        decimals: 6
        logo: https://static.alchemyapi.io/images/assets/3408.png
```
