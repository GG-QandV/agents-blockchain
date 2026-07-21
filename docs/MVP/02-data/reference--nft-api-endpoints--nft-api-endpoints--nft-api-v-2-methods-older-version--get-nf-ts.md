# getNFTs

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nf-ts.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nf-ts.md)

# getNFTs

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v2/{apiKey}/getNFTs

Gets all NFTs currently owned by a given address.

Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-api-v-2-methods-older-version/get-nf-ts

## Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| apiKey | string | Yes |  |

## Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| owner | string | Yes | String - Address for NFT owner (can be in ENS format for Eth Mainnet). |
| contractAddresses[] | string[] | No | Array of contract addresses to filter the responses with. Max limit 45 contracts. |
| withMetadata | boolean | No | Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`. |
| orderBy | enum | No | Enum - ordering scheme to use for ordering NFTs in the response. If unspecified, NFTs will be ordered by contract address and token ID.   - transferTime: NFTs will be ordered by the time they were transferred into the wallet, with newest NFTs first. Note: This ordering is supported on Ethereum Mainnet, Optimism Mainnet, Polygon Mainnet, Base Mainnet, Arbitrum One, Polygon Amoy, Base Sepolia, Arbitrum Sepolia, Ethereum Sepolia, and Optimism Sepolia. |
| excludeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. NFTs that match one or more of these filters will be excluded from the response. May not be used in conjunction with includeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |
| includeFilters[] | enum[] | No | Array of filters (as ENUMS) that will be applied to the query. Only NFTs that match one or more of these filters will be included in the response. May not be used in conjunction with excludeFilters[]. Filter Options:   - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.   - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.   - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts) |
| spamConfidenceLevel | enum | No | Enum - the confidence level at which to filter spam at. Confidence Levels:   - VERY_HIGH   - HIGH   - MEDIUM   - LOW The confidence level set means that any spam that is at that confidence level or higher will be filtered out. For example, if the confidence level is HIGH, contracts that we have HIGH or VERY_HIGH confidence in being spam will be filtered out from the response.  Defaults to VERY_HIGH for Ethereum Mainnet and MEDIUM for Matic Mainnet. **Please note that this filter is only available on paid tiers. Upgrade your account [here](https://dashboard.alchemy.com/settings/billing/).** |
| tokenUriTimeoutInMs | integer | No | No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0. |
| pageKey | string | No | String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results. |
| pageSize | integer | No | Number of NFTs to be returned per page. Defaults to 100. Max is 100. |

## Code Examples

### cURL

```bash
curl --request GET \
  --url 'https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v2/docs-demo/getNFTs?owner=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v2/{apiKey}/getNFTs
