# reportSpam

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/report-spam.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/report-spam.md)

# reportSpam

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/reportSpam

Report a particular address to our APIs if you think it is spam

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/report-spam

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string | Yes | String - The address to check for spam status. |
| isSpam | boolean | Yes | Boolean - Whether the address is spam. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/reportSpam?address=0x495f947276749ce646f68ac8c248420045cb7b5e&isSpam=true");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/reportSpam
method: GET
operation:
  summary: reportSpam
  description: Report a particular address to our APIs if you think it is spam
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
    - name: address
      description: String - The address to check for spam status.
      in: query
      schema:
        type: string
        default: '0x495f947276749ce646f68ac8c248420045cb7b5e'
      required: true
    - name: isSpam
      description: Boolean - Whether the address is spam.
      in: query
      schema:
        type: boolean
        default: true
      required: true
  responses:
    '200':
      description: ''
      content:
        application/json:
          schema:
            type: string
            description: 'String - "Address was successfully reported as spam" if calling the API was successful. '
  operationId: reportSpam
```
