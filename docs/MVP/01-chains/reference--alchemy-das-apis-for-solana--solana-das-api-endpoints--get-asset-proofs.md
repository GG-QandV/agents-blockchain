# getAssetProofs

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proofs.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proofs.md)

# getAssetProofs

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns merkle proofs for multiple compressed digital assets.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proofs

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ids | string[] | Yes | An array of compressed asset IDs. |

## Result

**Asset proofs** (object[]): Returns merkle proofs for the specified compressed assets.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getAssetProofs",
  "params": [
    [
      "Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss",
      "8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo"
    ]
  ],
  "id": 1
}
```

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://solana-mainnet.g.alchemy.com/v2/docs-demo \
  --header 'Content-Type: application/json' \
  --data '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getAssetProofs",
  "params": [
    [
      "Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss",
      "8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo"
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
    method: 'getAssetProofs',
    params: [
      [
        'Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss',
        '8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo'
      ]
    ]
  })
};

fetch('https://solana-mainnet.g.alchemy.com/v2/docs-demo', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://solana-mainnet.g.alchemy.com/v2/docs-demo"

payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getAssetProofs",
    "params": [["Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss", "8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo"]]
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

	url := "https://solana-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProofs\",\n  \"params\": [\n    [\n      \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n      \"8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo\"\n    ]\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://solana-mainnet.g.alchemy.com/v2/docs-demo")
  .header("Content-Type", "application/json")
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProofs\",\n  \"params\": [\n    [\n      \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n      \"8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo\"\n    ]\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProofs\",\n  \"params\": [\n    [\n      \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n      \"8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo\"\n    ]\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getAssetProofs
description: Returns merkle proofs for multiple compressed digital assets.
x-compute-units: 480
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: ids
    required: true
    description: An array of compressed asset IDs.
    schema:
      type: array
      items:
        title: Asset ID
        type: string
        description: The ID of the asset, typically a base-58 encoded string.
examples:
  - name: getAssetProofs example
    params:
      - name: ids
        value:
          - Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss
          - 8vw7tdLGE3FBjaetsJrZAbbyssUrXai1aBJbUnj5S5uo
result:
  name: Asset proofs
  description: Returns merkle proofs for the specified compressed assets.
  schema:
    title: Asset Proof List
    type: array
    description: Array of asset proofs.
    items:
      title: Asset Proof
      type: object
      properties:
        root:
          type: string
          description: The merkle tree root.
        proof:
          type: array
          description: The merkle proof path.
          items:
            type: string
        node_index:
          type: integer
          description: The node index in the tree.
        leaf:
          type: string
          description: The leaf hash.
        tree_id:
          description: The merkle tree address.
          title: Pubkey
          type: string
```