method: GET
operation:
  summary: getNFTs
  description: Gets all NFTs currently owned by a given address.
  tags:
    - NFT API V2 Methods (Older Version)
  parameters:
    - name: owner
      description: String - Address for NFT owner (can be in ENS format for Eth Mainnet).
      schema:
        type: string
        default: '0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045'
      in: query
      required: true
    - name: contractAddresses[]
      description: Array of contract addresses to filter the responses with. Max limit 45 contracts.
      schema:
        type: array
        items:
          type: string
      in: query
    - name: withMetadata
      description: Boolean - if set to `true`, returns NFT metadata. Setting this to false will reduce payload size and may result in a faster API call. Defaults to `true`.
      schema:
        type: boolean
        default: true
      in: query
    - name: orderBy
      description: |-
        Enum - ordering scheme to use for ordering NFTs in the response. If unspecified, NFTs will be ordered by contract address and token ID.
          - transferTime: NFTs will be ordered by the time they were transferred into the wallet, with newest NFTs first. Note: This ordering is supported on Ethereum Mainnet, Optimism Mainnet, Polygon Mainnet, Base Mainnet, Arbitrum One, Polygon Amoy, Base Sepolia, Arbitrum Sepolia, Ethereum Sepolia, and Optimism Sepolia.
      in: query
      schema:
        type: string
        enum:
          - transferTime
      required: false
    - name: excludeFilters[]
      description: |-
        Array of filters (as ENUMS) that will be applied to the query. NFTs that match one or more of these filters will be excluded from the response. May not be used in conjunction with includeFilters[]. Filter Options:
          - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.
          - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.
          - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts)
      schema:
        type: array
        items:
          type: string
          enum:
            - SPAM
            - AIRDROPS
          default: SPAM
      in: query
    - name: includeFilters[]
      description: |-
        Array of filters (as ENUMS) that will be applied to the query. Only NFTs that match one or more of these filters will be included in the response. May not be used in conjunction with excludeFilters[]. Filter Options:
          - SPAM: NFTs that have been classified as spam. Spam classification has a wide range of criteria that includes but is not limited to emitting fake events and copying other well-known NFTs. Please note that this filter is currently supported on Mainnet for Base, Arbitrum, Optimism, Ethereum, Polygon, Worldchain, Avax, BNB, Gnosis, Zksync, Unichain, and Blast, and is **available exclusively on paid tiers**.
          - AIRDROPS: NFTs that have were airdropped to the user. Airdrops are defined as NFTs that were minted to a user address in a transaction sent by a different address. NOTE: this filter is currently supported on Ethereum Mainnet, Ethereum Goerli, and Matic Mainnet only.
          - To learn more about spam, you can refer to this: [Spam NFTs and how to fix them](https://www.alchemy.com/overviews/spam-nfts)
      schema:
        type: array
        items:
          type: string
          enum:
            - SPAM
            - AIRDROPS
          default: SPAM
      in: query
    - name: spamConfidenceLevel
      description: |-
        Enum - the confidence level at which to filter spam at.

        Confidence Levels:
          - VERY_HIGH
          - HIGH
          - MEDIUM
          - LOW

        > 📄 **This content also appears in [getContractsForOwner](02-data/reference--nft-api-endpoints--nft-api-endpoints--nft-api-v-2-methods-older-version--get-contracts-for-owner.md)** — see there for full details.

        **Please note that this filter is only available on paid tiers. Upgrade your account [here](https://dashboard.alchemy.com/settings/billing/).**
      schema:
        type: string
        enum:
          - VERY_HIGH
          - HIGH
          - MEDIUM
          - LOW
      in: query
      required: false
    - name: tokenUriTimeoutInMs
      description: No set timeout by default - When metadata is requested, this parameter is the timeout (in milliseconds) for the website hosting the metadata to respond. If you want to _only_ access the cache and not live fetch any metadata for cache misses then set this value to 0.
      in: query
      schema:
        type: integer
    - name: apiKey
      in: path
      schema:
        type: string
        default: docs-demo
        description: For higher throughput, [create your own API key](https://dashboard.alchemy.com/signup)
      required: true
    - name: pageKey
      description: String - key for pagination. If more results are available, a pageKey will be returned in the response. Pass back the pageKey as a param to fetch the next page of results.
      schema:
        type: string
      in: query
    - name: pageSize
      description: Number of NFTs to be returned per page. Defaults to 100. Max is 100.
      schema:
        type: integer
        default: 100
      in: query
  responses:
    '200':
      description: Returns the list of all NFTs owned by the given address and satisfying the given input parameters.
      content:
        application/json:
          schema:
            type: object
            properties:
              ownedNfts:
                type: array
                items:
                  type: object
                  description: The object that represents an NFT and has all data corresponding to that NFT
                  properties:
                    contract:
                      description: Object - Contract for returned NFT
                      type: object
                      properties:
                        address:
                          type: string
                          description: String - Address of NFT contract.
                    id:
                      type: object
                      properties:
                        tokenId:
                          type: string
                          default: '44'
                        tokenMetadata:
                          type: object
                          properties:
                            tokenType:
                              type: string
                              enum:
                                - ERC721
                                - ERC1155
                                - NO_SUPPORTED_NFT_STANDARD
                                - NOT_A_CONTRACT
                              description: String - For valid NFTs, 'ERC721' or 'ERC1155.' For invalid NFTs, a descriptive reason such as 'NO_SUPPORTED_NFT_STANDARD' if the input contract address doesn't support a known NFT standard, or 'NOT_A_CONTRACT' if there is no contract deployed at the input address.
                    balance:
                      type: string
                      description: String - Token balance
                    title:
                      type: string
                      description: String - Name of the NFT asset.
                    description:
                      type: string
                      description: String - Brief human-readable description
                    tokenUri:
                      type: object
                      properties:
                        raw:
                          type: string
                          description: String - Uri representing the location of the NFT's original metadata blob. This is a backup for you to parse when the metadata field is not automatically populated.
                        gateway:
                          type: string
                          description: String - Public gateway uri for the raw uri above.
                    media:
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
                    metadata:
                      type: object
                      description: Relevant metadata for NFT contract. This is useful for viewing image url, traits, etc. without having to follow the metadata url in tokenUri to parse manually.
                      properties:
                        image:
                          type: string
                          description: String - URL to the NFT asset image. Can be standard URLs pointing to images on conventional servers, IPFS, or Arweave. Most types of images (SVGs, PNGs, JPEGs, etc.) are supported by NFT marketplaces.
                        external_url:
                          type: string
                          description: String - The image URL that appears alongside the asset image on NFT platforms.
                        background_color:
                          type: string
                          description: String - Background color of the NFT item. Usually must be defined as a six-character hexadecimal.
                        name:
                          type: string
                          description: String - Name of the NFT asset.
                        description:
                          type: string
                          description: String - Human-readable description of the NFT asset. (Markdown is supported/rendered on OpenSea and other NFT platforms)
                        attributes:
                          type: array
                          items:
                            type: object
                            properties:
                              value:
                                type: string
                              trait_type:
                                type: string
                          description: Object - Traits/attributes/characteristics for each NFT asset.
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
                    timeLastUpdated:
                      type: string
                      description: String - ISO timestamp of the last cache refresh for the information returned in the metadata field.
                    error:
                      type: string
                      description: String - A string describing a particular reason that we were unable to fetch complete metadata for the NFT.
                    contractMetadata:
                      type: object
                      properties:
                        name:
                          type: string
                          description: String - NFT contract name.
                        symbol:
                          type: string
                          description: String - NFT contract symbol abbreviation.
                        totalSupply:
                          type: string
                          description: String - Total number of NFTs in a given NFT collection.
                        tokenType:
                          type: string
                          enum:
                            - ERC721
                            - ERC1155
                            - NO_SUPPORTED_NFT_STANDARD
                            - NOT_A_CONTRACT
                          description: String - For valid NFTs, 'ERC721' or 'ERC1155.' For invalid NFTs, a descriptive reason such as 'NO_SUPPORTED_NFT_STANDARD' if the input contract address doesn't support a known NFT standard, or 'NOT_A_CONTRACT' if there is no contract deployed at the input address.
                        contractDeployer:
                          type: string
                          description: String - Address that deployed the smart contract
                        deployedBlockNumber:
                          type: number
                          description: Number - The Block Number when the deployment transaction is successfully mined
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
                    spamInfo:
                      type: object
                      description: Information about whether and why a contract was marked as spam.
                      properties:
                        isSpam:
                          type: string
                          description: '"true" if contract is spam, else "false". **Only available on paid tiers.**'
                        spamClassifications:
                          description: List of reasons why a contract was classified as spam. **Only available on paid tiers.**
                          type: array
                          items:
                            type: string
                    acquiredAt:
                      type: object
                      description: Only present if the request specified `orderBy=transferTime`.
                      properties:
                        blockTimestamp:
                          type: string
                          description: Block timestamp of the block where the NFT was most recently acquired.
                        blockNumber:
                          type: string
                          description: Block number of the block where the NFT was most recently acquired.
              pageKey:
                type: string
              totalCount:
                type: integer
                description: Integer - Total number of NFTs (distinct `tokenIds`) owned by the given address.
              blockHash:
                type: string
                description: String - The canonical head block hash of when your request was received i.e. the block corresponding to `latest`
          examples:
            byDefault:
              summary: Response (By Default)
              value:
                ownedNfts:
                  - contract:
                      address: '0x0beed7099af7514ccedf642cfea435731176fb02'
                    id:
                      tokenId: '28'
                      tokenMetadata:
                        tokenType: ERC721
                    title: 'DuskBreaker #28'
                    description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                    tokenUri:
                      raw: https://duskbreakers.gg/api/breakers/28
                      gateway: https://duskbreakers.gg/api/breakers/28
                    media:
                      - raw: https://duskbreakers.gg/breaker_images/28.png
                        gateway: https://duskbreakers.gg/breaker_images/28.png
                    metadata:
                      name: 'DuskBreaker #28'
                      description: Breakers have the honor of serving humanity through their work on The Dusk. They are part of a select squad of 10,000 recruits who spend their days exploring a mysterious alien spaceship filled with friends, foes, and otherworldly technology.
                      image: https://duskbreakers.gg/breaker_images/28.png
                      external_url: https://duskbreakers.gg
                      attributes:
                        - value: Locust Rider Armor (Red)
                          trait_type: Clothes
                        - value: Big Smile (Purple)
                          trait_type: Mouth
                        - value: Yellow
                          trait_type: Background
                    timeLastUpdated: '2022-02-16T22:52:54.719Z'
                    contractMetadata:
                      name: DuskBreakers
                      symbol: DUSK
                      totalSupply: '10000'
                      tokenType: ERC721
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '5527'
                      tokenMetadata:
                        tokenType: ERC721
                    title: 'Runner #5527'
                    description: Chain Runners are Mega City renegades 100% generated on chain.
                    tokenUri:
                      raw: https://api.chainrunners.xyz/tokens/metadata/5527?dna=73247164192459371523281785218958151913554625578441142916970699984935810987041
                      gateway: https://api.chainrunners.xyz/tokens/metadata/5527?dna=73247164192459371523281785218958151913554625578441142916970699984935810987041
                    media:
                      - raw: https://img.chainrunners.xyz/api/v1/tokens/png/5527
                        gateway: https://img.chainrunners.xyz/api/v1/tokens/png/5527
                    metadata:
                      name: 'Runner #5527'
                      description: Chain Runners are Mega City renegades 100% generated on chain.
                      image: https://img.chainrunners.xyz/api/v1/tokens/png/5527
                      attributes:
                        - value: Purple Green Diag
                          trait_type: Background
                        - value: Human
                          trait_type: Race
                        - value: Cig
                          trait_type: Mouth Accessory
                    timeLastUpdated: '2022-02-18T00:42:04.401Z'
                    contractMetadata:
                      name: Chain Runners
                      symbol: RUN
                      totalSupply: '10000'
                      tokenType: ERC721
                totalCount: 6
                blockHash: '0xeb2d26af5b6175344a14091777535a2cb21c681665a734a8285f889981987630'
            withoutMetadata:
              summary: Response (withMetadata = false)
              value:
                ownedNfts:
                  - contract:
                      address: '0x0beed7099af7514ccedf642cfea435731176fb02'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000001c'
                  - contract:
                      address: '0x0beed7099af7514ccedf642cfea435731176fb02'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000001d'
                    balance: '1'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x0000000000000000000000000000000000000000000000000000000000001597'
                    balance: '1'
                totalCount: 6
                blockHash: '0xf9a2a4e15116680e22b160c734529f62d89d54cde0759daf5135672fad0ecebc'
            withContractFiltering:
              summary: Response (with contract filtering)
              value:
                ownedNfts:
                  - contract:
                      address: '0x34d77a17038491a2a9eaa6e690b7c7cd39fc8392'
                    id:
                      tokenId: '0x0000000000000000000000000000000000000000000000000000000000000277'
                totalCount: 1
                blockHash: '0x3d8bca59c08e41f55d46ebbe738327eb12955cf280bd06ef7d40352919c188d8'
            withPagination:
              summary: Response (with pagination)
              value:
                ownedNfts:
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x00000000000000000000000000000000000000000000000000000000000009cb'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x00000000000000000000000000000000000000000000000000000000000009cc'
                  - contract:
                      address: '0x5ab21ec0bfa0b29545230395e3adaca7d552c948'
                    id:
                      tokenId: '0x00000000000000000000000000000000000000000000000000000000000006dc'
                  - contract:
                      address: '0x3b3ee1931dc30c1957379fac9aba94d1c48a5405'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000001a'
                  - contract:
                      address: '0x69c40e500b84660cb2ab09cb9614fa2387f95f64'
                    id:
                      tokenId: '0x0000000000000000000000000000000000000000000000000000000000000391'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x00000000000000000000000000000000000000000000000000000000000008d5'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x0000000000000000000000000000000000000000000000000000000000000a1d'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000002a'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000038e'
                  - contract:
                      address: '0x97597002980134bea46250aa0510c9b90d87a587'
                    id:
                      tokenId: '0x000000000000000000000000000000000000000000000000000000000000244b'
                pageKey: 88434286-7eaa-472d-8739-32a0497c2a18
                totalCount: 277
                blockHash: '0x94d5ab52b8a6571733f6b183ef89f31573b82a4e78f8129b0ce90ef0beaf208b'
  operationId: getNFTs
```
