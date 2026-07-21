# Collection Metadata By Slug

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-collection-metadata-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-collection-metadata-v-3.md)

# Collection Metadata By Slug

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/getCollectionMetadata

getCollectionMetadata - Retrieves high-level collection or contract-level information for an NFT collection. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/get-collection-metadata-v-3

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| collectionSlug | string | Yes | String - OpenSea slug for the NFT collection. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/getCollectionMetadata?collectionSlug=boredapeyachtclub");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/getCollectionMetadata
method: GET
operation:
  summary: Collection Metadata By Slug
  description: getCollectionMetadata - Retrieves high-level collection or contract-level information for an NFT collection. This endpoint is supported on Ethereum and many L2s, including Polygon, Arbitrum, Optimism, Base, World Chain and more. See the full list of supported networks [here](https://dashboard.alchemy.com/chains).
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
    - name: collectionSlug
      description: String - OpenSea slug for the NFT collection.
      in: query
      schema:
        type: string
        default: boredapeyachtclub
      required: true
  responses:
    '200':
      description: Returns the collection metadata for the specified slug.
      content:
        application/json:
          schema:
            type: object
            properties:
              name:
                type: string
                description: String - Name of the queried NFT Collection
              slug:
                description: The human-readable string used to identify the collection on OpenSea.
                type: string
              floorPrice:
                type: object
                description: Floor price data for the collection
                properties:
                  marketplace:
                    description: The marketplace the floor price is on
                    type: string
                  floorPrice:
                    description: Floor price of the collection on the marketplace
                    type: number
                  priceCurrency:
                    description: The currency of the floor price
                    type: string
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
  operationId: getCollectionMetadata-v3
```
