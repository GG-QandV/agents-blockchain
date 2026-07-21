# getContractMetadataBatch

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-contract-metadata-batch.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-contract-metadata-batch.md)

# getContractMetadataBatch

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getContractMetadataBatch

Gets the metadata associated with the given list of contract addresses

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-contract-metadata-batch

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch \
  --header 'Content-Type: application/json' \
  --data '{
  "contractAddresses": [
    "0xe785E82358879F061BC3dcAC6f0444462D4b5330"
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({contractAddresses: ['0xe785E82358879F061BC3dcAC6f0444462D4b5330']})
};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch"

payload = { "contractAddresses": ["0xe785E82358879F061BC3dcAC6f0444462D4b5330"] }
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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch"

	payload := strings.NewReader("{\n  \"contractAddresses\": [\n    \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\"\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch")
  .header("Content-Type", "application/json")
  .body("{\n  \"contractAddresses\": [\n    \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\"\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getContractMetadataBatch");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"contractAddresses\": [\n    \"0xe785E82358879F061BC3dcAC6f0444462D4b5330\"\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getContractMetadataBatch
method: POST
operation:
  summary: getContractMetadataBatch
  description: Gets the metadata associated with the given list of contract addresses
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
          properties:
            contractAddresses:
              type: array
              description: list of contract addresses to batch metadata requests for
              default:
                - '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
                - '0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d'
              items:
                type: string
                default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: array
            items:
              type: object
              properties:
                address:
                  type: string
                  default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
                contractMetadata:
                  type: object
                  description: The object that represents a smart contract and has all data corresponding to that contract
                  properties:
                    address:
                      description: Address of the held contract
                      type: string
                    totalBalance:
                      type: number
                      description: Sum of NFT balances across all token IDs held by the owner. For non-fungible tokens this will be equal to the `numDistinctTokensOwned`, but it may be higher if the user holds some fungible ERC1155 tokens.
                    numDistinctTokensOwned:
                      type: number
                      description: Number of distinct token IDs held by the owner. For non-fungible tokens this will be equal to the `totalBalance`, but it may be lower if the user holds some fungible ERC1155 tokens.
                    isSpam:
                      type: boolean
                    tokenId:
                      description: One of the tokens from this contract held by the owner.
                      type: string
                    name:
                      description: The name of the contract, i.e. "Bored Ape Yacht Club".
                      type: string
                    title:
                      description: 'The title of the token held by the owner i.e. "Something #22".'
                      type: string
                    symbol:
                      description: The symbol of the contract, i.e. BAYC.
                      type: string
                    tokenType:
                      description: The NFT standard used by the contract, i.e. ERC721 or ERC1155.
                      type: string
                    contractDeployer:
                      type: string
                      description: String - Address that deployed the smart contract
                    deployedBlockNumber:
                      type: number
                      description: Number - The Block Number when the deployment transaction is successfully mined
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
  operationId: getContractMetadataBatch
```
