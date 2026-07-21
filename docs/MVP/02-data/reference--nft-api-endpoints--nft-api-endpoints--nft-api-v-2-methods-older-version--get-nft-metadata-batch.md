# getNFTMetadataBatch

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-metadata-batch.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-metadata-batch.md)

# getNFTMetadataBatch

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getNFTMetadataBatch

Gets the metadata associated with up to 100 given NFT contracts.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nft-metadata-batch

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch \
  --header 'Content-Type: application/json' \
  --data '{
  "tokens": [
    {
      "contractAddress": "0xe785E82358879F061BC3dcAC6f0444462D4b5330",
      "tokenId": "44",
      "tokenType": "string"
    }
  ],
  "tokenUriTimeoutInMs": 1,
  "refreshCache": false
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    tokens: [
      {
        contractAddress: '0xe785E82358879F061BC3dcAC6f0444462D4b5330',
        tokenId: '44',
        tokenType: 'string'
      }
    ],
    tokenUriTimeoutInMs: 1,
    refreshCache: false
  })
};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch"

payload = {
    "tokens": [
        {
            "contractAddress": "0xe785E82358879F061BC3dcAC6f0444462D4b5330",
            "tokenId": "44",
            "tokenType": "string"
        }
    ],
    "tokenUriTimeoutInMs": 1,
    "refreshCache": False
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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch"

	payload := strings.NewReader("{\n  \"tokens\": [\n    {\n      \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n      \"tokenId\": \"44\",\n      \"tokenType\": \"string\"\n    }\n  ],\n  \"tokenUriTimeoutInMs\": 1,\n  \"refreshCache\": false\n}")

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
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch")
  .header("Content-Type", "application/json")
  .body("{\n  \"tokens\": [\n    {\n      \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n      \"tokenId\": \"44\",\n      \"tokenType\": \"string\"\n    }\n  ],\n  \"tokenUriTimeoutInMs\": 1,\n  \"refreshCache\": false\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTMetadataBatch");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"tokens\": [\n    {\n      \"contractAddress\": \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\",\n      \"tokenId\": \"44\",\n      \"tokenType\": \"string\"\n    }\n  ],\n  \"tokenUriTimeoutInMs\": 1,\n  \"refreshCache\": false\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getNFTMetadataBatch
method: POST
operation:
  summary: getNFTMetadataBatch
  description: Gets the metadata associated with up to 100 given NFT contracts.
  tags:
    - NFT API V2 Methods (Older Version)
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
  requestBody:
    content:
      application/json:
        schema:
          type: object
          required:
            - tokens
          properties:
            tokens:
              type: array
              description: List of token objects to batch request NFT metadata for. Maximum 100.
              items:
                type: object
                required:
                  - contractAddress
                  - tokenId
                properties:
                  contractAddress:
                    type: string
                    default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
                  tokenId:
                    type: string
                    default: '44'
                  tokenType:
                    type: string
            tokenUriTimeoutInMs:
              type: integer
            refreshCache:
              type: boolean
              default: false
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: array
            items:
              type: object
              description: The object that represents an NFT and has all data corresponding to that NFT
              properties:
                contract:
                  description: Object - Contract for returned NFT
                  type: object
                  properties:
                    address:
                      type: string
                      description: String - Address of NFT contract.
                id:
                  type: object
                  properties:
                    tokenId:
                      type: string
                      default: '44'
                    tokenMetadata:
                      type: object
                      properties:
                        tokenType:
                          type: string
                          enum:
                            - ERC721
                            - ERC1155
                            - NO_SUPPORTED_NFT_STANDARD
                            - NOT_A_CONTRACT
                          description: String - For valid NFTs, 'ERC721' or 'ERC1155.' For invalid NFTs, a descriptive reason such as 'NO_SUPPORTED_NFT_STANDARD' if the input contract address doesn't support a known NFT standard, or 'NOT_A_CONTRACT' if there is no contract deployed at the input address.
                balance:
                  type: string
                  description: String - Token balance
                title:
                  type: string
                  description: String - Name of the NFT asset.
                description:
                  type: string
                  description: String - Brief human-readable description
                tokenUri:
                  type: object
                  properties:
                    raw:
                      type: string
                      description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                    gateway:
                      type: string
                      description: String - Public gateway uri for the raw uri above.
                media:
                  type: object
                  properties:
                    raw:
                      type: string
                      description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                    gateway:
                      type: string
                      description: String - Public gateway uri for the raw uri above.
                    thumbnail:
                      type: string
                      description: URL for a resized thumbnail of the NFT media asset.
                    format:
                      type: string
                      description: The media format (jpg, gif, png, etc.) of the gateway and thumbnail assets.
                    bytes:
                      type: integer
                      description: The size of the media asset in bytes.
                metadata:
                  type: object
                  description: Relevant metadata for NFT contract. This is useful for viewing image url, traits, etc. without having to follow the metadata url in tokenUri to parse manually.
                  properties:
                    image:
                      type: string
                      description: String - URL to the NFT asset image. Can be standard URLs pointing to images on conventional servers, IPFS, or Arweave. Most types of images (SVGs, PNGs, JPEGs, etc.) are supported by NFT marketplaces.
                    external_url:
                      type: string
                      description: String - The image URL that appears alongside the asset image on NFT platforms.
                    background_color:
                      type: string
                      description: String - Background color of the NFT item. Usually must be defined as a six-character hexadecimal.
                    name:
                      type: string
                      description: String - Name of the NFT asset.
                    description:
                      type: string
                      description: String - Human-readable description of the NFT asset. (Markdown is supported/rendered on OpenSea and other NFT platforms)
                    attributes:
                      type: array
                      items:
                        type: object
                        properties:
                          value:
                            type: string
                          trait_type:
                            type: string
                      description: Object - Traits/attributes/characteristics for each NFT asset.
                    media:
                      type: array
                      items:
                        type: object
                        properties:
                          raw:
                            type: string
                            description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                          gateway:
                            type: string
                            description: String - Public gateway uri for the raw uri above.
                          thumbnail:
                            type: string
                            description: URL for a resized thumbnail of the NFT media asset.
                          format:
                            type: string
                            description: The media format (jpg, gif, png, etc.) of the gateway and thumbnail assets.
                          bytes:
                            type: integer
                            description: The size of the media asset in bytes.
                timeLastUpdated:
                  type: string
                  description: String - ISO timestamp of the last cache refresh for the information returned in the metadata field.
                error:
                  type: string
                  description: String - A string describing a particular reason that we were unable to fetch complete metadata for the NFT.
                contractMetadata:
                  type: object
                  properties:
                    name:
                      type: string
                      description: String - NFT contract name.
                    symbol:
                      type: string
                      description: String - NFT contract symbol abbreviation.
                    totalSupply:
                      type: string
                      description: String - Total number of NFTs in a given NFT collection.
                    tokenType:
                      type: string
                      enum:
                        - ERC721
                        - ERC1155
                        - NO_SUPPORTED_NFT_STANDARD
                        - NOT_A_CONTRACT
                      description: String - For valid NFTs, 'ERC721' or 'ERC1155.' For invalid NFTs, a descriptive reason such as 'NO_SUPPORTED_NFT_STANDARD' if the input contract address doesn't support a known NFT standard, or 'NOT_A_CONTRACT' if there is no contract deployed at the input address.
                    contractDeployer:
                      type: string
                      description: String - Address that deployed the smart contract
                    deployedBlockNumber:
                      type: number
                      description: Number - The Block Number when the deployment transaction is successfully mined
                    opensea:
                      type: object
                      description: Note that the OpenSea metadata object is currently only available on ETH and Polygon Mainnet. Please reach out to us at support@alchemy.com if you would like to access this data on other networks.
                      properties:
                        floorPrice:
                          type: number
                          description: NFT floor price
                        collectionName:
                          type: string
                          description: OpenSea collection name
                        safelistRequestStatus:
                          type: string
                          description: Collection approval status within OpenSea. For more info, see the Opensea docs at docs.opensea.io/reference/collection-model
                        imageUrl:
                          type: string
                          description: OpenSea CDN image URL
                        description:
                          type: string
                          description: 'OpenSea collection description. Note: this value is truncated to 255 characters.'
                        externalUrl:
                          type: string
                          description: Collection homepage
                        twitterUsername:
                          type: string
                          description: The twitter username of the collection
                        discordUrl:
                          type: string
                          description: The discord URL of the collection
                        bannerImageUrl:
                          type: string
                          description: The banner image URL of the collection
                        lastIngestedAt:
                          type: string
                          description: The timestamp when the collection was last ingested by us
                spamInfo:
                  type: object
                  description: Information about whether and why a contract was marked as spam.
                  properties:
                    isSpam:
                      type: string
                      description: '"true" if contract is spam, else "false". **Only available on paid tiers.**'
                    spamClassifications:
                      description: List of reasons why a contract was classified as spam. **Only available on paid tiers.**
                      type: array
                      items:
                        type: string
                acquiredAt:
                  type: object
                  description: Only present if the request specified `orderBy=transferTime`.
                  properties:
                    blockTimestamp:
                      type: string
                      description: Block timestamp of the block where the NFT was most recently acquired.
                    blockNumber:
                      type: string
                      description: Block number of the block where the NFT was most recently acquired.
  operationId: getNFTMetadataBatch
```
