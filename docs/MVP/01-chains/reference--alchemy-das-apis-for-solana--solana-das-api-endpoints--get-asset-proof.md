# getAssetProof

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proof.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proof.md)

# getAssetProof

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns the merkle proof for a compressed digital asset.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-proof

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | The ID of the compressed asset. |

## Result

**Asset proof** (object): Returns the merkle proof for the specified compressed asset.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getAssetProof",
  "params": [
    "Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss"
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
  "method": "getAssetProof",
  "params": [
    "Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss"
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
    method: 'getAssetProof',
    params: ['Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss']
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
    "method": "getAssetProof",
    "params": ["Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProof\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProof\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetProof\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getAssetProof
description: Returns the merkle proof for a compressed digital asset.
x-compute-units: 160
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: id
    required: true
    description: The ID of the compressed asset.
    schema:
      title: Asset ID
      type: string
      description: The ID of the asset, typically a base-58 encoded string.
examples:
  - name: getAssetProof example
    params:
      - name: id
        value: Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss
result:
  name: Asset proof
  description: Returns the merkle proof for the specified compressed asset.
  schema:
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
