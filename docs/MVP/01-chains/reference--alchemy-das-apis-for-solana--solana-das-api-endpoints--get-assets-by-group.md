# getAssetsByGroup

> Source: [https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-assets-by-group.md](https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-assets-by-group.md)

# getAssetsByGroup

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://solana-mainnet.g.alchemy.com/v2/{apiKey}

Returns assets filtered by their group information.

Reference: https://www.alchemy.com/docs/reference/alchemy-das-apis-for-solana/solana-das-api-endpoints/get-assets-by-group

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| groupKey | string | Yes | The group key to filter by. |
| groupValue | string | Yes | The group value to filter by. |
| sortBy | object | No | Sorting criteria for assets. |
| limit | integer | No | The maximum number of assets to retrieve. |
| page | integer | No | The index of the "page" to retrieve. |
| before | string | No | Retrieve assets before this cursor. |
| after | string | No | Retrieve assets after this cursor. |
| options | object | No | Optional response shaping flags. The server also accepts `displayOptions` as an alias of this field, but only one of `options` or `displayOptions` may be present per request. |

## Result

**Assets by group** (object): Returns assets filtered by the specified group.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "getAssetsByGroup",
  "params": [
    "collection",
    "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w",
    {
      "sortBy": "created",
      "sortDirection": "asc"
    },
    1
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
  "method": "getAssetsByGroup",
  "params": [
    "collection",
    "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w",
    {
      "sortBy": "created",
      "sortDirection": "asc"
    },
    1,
    1,
    "string",
    "string",
    {
      "showUnverifiedCollections": false,
      "showCollectionMetadata": false,
      "showZeroBalance": false,
      "showInscription": false,
      "showFungible": false
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
    method: 'getAssetsByGroup',
    params: [
      'collection',
      'J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w',
      {sortBy: 'created', sortDirection: 'asc'},
      1,
      1,
      'string',
      'string',
      {
        showUnverifiedCollections: false,
        showCollectionMetadata: false,
        showZeroBalance: false,
        showInscription: false,
        showFungible: false
      }
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
    "method": "getAssetsByGroup",
    "params": [
        "collection",
        "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w",
        {
            "sortBy": "created",
            "sortDirection": "asc"
        },
        1,
        1,
        "string",
        "string",
        {
            "showUnverifiedCollections": False,
            "showCollectionMetadata": False,
            "showZeroBalance": False,
            "showInscription": False,
            "showFungible": False
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

	url := "https://solana-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetsByGroup\",\n  \"params\": [\n    \"collection\",\n    \"J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w\",\n    {\n      \"sortBy\": \"created\",\n      \"sortDirection\": \"asc\"\n    },\n    1,\n    1,\n    \"string\",\n    \"string\",\n    {\n      \"showUnverifiedCollections\": false,\n      \"showCollectionMetadata\": false,\n      \"showZeroBalance\": false,\n      \"showInscription\": false,\n      \"showFungible\": false\n    }\n  ]\n}")

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
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetsByGroup\",\n  \"params\": [\n    \"collection\",\n    \"J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w\",\n    {\n      \"sortBy\": \"created\",\n      \"sortDirection\": \"asc\"\n    },\n    1,\n    1,\n    \"string\",\n    \"string\",\n    {\n      \"showUnverifiedCollections\": false,\n      \"showCollectionMetadata\": false,\n      \"showZeroBalance\": false,\n      \"showInscription\": false,\n      \"showFungible\": false\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://solana-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"getAssetsByGroup\",\n  \"params\": [\n    \"collection\",\n    \"J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w\",\n    {\n      \"sortBy\": \"created\",\n      \"sortDirection\": \"asc\"\n    },\n    1,\n    1,\n    \"string\",\n    \"string\",\n    {\n      \"showUnverifiedCollections\": false,\n      \"showCollectionMetadata\": false,\n      \"showZeroBalance\": false,\n      \"showInscription\": false,\n      \"showFungible\": false\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: getAssetsByGroup
description: Returns assets filtered by their group information.
x-compute-units: 480
x-rate-limit-cus: 200
paramStructure: by-name
params:
  - name: groupKey
    required: true
    description: The group key to filter by.
    schema:
      type: string
  - name: groupValue
    required: true
    description: The group value to filter by.
    schema:
      type: string
  - name: sortBy
    required: false
    description: Sorting criteria for assets.
    schema:
      type: object
      properties:
        sortBy:
          type: string
          enum:
            - created
            - updated
            - recent_action
            - none
          description: The field to sort by.
        sortDirection:
          type: string
          enum:
            - asc
            - desc
          description: Sort direction.
  - name: limit
    required: false
    description: The maximum number of assets to retrieve.
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
    description: Retrieve assets before this cursor.
    schema:
      type: string
  - name: after
    required: false
    description: Retrieve assets after this cursor.
    schema:
      type: string
  - name: options
    required: false
    description: Optional response shaping flags. The server also accepts `displayOptions` as an alias of this field, but only one of `options` or `displayOptions` may be present per request.
    schema:
      title: Display Options
      type: object
      description: |
        Optional response shaping flags accepted by the Alchemy DAS proxy. All flags are booleans and default to `false`.
      properties:
        showUnverifiedCollections:
          type: boolean
          description: Show unverified collections instead of skipping them.
          default: false
        showCollectionMetadata:
          type: boolean
          description: Show metadata for the collection.
          default: false
        showZeroBalance:
          type: boolean
          description: Display assets with zero balance.
          default: false
        showInscription:
          type: boolean
          description: Display inscription details.
          default: false
        showFungible:
          type: boolean
          description: Include fungible assets in the result.
          default: false
examples:
  - name: getAssetsByGroup example
    params:
      - name: groupKey
        value: collection
      - name: groupValue
        value: J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w
      - name: sortBy
        value:
          sortBy: created
          sortDirection: asc
      - name: page
        value: 1
result:
  name: Assets by group
  description: Returns assets filtered by the specified group.
  schema:
    title: Asset List
    type: object
    properties:
      total:
        type: integer
        description: Total number of assets.
      limit:
        type: integer
        description: Number of assets returned.
      page:
        type: integer
        nullable: true
        description: Current page number.
      items:
        type: array
        description: Array of assets.
        items:
          title: Asset
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
            mutable:
              type: boolean
              description: Whether the asset is mutable.
            burnt:
              type: boolean
              description: Whether the asset is burnt.
            supply:
              type: object
              properties:
                print_max_supply:
                  type: integer
                  nullable: true
                  description: The maximum print supply.
                print_current_supply:
                  type: integer
                  nullable: true
                  description: The current print supply.
                edition_nonce:
                  type: integer
                  nullable: true
                  description: The edition nonce.
```
