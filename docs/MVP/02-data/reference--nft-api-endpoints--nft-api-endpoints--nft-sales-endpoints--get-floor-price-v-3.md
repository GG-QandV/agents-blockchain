# Floor Prices By Slug

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-sales-endpoints/get-floor-price-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-sales-endpoints/get-floor-price-v-3.md)

# Floor Prices By Slug

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getFloorPrice

Retrieves the floor prices of an NFT collection across different marketplaces.

<Note>Please note that this endpoint is only available on Ethereum mainnet for Opensea & Looksrare marketplaces.</Note>


Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-sales-endpoints/get-floor-price-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | Yes | String - Contract address for the NFT contract (ERC721 and ERC1155 supported). |
| collectionSlug | string | No | String - OpenSea slug for the NFT collection. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getFloorPrice?contractAddress=0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getFloorPrice
method: GET
operation:
  summary: Floor Prices By Slug
  description: |
    Retrieves the floor prices of an NFT collection across different marketplaces.

    <Note>Please note that this endpoint is only available on Ethereum mainnet for Opensea & Looksrare marketplaces.</Note>
  tags:
    - NFT Sales Endpoints
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
        default: '0x1f02bf9dde7c79137a08b2dd4fc964bfd2499734'
      required: true
    - name: collectionSlug
      description: String - OpenSea slug for the NFT collection.
      in: query
      schema:
        type: string
        default: boredapeyachtclub
      required: false
  responses:
    '200':
      description: Returns the floor prices of the specified NFT collection across different marketplaces.
      content:
        application/json:
          schema:
            type: object
            properties:
              nftMarketplaceName:
                type: object
                description: Name of the NFT marketplace where the collection is listed (in camel case). Current marketplaces supported - `openSea`, `looksRare`. So instead of the word `nftMarketplaceName` you will see marketplace names like `openSea` here.
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
                    description: String - Returns the error `unable to fetch floor price` if there was an error fetching floor prices from the given marketplace.
  operationId: getFloorPrice-v3
```
