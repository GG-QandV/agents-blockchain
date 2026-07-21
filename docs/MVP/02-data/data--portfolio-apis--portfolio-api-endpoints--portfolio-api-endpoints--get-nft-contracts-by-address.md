# NFT Collections By Wallet

> Source: [https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nft-contracts-by-address.md](https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nft-contracts-by-address.md)

# NFT Collections By Wallet

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://api.g.alchemy.com/data/v1/{apiKey}/assets/nfts/contracts/by-address

Fetches NFT collections (contracts) for multiple wallet addresses and networks. Returns a list of collections and metadata for each wallet/network combination. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).


Reference: https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-nft-contracts-by-address

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address \
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

fetch('https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address"

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

	url := "https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address"

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
HttpResponse<String> response = Unirest.post("https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address")
  .header("Content-Type", "application/json")
  .body("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ],\n      \"excludeFilters\": [\n        \"SPAM\"\n      ],\n      \"includeFilters\": [\n        \"SPAM\"\n      ],\n      \"spamConfidenceLevel\": \"VERY_HIGH\"\n    }\n  ],\n  \"withMetadata\": true,\n  \"pageKey\": \"string\",\n  \"pageSize\": 100,\n  \"orderBy\": \"transferTime\",\n  \"sortOrder\": \"asc\"\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://api.g.alchemy.com/data/v1/docs-demo/assets/nfts/contracts/by-address");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"addresses\": [\n    {\n      \"address\": \"0x1E6E8695FAb3Eb382534915eA8d7Cc1D1994B152\",\n      \"networks\": [\n        \"eth-mainnet\"\n      ],\n      \"excludeFilters\": [\n        \"SPAM\"\n      ],\n      \"includeFilters\": [\n        \"SPAM\"\n      ],\n      \"spamConfidenceLevel\": \"VERY_HIGH\"\n    }\n  ],\n  \"withMetadata\": true,\n  \"pageKey\": \"string\",\n  \"pageSize\": 100,\n  \"orderBy\": \"transferTime\",\n  \"sortOrder\": \"asc\"\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /{apiKey}/assets/nfts/contracts/by-address
method: POST
operation:
  summary: NFT Collections By Wallet
  description: |
    Fetches NFT collections (contracts) for multiple wallet addresses and networks. Returns a list of collections and metadata for each wallet/network combination. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
                description: List of nft collections.
                properties:
                  contracts:
                    type: array
                    items:
                      type: object
                      description: The object that represents an NFT collection
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
  operationId: get-nft-contracts-by-address
```
