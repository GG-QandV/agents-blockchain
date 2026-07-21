# Owners By NFT

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-nft-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-nft-v-3.md)

# Owners By NFT

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getOwnersForNFT

getOwnersForNFT - Retrieves the owner(s) for a specific token. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/get-owners-for-nft-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

> 📄 **This content also appears in [computeRarity](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--compute-rarity.md)** — see there for full details.

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getOwnersForNFT?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330&tokenId=44");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getOwnersForNFT
method: GET
operation:
  summary: Owners By NFT
  description: getOwnersForNFT - Retrieves the owner(s) for a specific token. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
        default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
      required: true
    - name: tokenId
      description: String - The ID of the token. Can be in hex or decimal format.
      in: query
      schema:
        type: string
        default: '44'
      required: true
  responses:
    '200':
      description: Returns the owner(s) of the specified NFT.
      content:
        application/json:
          schema:
            type: object
            properties:
              owners:
                type: array
                description: List of all addresses that own the given NFT.
                items:
                  type: string
              pageKey:
                type: string
          examples:
            getOwnersForNFT_response:
              value:
                owners:
                  - '0x9f4F78A6c4a5E6F8AFA81631b9120ae3C831b494'
                pageKey: null
  operationId: getOwnersForNFT-v3
```
