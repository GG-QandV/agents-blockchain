# alchemy_getAssetTransfers

> Source: [https://www.alchemy.com/docs/data/transfers-api/transfers-endpoints/alchemy-get-asset-transfers.md](https://www.alchemy.com/docs/data/transfers-api/transfers-endpoints/alchemy-get-asset-transfers.md)

# alchemy_getAssetTransfers

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

POST https://eth-mainnet.g.alchemy.com/v2/{apiKey}

The Transfers API allows you to easily fetch historical transactions for any address across Ethereum and supported L2s including Base, Polygon, Arbitrum, and Optimism (internal transfer data is only available on Ethereum Mainnet and Polygon Mainnet).

Reference: https://www.alchemy.com/docs/data/transfers-api/transfers-endpoints/alchemy-get-asset-transfers

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| assetTransferParams | object | Yes |  |

## Result

**Asset transfers** (string or object): Returns the list of transfers, and a pageKey if additional results remain.

## Example

### Request

```json
{
  "jsonrpc": "2.0",
  "method": "alchemy_getAssetTransfers",
  "params": [
    {
      "fromBlock": "0x0",
      "fromAddress": "0x0000000000000000000000000000000000000000",
      "toAddress": "0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1",
      "excludeZeroValue": true,
      "withMetadata": true,
      "category": [
        "erc721",
        "erc1155"
      ]
    }
  ],
  "id": 1
}
```

### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "transfers": [
      {
        "blockNum": "0xb0eadc",
        "uniqueId": "0x3847245c01829b043431067fb2bfa95f7b5bdc7e4246c843e7a573ab6f26f5ff:external",
        "hash": "0x3847245c01829b043431067fb2bfa95f7b5bdc7e4246c843e7a573ab6f26f5ff",
        "from": "0xef4396d9ff8107086d215a1c9f8866c54795d7c7",
        "to": "0x5c43b1ed97e52d009611d89b74fa829fe4ac56b1",
        "value": 0.5,
        "erc721TokenId": null,
        "erc1155Metadata": null,
        "tokenId": null,
        "asset": "ETH",
        "category": "external",
        "rawContract": {
          "value": "0x6f05b59d3b20000",
          "address": null,
          "decimal": "0x12"
        }
      },
      {
        "blockNum": "0xb96042",
        "uniqueId": "0x5c88806ce2e4a42c5fbd5804f340ed887995914546cf92ec39eb5472cf22c88c:external",
        "hash": "0x5c88806ce2e4a42c5fbd5804f340ed887995914546cf92ec39eb5472cf22c88c",
        "from": "0xef4396d9ff8107086d215a1c9f8866c54795d7c7",
        "to": "0x5c43b1ed97e52d009611d89b74fa829fe4ac56b1",
        "value": 0.27,
        "erc721TokenId": null,
        "erc1155Metadata": null,
        "tokenId": null,
        "asset": "ETH",
        "category": "external",
        "rawContract": {
          "value": "0x3bf3b91c95b0000",
          "address": null,
          "decimal": "0x12"
        }
      }
    ],
    "pageKey": ""
  },
  "id": 1
}
```

## Code Examples

### cURL

```bash
curl --request POST \
  --url https://eth-mainnet.g.alchemy.com/v2/docs-demo \
  --header 'Content-Type: application/json' \
  --data '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "alchemy_getAssetTransfers",
  "params": [
    {
      "fromBlock": "0x0",
      "fromAddress": "0x0000000000000000000000000000000000000000",
      "toAddress": "0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1",
      "excludeZeroValue": true,
      "withMetadata": true,
      "category": [
        "erc721",
        "erc1155"
      ]
    }
  ]
}'
```

### JavaScript

```javascript
const options = {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    jsonrpc: '2.0',
    id: 1,
    method: 'alchemy_getAssetTransfers',
    params: [
      {
        fromBlock: '0x0',
        fromAddress: '0x0000000000000000000000000000000000000000',
        toAddress: '0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1',
        excludeZeroValue: true,
        withMetadata: true,
        category: ['erc721', 'erc1155']
      }
    ]
  })
};

