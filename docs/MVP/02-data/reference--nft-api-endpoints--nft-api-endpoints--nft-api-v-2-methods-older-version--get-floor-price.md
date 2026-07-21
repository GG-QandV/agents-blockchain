# getFloorPrice

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-floor-price.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-floor-price.md)

# getFloorPrice

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getFloorPrice

Returns the floor prices of a NFT collection by marketplace.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-floor-price

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
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getFloorPrice?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getFloorPrice
method: GET
operation:
  summary: getFloorPrice
  description: Returns the floor prices of a NFT collection by marketplace.
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
    - name: contractAddress
      description: String - Contract address for the NFT contract (ERC721 and ERC1155 supported).
      in: query
      schema:
        type: string
        default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
      required: true
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: object
            properties:
              nftMarketplace:
                type: object
                description: Name of the NFT marketplace where the collection is listed. Current marketplaces supported -  OpenSea, LooksRare
                properties:
                  floorPrice:
                    type: number
                    description: Number - The floor price of the collection on the given marketplace.
                  priceCurrency:
                    type: string
                    description: String - The currency in which the floor price is denominated. Typically, denominated in ETH
                    enum:
                      - ETH
                  collectionUrl:
                    type: string
                    description: String - Link to the collection on the given marketplace.
                  retrievedAt:
                    type: string
                    description: String - UTC timestamp of when the floor price was retrieved from the marketplace.
                  error:
                    type: string
                    description: String - Returns an error if there was an error fetching floor prices from the given marketplace.
  operationId: getFloorPrice
```
