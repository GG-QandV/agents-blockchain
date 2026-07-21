# searchContractMetadata

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/search-contract-metadata.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/search-contract-metadata.md)

# searchContractMetadata

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/searchContractMetadata

Search for a keyword across metadata of all ERC-721 and ERC-1155 smart contracts

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/search-contract-metadata

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| query | string | Yes | String - The search string that you want to search for in contract metadata |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/searchContractMetadata?query=bored");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/searchContractMetadata
method: GET
operation:
  summary: searchContractMetadata
  description: Search for a keyword across metadata of all ERC-721 and ERC-1155 smart contracts
  tags:
    - NFT API V2 Methods (Older Version)
  servers:
    - url: https://{network}.g.alchemy.com/nft
      variables:
        network:
          enum:
            - eth-mainnet
          default: eth-mainnet
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: query
      description: String - The search string that you want to search for in contract metadata
      in: query
      schema:
        type: string
        default: bored
      required: true
  responses:
    '200':
      description: Returns the list of NFT contracts where the metadata has one or more keywords from the search string.
      content:
        application/json:
          schema:
            type: array
            description: List of contracts where the metadata contains one or more keywords from the search string.
            items:
              type: object
              properties:
                address:
                  type: string
                  default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
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
  operationId: searchContractMetadata
```
