# Owners By Contract

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-contract-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-contract-v-3.md)

# Owners By Contract

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getOwnersForContract

getOwnersForContract - Retrieves all owners associated with a specific NFT contract. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-contract-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |
| withTokenBalances | boolean | No | Boolean - If set to `true` the query will include the token balances per token id for each owner. `false` by default. |
| pageKey | string | No | String - used for contracts with >50,000 owners. `pageKey` field can be passed back as request parameter to get the next page of results. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForContract?contractAddress=0x495f947276749ce646f68ac8c248420045cb7b5e");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getOwnersForContract
method: GET
operation:
  summary: Owners By Contract
  description: getOwnersForContract - Retrieves all owners associated with a specific NFT contract. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
  tags:
    - NFT Ownership Endpoints
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
        default: '0x495f947276749ce646f68ac8c248420045cb7b5e'
      required: true
    - name: withTokenBalances
      description: Boolean - If set to `true` the query will include the token balances per token id for each owner. `false` by default.
      in: query
      schema:
        type: boolean
        default: false
    - description: String - used for contracts with >50,000 owners. `pageKey` field can be passed back as request parameter to get the next page of results.
      name: pageKey
      schema:
        type: string
      in: query
  responses:
    '200':
      description: Returns a list of all owners for the specified contract.
      content:
        application/json:
          schema:
            type: object
            properties:
              owners:
                description: List of all addresses that own one of the NFTs from the queried contract address. The format is applicable when `withTokenBalances=true`.
                type: array
                items:
                  type: object
                  properties:
                    ownerAddress:
                      type: string
                      default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
                    tokenBalances:
                      type: array
                      description: a list of the token ids and balances for the owner of the collection
                      items:
                        type: object
                        properties:
                          tokenId:
                            type: string
                            description: tokenId of the NFT in the collection that an owner has
                          balance:
                            type: integer
                            description: the number of the specified token in the collection that the user owns
  operationId: getOwnersForContract-v3
```