fetch('https://eth-mainnet.g.alchemy.com/v2/docs-demo', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "alchemy_getAssetTransfers",
    "params": [
        {
            "fromBlock": "0x0",
            "fromAddress": "0x0000000000000000000000000000000000000000",
            "toAddress": "0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1",
            "excludeZeroValue": True,
            "withMetadata": True,
            "category": ["erc721", "erc1155"]
        }
    ]
}
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

	url := "https://eth-mainnet.g.alchemy.com/v2/docs-demo"

	payload := strings.NewReader("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getAssetTransfers\",\n  \"params\": [\n    {\n      \"fromBlock\": \"0x0\",\n      \"fromAddress\": \"0x0000000000000000000000000000000000000000\",\n      \"toAddress\": \"0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1\",\n      \"excludeZeroValue\": true,\n      \"withMetadata\": true,\n      \"category\": [\n        \"erc721\",\n        \"erc1155\"\n      ]\n    }\n  ]\n}")

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
HttpResponse<String> response = Unirest.post("https://eth-mainnet.g.alchemy.com/v2/docs-demo")
  .header("Content-Type", "application/json")
  .body("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getAssetTransfers\",\n  \"params\": [\n    {\n      \"fromBlock\": \"0x0\",\n      \"fromAddress\": \"0x0000000000000000000000000000000000000000\",\n      \"toAddress\": \"0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1\",\n      \"excludeZeroValue\": true,\n      \"withMetadata\": true,\n      \"category\": [\n        \"erc721\",\n        \"erc1155\"\n      ]\n    }\n  ]\n}")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/v2/docs-demo");
var client = new RestClient(options);
var request = new RestRequest("");
request.AddJsonBody("{\n  \"jsonrpc\": \"2.0\",\n  \"id\": 1,\n  \"method\": \"alchemy_getAssetTransfers\",\n  \"params\": [\n    {\n      \"fromBlock\": \"0x0\",\n      \"fromAddress\": \"0x0000000000000000000000000000000000000000\",\n      \"toAddress\": \"0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1\",\n      \"excludeZeroValue\": true,\n      \"withMetadata\": true,\n      \"category\": [\n        \"erc721\",\n        \"erc1155\"\n      ]\n    }\n  ]\n}", false);
var response = await client.PostAsync(request);

Console.WriteLine("{0}", response.Content);

```


## OpenRPC Method Specification

```yaml
name: alchemy_getAssetTransfers
description: The Transfers API allows you to easily fetch historical transactions for any address across Ethereum and supported L2s including Base, Polygon, Arbitrum, and Optimism (internal transfer data is only available on Ethereum Mainnet and Polygon Mainnet).
x-compute-units: 120
params:
  - name: assetTransferParams
    required: true
    schema:
      type: object
      properties:
        fromBlock:
          type: string
          default: '0x0'
        toBlock:
          type: string
          default: latest
        fromAddress:
          type: string
          default: '0x0000000000000000000000000000000000000000'
        toAddress:
          type: string
          default: '0x0000000000000000000000000000000000000000'
        excludeZeroValue:
          type: boolean
          default: true
        category:
          type: array
          description: 'Array of transfer categories to include. Note: ''internal'' category is not supported on Base, it is only available on Ethereum Mainnet and Polygon Mainnet.'
          items:
            type: string
            enum:
              - external
              - internal
              - erc20
              - erc721
              - erc1155
              - specialnft
        contractAddresses:
          type: array
          description: An array of contract addresses to filter for.
          items:
            type: string
        order:
          type: string
          enum:
            - asc
            - desc
        withMetadata:
          type: boolean
          description: Available only on ETH, Base, Polygon, Arbitrum, Optimism
          default: false
        maxCount:
          type: string
          default: '0x3e8'
        pageKey:
          type: string
          default: '0x0'
