# getSpamContracts

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-spam-contracts.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-spam-contracts.md)

# getSpamContracts

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getSpamContracts

Returns a list of all spam contracts marked by Alchemy.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-spam-contracts

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Code Examples

### cURL

```bash
curl --request GET \
  --url https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getSpamContracts");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getSpamContracts
method: GET
operation:
  summary: getSpamContracts
  description: Returns a list of all spam contracts marked by Alchemy.
  tags:
    - NFT API V2 Methods (Older Version)
  servers:
    - url: https://{network}.g.alchemy.com/nft
      variables:
        network:
          enum:
            - eth-mainnet
            - polygon-mainnet
          default: eth-mainnet
  parameters:
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: object
            description: Object that has list of contract addresses
            properties:
              contractAddresses:
                type: array
                items:
                  type: string
                description: A list of contract addresses earmarked as spam by Alchemy.
  operationId: getSpamContracts
```
