# Is Holder Of Contract

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/is-holder-of-contract-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/is-holder-of-contract-v-3.md)

# Is Holder Of Contract

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/isHolderOfContract

isHolderOfContract - Determines whether a specific wallet holds any NFT from a given contract. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-ownership-endpoints/is-holder-of-contract-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| wallet | string | Yes | String - Wallet address to check for contract ownership. |
| contractAddress | string | Yes | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/isHolderOfContract?wallet=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/isHolderOfContract
method: GET
operation:
  summary: Is Holder Of Contract
  description: isHolderOfContract - Determines whether a specific wallet holds any NFT from a given contract. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
    - name: wallet
      description: String - Wallet address to check for contract ownership.
      schema:
        type: string
        default: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
      in: query
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
      description: Returns whether the specified wallet holds any NFT from the given contract.
      content:
        application/json:
          schema:
            type: object
            description: Data related to a wallet's ownership of any token in an NFT contract.
            properties:
              isHolderOfContract:
                type: boolean
                description: Whether the given wallet owns any token in the given NFT contract.
  operationId: isHolderOfContract-v3
```
