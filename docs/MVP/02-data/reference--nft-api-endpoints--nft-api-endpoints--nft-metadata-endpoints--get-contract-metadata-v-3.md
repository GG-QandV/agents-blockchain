# Contract Metadata By Address

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-contract-metadata-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-contract-metadata-v-3.md)

# Contract Metadata By Address

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getContractMetadata

getContractMetadata - Retrieves high-level collection or contract-level information for an NFT. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-contract-metadata-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

> 📄 **This content also appears in [getContractMetadata](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contract-metadata.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getContractMetadata?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getContractMetadata
method: GET
operation:
  summary: Contract Metadata By Address
  description: getContractMetadata - Retrieves high-level collection or contract-level information for an NFT. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
  responses:
    '200':
      description: Returns the contract metadata for the specified address.
      content:
        application/json:
          schema:
            type: object
            properties:
              address:
                type: string
                description: String - Contract address for the queried NFT collection
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
  operationId: getContractMetadata-v3
```
