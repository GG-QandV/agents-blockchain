# NFTs By Wallet

> Source: [https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nfts-by-address.md](https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nfts-by-address.md)

# NFTs By Wallet

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/v1/{apiKey}/assets/nfts/by-address

Fetches NFTs for multiple wallet addresses and networks. Returns a list of NFTs and metadata for each wallet/network combination. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).


Reference: https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nfts-by-address

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address \
  --header 'Content-Type: application/json' \
  --data '{
  "addresses": [
    {
      "address": "0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152",
      "networks": [
        "eth-mainnet"
      ],
      "excludeFilters": [
        "SPAM"
      ],
      "includeFilters": [
        "SPAM"
      ],
      "spamConfidenceLevel": "VERY_HIGH"
    }
  ],
  "withMetadata": true,
  "pageKey": "string",
  "pageSize": 100,
  "orderBy": "transferTime",
  "sortOrder": "asc"
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    addresses: [
      {
        address: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152',
        networks: ['eth-mainnet'],
        excludeFilters: ['SPAM'],
        includeFilters: ['SPAM'],
        spamConfidenceLevel: 'VERY_HIGH'
      }
    ],
    withMetadata: true,
    pageKey: 'string',
    pageSize: 100,
    orderBy: 'transferTime',
    sortOrder: 'asc'
  })
};

fetch('https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address"

payload = {
    "addresses": [
        {
            "address": "0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152",
            "networks": ["eth-mainnet"],
            "excludeFilters": ["SPAM"],
            "includeFilters": ["SPAM"],
            "spamConfidenceLevel": "VERY_HIGH"
        }
    ],
    "withMetadata": True,
    "pageKey": "string",
    "pageSize": 100,
    "orderBy": "transferTime",
    "sortOrder": "asc"
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

	url := "https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address"

	payload := strings.NewReader("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ],\n      \"excludeFilters\": [\n        \"SPAM\"\n      ],\n      \"includeFilters\": [\n        \"SPAM\"\n      ],\n      \"spamConfidenceLevel\": \"VERY_HIGH\"\n    }\n  ],\n  \"withMetadata\": true,\n  \"pageKey\": \"string\",\n  \"pageSize\": 100,\n  \"orderBy\": \"transferTime\",\n  \"sortOrder\": \"asc\"\n}")

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address")
  .header("Content-Type", "application/json")
  .body("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ],\n      \"excludeFilters\": [\n        \"SPAM\"\n      ],\n      \"includeFilters\": [\n        \"SPAM\"\n      ],\n      \"spamConfidenceLevel\": \"VERY_HIGH\"\n    }\n  ],\n  \"withMetadata\": true,\n  \"pageKey\": \"string\",\n  \"pageSize\": 100,\n  \"orderBy\": \"transferTime\",\n  \"sortOrder\": \"asc\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/by-address");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ],\n      \"excludeFilters\": [\n        \"SPAM\"\n      ],\n      \"includeFilters\": [\n        \"SPAM\"\n      ],\n      \"spamConfidenceLevel\": \"VERY_HIGH\"\n    }\n  ],\n  \"withMetadata\": true,\n  \"pageKey\": \"string\",\n  \"pageSize\": 100,\n  \"orderBy\": \"transferTime\",\n  \"sortOrder\": \"asc\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/assets/nfts/by-address
method: POST
operation:
  summary: NFTs By Wallet
  description: |
    Fetches NFTs for multiple wallet addresses and networks. Returns a list of NFTs and metadata for each wallet/network combination. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
  tags:
    - Portfolio API Endpoints
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
  requestBody:
    required: true
    content:
      application/json:
        schema:
          allOf:
            - type: object
              properties:
                addresses:
                  type: array
                  description: |
                    Array of address and networks pairs (limit 2 pairs, max 15 networks each). Networks should match network enums.
                  items:
                    type: object
                    properties:
                      address:
                        type: string
                        description: Wallet address.
                        example: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152'
                        default: '0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152'
                      networks:
                        type: array
                        default:
                          - eth-mainnet
                          - base-mainnet
                          - matic-mainnet
                        items:
                          type: string
                          enum:
                            - eth-mainnet
                            - eth-sepolia
                            - eth-holesky
                            - avax-mainnet
                            - avax-fuji
                            - zksync-mainnet
                            - opt-mainnet
                            - polygon-mainnet
                            - polygon-amoy
                            - arb-mainnet
                            - arb-sepolia
                            - blast-mainnet
                            - blast-sepolia
                            - base-mainnet
                            - base-sepolia
                            - soneium-mainnet
                            - soneium-minato
                            - scroll-mainnet
                            - scroll-sepolia
                            - shape-mainnet
                            - shape-sepolia
                            - lens-mainnet
                            - lens-sepolia
                            - starknet-mainnet
                            - starknet-sepolia
                            - rootstock-mainnet
                            - rootstock-testnet
                            - linea-mainnet
                            - linea-sepolia
                            - settlus-septestnet
                            - abstract-mainnet
                            - abstract-testnet
                            - apechain-mainnet
                          default: eth-mainnet
                        description: Network identifier (e.g., eth-mainnet). Find more network enums [here](https://dashboard.alchemy.com/chains)
                      excludeFilters:
                        type: array
                        items:
                          type: string
                          enum:
                            - SPAM
                            - AIRDROPS
                          default: SPAM
                      includeFilters:
                        type: array
                        items:
                          type: string
                          enum:
                            - SPAM
                            - AIRDROPS
                          default: SPAM
                      spamConfidenceLevel:
                        type: string
                        enum:
                          - VERY_HIGH
                          - HIGH
                          - MEDIUM
                          - LOW
                    required:
                      - address
                      - networks
                withMetadata:
                  description: Boolean - if set to `true`, returns metadata. Setting this to false will reduce payload size and may result in a faster API call.
                  type: boolean
                  default: true
                pageKey:
                  type: string
                pageSize:
                  type: integer
                  default: 100
              required:
                - addresses
            - type: object
              properties:
                orderBy:
                  type: string
                  enum:
                    - transferTime
                  description: Field to order results by
                sortOrder:
                  type: string
                  enum:
                    - asc
                    - desc
                  description: Sort order for results
  responses:
    '200':
      description: Successful response!
      content:
        application/json:
          schema:
            type: object
            properties:
              data:
                type: object
                description: List of nfts by address with appropriate metadata.
                properties:
                  ownedNfts:
                    type: array
                    items:
                      type: object
                      description: The object that represents an NFT and has all data corresponding to that NFT
                      properties:
                        network:
                          type: string
                          description: Network identifier.
                        address:
                          type: string
                          description: Wallet address.
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
                  totalCount:
                    type: integer
                    description: Integer - Total number of NFTs (distinct `tokenIds`) owned by the given address.
                  pageKey:
                    type: string
            required:
              - data
    '400':
      description: 'Bad Request: Invalid input (e.g., malformed JSON).'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
    '429':
      description: 'Too Many Requests: Rate limit exceeded.'
      content:
        application/json:
          schema:
            type: object
            properties:
              error:
                type: object
                properties:
                  message:
                    type: string
                    description: Detailed error message.
                required:
                  - message
                description: Error details.
            required:
              - error
  operationId: get-nfts-by-address
```
