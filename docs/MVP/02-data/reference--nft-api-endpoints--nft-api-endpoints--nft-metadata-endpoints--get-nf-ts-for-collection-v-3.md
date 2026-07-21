# NFTs By Collection

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nf-ts-for-collection-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nf-ts-for-collection-v-3.md)

# NFTs By Collection

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getNFTsForCollection

getNFTsForCollection - Retrieves all NFTs associated with a specific NFT collection. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nf-ts-for-collection-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | No | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |
| collectionSlug | string | No | String - OpenSea slug for the NFT collection. |
| withMetadata | boolean | No | Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`. |
| startToken | string | No | String - A tokenID offset used for pagination. Can be a hex string, or a decimal. Users can specify the offset themselves to start from a custom offset, or to fetch multiple token ranges in parallel. |
| limit | integer | No | Integer - Sets the total number of NFTs returned in the response. Defaults to 100. |
| tokenUriTimeoutInMs | integer | No | No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection"

response = requests.get(url)

print(response.text)
```

### Go

```go
package main

import (
	"fmt"
	"net/http"
	"io"
)

func main() {

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTsForCollection");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getNFTsForCollection
method: GET
operation:
  summary: NFTs By Collection
  description: getNFTsForCollection - Retrieves all NFTs associated with a specific NFT collection. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
  tags:
    - NFT Metadata Endpoints
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: contractAddress
      description: String - Contract address for the NFT contract (ERC721 and ERC1155 supported).
      in: query
      schema:
        type: string
      required: false
    - name: collectionSlug
      description: String - OpenSea slug for the NFT collection.
      in: query
      schema:
        type: string
        default: boredapeyachtclub
      required: false
    - name: withMetadata
      description: Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`.
      schema:
        type: boolean
        default: true
      in: query
    - name: startToken
      description: String - A tokenID offset used for pagination. Can be a hex string, or a decimal. Users can specify the offset themselves to start from a custom offset, or to fetch multiple token ranges in parallel.
      in: query
      schema:
        type: string
    - name: limit
      description: Integer - Sets the total number of NFTs returned in the response. Defaults to 100.
      in: query
      schema:
        type: integer
    - name: tokenUriTimeoutInMs
      description: No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0.
      in: query
      schema:
        type: integer
  responses:
    '200':
      description: Returns a list of NFTs associated with the specified collection.
      content:
        application/json:
          schema:
            type: object
            properties:
              nfts:
                description: List of objects that represent NFTs stored under the queried contract address or collection slug.
                type: array
                items:
                  type: object
                  properties:
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
                    tokenUri:
                      type: object
                      properties:
                        raw:
                          type: string
                          description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                        gateway:
                          type: string
                          description: String - Public gateway uri for the raw uri above.
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
              nextToken:
                type: string
                description: String - An offset used for pagination. Can be passed back as the "startToken" of a subsequent request to get the next page of results. Absent if there are no more results.
  operationId: getNFTsForCollection-v3
```
