# getAssetSignatures

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-signatures.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-signatures.md)

# getAssetSignatures

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns signatures for transactions that have interacted with the given asset.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-asset-signatures

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | The ID of the asset to get signatures for. |
| limit | integer | No | The maximum number of signatures to retrieve. |
| page | integer | No | The index of the "page" to retrieve. |
| before | string | No | Retrieve signatures before this cursor. |
| after | string | No | Retrieve signatures after this cursor. |
| tree | string | No | The merkle tree address to filter by. |

## Result

**Asset signatures** (object): Returns signatures for transactions involving the specified asset.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getAssetSignatures",
  "params": [
    "FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC"
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
  "method": "getAssetSignatures",
  "params": [
    "FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC",
    100,
    1,
    "string",
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
    method: 'getAssetSignatures',
    params: [
      'FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC',
      100,
      1,
      'string',
      'string',
      'string'
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
    "method": "getAssetSignatures",
    "params": ["FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC", 100, 1, "string", "string", "string"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetSignatures\",\n  \"params\": [\n    \"FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC\",\n    100,\n    1,\n    \"string\",\n    \"string\",\n    \"string\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetSignatures\",\n  \"params\": [\n    \"FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC\",\n    100,\n    1,\n    \"string\",\n    \"string\",\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetSignatures\",\n  \"params\": [\n    \"FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC\",\n    100,\n    1,\n    \"string\",\n    \"string\",\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getAssetSignatures
description: Returns signatures for transactions that have interacted with the given asset.
x-compute-units: 160
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: id
    required: true
    description: The ID of the asset to get signatures for.
    schema:
      title: Asset ID
      type: string
      description: The ID of the asset, typically a base-58 encoded string.
  - name: limit
    required: false
    description: The maximum number of signatures to retrieve.
    schema:
      type: integer
      maximum: 1000
      default: 100
  - name: page
    required: false
    description: The index of the "page" to retrieve.
    schema:
      type: integer
      default: 1
  - name: before
    required: false
    description: Retrieve signatures before this cursor.
    schema:
      type: string
  - name: after
    required: false
    description: Retrieve signatures after this cursor.
    schema:
      type: string
  - name: tree
    required: false
    description: The merkle tree address to filter by.
    schema:
      title: Pubkey
      type: string
      description: Base-58 encoded public key.
examples:
  - name: getAssetSignatures example
    params:
      - name: id
        value: FNt6A9Mfnqbwc1tY7uwAguKQ1JcpBrxmhczDgbdJy5AC
result:
  name: Asset signatures
  description: Returns signatures for transactions involving the specified asset.
  schema:
    title: Asset Signature List
    type: object
    properties:
      total:
        type: integer
        description: Total number of signatures.
      limit:
        type: integer
        description: Number of signatures returned.
      page:
        type: integer
        nullable: true
        description: Current page number.
      items:
        type: array
        description: Array of asset signatures.
        items:
          title: Asset Signature
          type: object
          properties:
            signature:
              type: string
              description: The transaction signature.
            slot:
              type: integer
              description: The slot number.
            blockTime:
              type: integer
              nullable: true
              description: The block time as a Unix timestamp.
            instruction:
              type: string
              description: The instruction type.
            tree:
              title: Pubkey
              type: string
              description: The merkle tree address.
            leafIndex:
              type: integer
              nullable: true
              description: The leaf index in the merkle tree.
            seq:
              type: integer
              nullable: true
              description: The sequence number.
```