result:
  name: Asset transfers
  description: Returns the list of transfers, and a pageKey if additional results remain.
  schema:
    oneOf:
      - title: Not Found (null)
        type: string
      - type: object
        properties:
          pageKey:
            type: string
            description: Uuid of next page of results (if exists, else blank).
          transfers:
            type: array
            description: Array of transfer objects sorted in ascending or descending order by block number.
            items:
              type: object
              properties:
                category:
                  type: string
                  description: '''external'', ''internal'', ''token'', ''erc20'', ''erc721'', ''erc1155'', ''specialnft'''
                blockNum:
                  type: string
                  description: Block number of the transfer (hex string).
                from:
                  type: string
                  description: From address (hex string).
                to:
                  type: string
                  description: To address (hex string). null if contract creation.
                value:
                  type: number
                  nullable: true
                  description: Asset transfer value. null if it's an ERC721 or unknown decimals.
                erc721TokenId:
                  type: string
                  nullable: true
                  description: (Deprecated) Legacy token ID field for ERC721 tokens. Use `tokenId` instead.
                erc1155Metadata:
                  type: array
                  nullable: true
                  description: Array of objects with (tokenId, value) for ERC1155 transfers, null otherwise.
                  items:
                    type: object
                    properties:
                      tokenId:
                        type: string
                      value:
                        type: string
                tokenId:
                  type: string
                  description: Token ID for NFT tokens (ERC721, ERC1155, etc.).
                asset:
                  type: string
                  nullable: true
                  description: ETH or the token's symbol. null if unavailable.
                uniqueId:
                  type: string
                  description: Unique identifier for the transfer object.
                hash:
                  type: string
                  description: Transaction hash (hex string).
                rawContract:
                  type: object
                  properties:
                    value:
                      type: string
                      nullable: true
                      description: Raw hex transfer value. null for NFT transfers.
                    address:
                      type: string
                      nullable: true
                      description: Contract address (hex string). null for external or internal transfers.
                    decimal:
                      type: string
                      nullable: true
                      description: Contract decimal in hex. null if not known.
                metadata:
                  type: object
                  properties:
                    blockTimestamp:
                      type: string
                      description: ISO-formatted timestamp of the block containing this transfer. (Available only on ETH, Base, Polygon, Arbitrum, Optimism)
examples:
  - name: alchemy_getAssetTransfers example
    params:
      - name: assetTransferParams
        value:
          fromBlock: '0x0'
          fromAddress: '0x0000000000000000000000000000000000000000'
          toAddress: '0x5c43B1eD97e52d009611D89b74fA829FE4ac56b1'
          excludeZeroValue: true
          withMetadata: true
          category:
            - erc721
            - erc1155
    result:
      name: Asset transfers
      value:
        transfers:
          - blockNum: '0xb0eadc'
            uniqueId: 0x3847245c01829b043431067fb2bfa95f7b5bdc7e4246c843e7a573ab6f26f5ff:external
            hash: '0x3847245c01829b043431067fb2bfa95f7b5bdc7e4246c843e7a573ab6f26f5ff'
            from: '0xef4396d9ff8107086d215a1c9f8866c54795d7c7'
            to: '0x5c43b1ed97e52d009611d89b74fa829fe4ac56b1'
            value: 0.5
            erc721TokenId: null
            erc1155Metadata: null
            tokenId: null
            asset: ETH
            category: external
            rawContract:
              value: '0x6f05b59d3b20000'
              address: null
              decimal: '0x12'
          - blockNum: '0xb96042'
            uniqueId: 0x5c88806ce2e4a42c5fbd5804f340ed887995914546cf92ec39eb5472cf22c88c:external
            hash: '0x5c88806ce2e4a42c5fbd5804f340ed887995914546cf92ec39eb5472cf22c88c'
            from: '0xef4396d9ff8107086d215a1c9f8866c54795d7c7'
            to: '0x5c43b1ed97e52d009611d89b74fa829fe4ac56b1'
            value: 0.27
            erc721TokenId: null
            erc1155Metadata: null
            tokenId: null
            asset: ETH
            category: external
            rawContract:
              value: '0x3bf3b91c95b0000'
              address: null
              decimal: '0x12'
        pageKey: ''
```
