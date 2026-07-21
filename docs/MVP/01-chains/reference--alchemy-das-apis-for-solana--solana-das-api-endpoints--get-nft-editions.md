# getNftEditions

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-nft-editions.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-nft-editions.md)

# getNftEditions

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns all editions of a given master NFT.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-nft-editions

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| mintAddress | string | Yes | The ID of the master NFT to get editions for. |
| limit | integer | No | The maximum number of editions to retrieve. |
| page | integer | No | The index of the "page" to retrieve. |
| before | string | No | Retrieve editions before this cursor. |
| after | string | No | Retrieve editions after this cursor. |

## Result

**NFT editions** (object): Returns all editions of the specified master NFT.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getNftEditions",
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
  "method": "getNftEditions",
  "params": [
    "Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss",
    100,
    1,
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
    method: 'getNftEditions',
    params: ['Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss', 100, 1, 'string', 'string']
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
    "method": "getNftEditions",
    "params": ["Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss", 100, 1, "string", "string"]
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

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getNftEditions\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n    100,\n    1,\n    \"string\",\n    \"string\"\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getNftEditions\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n    100,\n    1,\n    \"string\",\n    \"string\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getNftEditions\",\n  \"params\": [\n    \"Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss\",\n    100,\n    1,\n    \"string\",\n    \"string\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getNftEditions
description: Returns all editions of a given master NFT.
x-compute-units: 160
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: mintAddress
    required: true
    description: The ID of the master NFT to get editions for.
    schema:
      title: Asset ID
      type: string
      description: The ID of the asset, typically a base-58 encoded string.
  - name: limit
    required: false
    description: The maximum number of editions to retrieve.
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
    description: Retrieve editions before this cursor.
    schema:
      type: string
  - name: after
    required: false
    description: Retrieve editions after this cursor.
    schema:
      type: string
examples:
  - name: getNftEditions example
    params:
      - name: mintAddress
        value: Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss
result:
  name: NFT editions
  description: Returns all editions of the specified master NFT.
  schema:
    title: NFT Edition List
    type: object
    properties:
      total:
        type: integer
        description: Total number of editions.
      limit:
        type: integer
        description: Number of editions returned.
      page:
        type: integer
        nullable: true
        description: Current page number.
      items:
        type: array
        description: Array of NFT editions.
        items:
          title: NFT Edition
          type: object
          properties:
            interface:
              title: Asset Interface
              type: string
              enum:
                - V1_NFT
                - V1_PRINT
                - LEGACY_NFT
                - V2_NFT
                - FungibleAsset
                - Custom
                - Identity
                - Executable
                - ProgrammableNFT
              description: The interface of the asset.
            id:
              title: Asset ID
              type: string
              description: The ID of the asset, typically a base-58 encoded string.
            content:
              title: Asset Content
              type: object
              properties:
                $schema:
                  type: string
                  description: The schema version.
                json_uri:
                  type: string
                  description: The URI of the JSON metadata.
                files:
                  type: array
                  items:
                    type: object
                    properties:
                      uri:
                        type: string
                        description: The URI of the file.
                      cdn_uri:
                        type: string
                        nullable: true
                        description: The CDN URI of the file.
                      mime:
                        type: string
                        description: The MIME type of the file.
                metadata:
                  type: object
                  properties:
                    attributes:
                      type: array
                      items:
                        type: object
                        properties:
                          value:
                            oneOf:
                              - type: string
                              - type: number
                            description: The value of the attribute.
                          trait_type:
                            type: string
                            description: The trait type of the attribute.
                    description:
                      type: string
                      description: The description of the asset.
                    name:
                      type: string
                      description: The name of the asset.
                    symbol:
                      type: string
                      description: The symbol of the asset.
                links:
                  type: object
                  nullable: true
                  description: External links related to the asset.
            authorities:
              type: array
              items:
                title: Asset Authority
                type: object
                properties:
                  address:
                    description: The address of the authority.
                    title: Pubkey
                    type: string
                  scopes:
                    type: array
                    description: The scopes of the authority.
                    items:
                      type: string
            compression:
              title: Asset Compression
              type: object
              properties:
                eligible:
                  type: boolean
                  description: Whether the asset is eligible for compression.
                compressed:
                  type: boolean
                  description: Whether the asset is compressed.
                data_hash:
                  type: string
                  nullable: true
                  description: The data hash of the compressed asset.
                creator_hash:
                  type: string
                  nullable: true
                  description: The creator hash of the compressed asset.
                asset_hash:
                  type: string
                  nullable: true
                  description: The asset hash of the compressed asset.
                tree:
                  nullable: true
                  description: The merkle tree address.
                  title: Pubkey
                  type: string
                seq:
                  type: integer
                  nullable: true
                  description: The sequence number.
                leaf_id:
                  type: integer
                  nullable: true
                  description: The leaf ID in the merkle tree.
            grouping:
              type: array
              items:
                title: Asset Grouping
                type: object
                properties:
                  group_key:
                    type: string
                    description: The key of the group.
                  group_value:
                    type: string
                    description: The value of the group.
            royalty:
              title: Asset Royalty
              type: object
              properties:
                royalty_model:
                  type: string
                  enum:
                    - creators
                    - fanout
                    - single
                  description: The royalty model.
                target:
                  type: string
                  nullable: true
                  description: The target of the royalty.
                percent:
                  type: number
                  description: The royalty percentage.
                basis_points:
                  type: integer
                  description: The royalty in basis points.
                primary_sale_happened:
                  type: boolean
                  description: Whether the primary sale has happened.
                locked:
                  type: boolean
                  description: Whether the royalty is locked.
            creators:
              type: array
              items:
                title: Asset Creator
                type: object
                properties:
                  address:
                    description: The address of the creator.
                    title: Pubkey
                    type: string
                  share:
                    type: integer
                    description: The creator's share percentage.
                  verified:
                    type: boolean
                    description: Whether the creator is verified.
            ownership:
              title: Asset Ownership
              type: object
              properties:
                frozen:
                  type: boolean
                  description: Whether the asset is frozen.
                delegated:
                  type: boolean
                  description: Whether the asset is delegated.
                delegate:
                  type: string
                  nullable: true
                  description: The delegate of the asset, if any.
                ownership_model:
                  type: string
                  enum:
                    - single
                    - token
                  description: The ownership model.
                owner:
                  description: The owner of the asset.
                  title: Pubkey
                  type: string
            supply:
              type: object
              properties:
                edition_nonce:
                  type: integer
                  nullable: true
                  description: The edition nonce.
            mutable:
              type: boolean
              description: Whether the edition is mutable.
            burnt:
              type: boolean
              description: Whether the edition is burnt.
```
