# NFT Metadata By Token ID

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nft-metadata-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nft-metadata-v-3.md)

# NFT Metadata By Token ID

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getNFTMetadata

getNFTMetadata - Retrieves the metadata associated with a specific NFT. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-nft-metadata-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |
| tokenId | string | Yes | String - The ID of the token. Can be in hex or decimal format. |
| tokenType | string | No | String - 'ERC721' or 'ERC1155'; specifies type of token to query for. API requests will perform faster if this is specified. |
| tokenUriTimeoutInMs | integer | No | No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0. |
| refreshCache | boolean | No | Defaults to false for faster response times.  If true will refresh metadata for given token. If false will check the cache and use it or refresh if cache doesn't exist. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getNFTMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getNFTMetadata
method: GET
operation:
  summary: NFT Metadata By Token ID
  description: getNFTMetadata - Retrieves the metadata associated with a specific NFT. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
        default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
      required: true
    - name: tokenId
      description: String - The ID of the token. Can be in hex or decimal format.
      in: query
      schema:
        type: string
        default: '44'
      required: true
    - name: tokenType
      description: String - 'ERC721' or 'ERC1155'; specifies type of token to query for. API requests will perform faster if this is specified.
      in: query
      schema:
        type: string
    - name: tokenUriTimeoutInMs
      description: No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0.
      in: query
      schema:
        type: integer
    - name: refreshCache
      description: Defaults to false for faster response times.  If true will refresh metadata for given token. If false will check the cache and use it or refresh if cache doesn't exist.
      in: query
      schema:
        type: boolean
        default: false
  responses:
    '200':
      description: Returns the metadata of the specified NFT.
      content:
        application/json:
          schema:
            type: object
            description: The object that represents an NFT and has all data corresponding to that NFT
            properties:
              contract:
                type: object
                description: The contract object that has details of a contract
                properties:
                  address:
                    description: Address of the held contract
                    type: string
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
                  openseaMetadata:
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
                  isSpam:
                    type: string
                    description: '"true" if contract is spam, else "false". **Only available on paid tiers.**'
                  spamClassifications:
                    description: List of reasons why a contract was classified as spam. **Only available on paid tiers.**
                    type: array
                    items:
                      type: string
              tokenId:
                type: string
                default: '44'
              tokenType:
                type: string
              name:
                type: string
                description: String - Name of the NFT asset.
              description:
                type: string
                description: String - Brief human-readable description
              image:
                type: object
                description: Details of the image corresponding to this contract
                properties:
                  cachedUrl:
                    type: string
                    description: The Url of the image stored in Alchemy cache
                  thumbnailUrl:
                    type: string
                    description: The Url that has the thumbnail version of the NFT
                  pngUrl:
                    type: string
                    description: The Url that has the NFT image in png
                  contentType:
                    type: string
                    description: The Url of the image stored in Alchemy cache
                  size:
                    type: integer
                    description: The size of the media asset in bytes.
                  originalUrl:
                    type: string
                    description: The original Url of the image coming straight from the smart contract
              raw:
                type: object
                description: Raw details of the NFT like its tokenUri and metadata info obtained directly from the smart contract
                properties:
                  tokenUri:
                    type: string
                    description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                  metadata:
                    type: object
                    description: Relevant metadata for NFT contract. This is useful for viewing image url, traits, etc. without having to follow the metadata url in tokenUri to parse manually.
                    properties:
                      image:
                        type: string
                        description: String - URL to the NFT asset image. Can be standard URLs pointing to images on conventional servers, IPFS, or Arweave. Most types of images (SVGs, PNGs, JPEGs, etc.) are supported by NFT marketplaces.
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
                  error:
                    type: string
                    description: String - A string describing a particular reason that we were unable to fetch complete metadata for the NFT.
              collection:
                type: object
                description: The collection object that has details of a collection
                properties:
                  name:
                    type: string
                    description: String - Collection name
                  slug:
                    type: string
                    description: String - OpenSea collection slug
                  externalUrl:
                    type: string
                    description: String - URL for the external site of the collection
                  bannerImageUrl:
                    type: string
                    description: String - Banner image URL for the collection
              tokenUri:
                type: string
                description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
              timeLastUpdated:
                type: string
                description: String - ISO timestamp of the last cache refresh for the information returned in the metadata field.
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
              animation:
                type: object
                properties:
                  cachedUrl:
                    type: string
                  contentType:
                    type: string
                  size:
                    type: integer
                  orginalUrl:
                    type: string
              mint:
                type: object
                properties:
                  mintAddress:
                    type: string
                    description: Address that minted the NFT
                  blockNumber:
                    type: integer
                    description: Block number when the NFT was minted
                  timestamp:
                    type: string
                    description: Timestamp when the NFT was minted
                  transactionHash:
                    type: string
                    description: Transaction hash of the mint transaction
              owners:
                type: array
                description: List of all addresses that own the given NFT.
                items:
                  type: string
  operationId: getNFTMetadata-v3
```
