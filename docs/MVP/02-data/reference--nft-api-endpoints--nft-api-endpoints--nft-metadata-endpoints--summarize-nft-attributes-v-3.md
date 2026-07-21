# Attributes Summary By Contract

> Source: [https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/summarize-nft-attributes-v-3.md](https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/summarize-nft-attributes-v-3.md)

# Attributes Summary By Contract

> For the complete documentation index, see [llms.txt](/docs/llms.txt).

GET https://eth-mainnet.g.alchemy.com/nft/v3/{apiKey}/summarizeNFTAttributes

Generates a summary of attribute prevalence for a specific NFT collection.

<Note>Please note that this endpoint is only available on Ethereum (mainnet) & Polygon (mainnet & mumbai).</Note>


Reference: https://www.alchemy.com/docs/reference/nft-api-endpoints/nft-api-endpoints/nft-metadata-endpoints/summarize-nft-attributes-v-3

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
  --url 'https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330'
```

### JavaScript

```javascript
const options = {method: 'GET'};

fetch('https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330', options)
  .then(res => res.json())
  .then(res => console.log(res))
  .catch(err => console.error(err));
```

### Python

```python
import requests

url = "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

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

	url := "https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330"

	req, _ := http.NewRequest("GET", url, nil)

	res, _ := http.DefaultClient.Do(req)

	defer res.Body.Close()
	body, _ := io.ReadAll(res.Body)

	fmt.Println(string(body))

}
```

### Java

```java
HttpResponse<String> response = Unirest.get("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330")
  .asString();
```

### C#

```csharp
using RestSharp;


var options = new RestClientOptions("https://eth-mainnet.g.alchemy.com/nft/v3/docs-demo/summarizeNFTAttributes?contractAddress=0xe785E82358879F061BC3dcAC6f0444462D4b5330");
var client = new RestClient(options);
var request = new RestRequest("");
var response = await client.GetAsync(request);

Console.WriteLine("{0}", response.Content);

```


## Operation Specification

```yaml
path: /v3/{apiKey}/summarizeNFTAttributes
method: GET
operation:
  summary: Attributes Summary By Contract
  description: |
    Generates a summary of attribute prevalence for a specific NFT collection.

    <Note>Please note that this endpoint is only available on Ethereum (mainnet) & Polygon (mainnet & mumbai).</Note>
  tags:
    - NFT Metadata Endpoints
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
    - name: contractAddress
      description: String - Contract address for the NFT contract (ERC721 and ERC1155 supported).
      in: query
      schema:
        type: string
        default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
      required: true
  responses:
    '200':
      description: Returns a summary of attribute prevalence for the specified NFT collection.
      content:
        application/json:
          schema:
            type: object
            description: Prevalence counts for each attribute within a collection.
            properties:
              totalSupply:
                type: string
                description: String - Total number of NFTs in a given NFT collection.
              summary:
                type: object
                description: Object mapping trait types to the prevalence of each trait within that type.
              contractAddress:
                type: string
                default: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
          examples:
            summarizeNFTAttributes_response:
              value:
                summary:
                  Earrings:
                    WoW Coins: 437
                    Pizza Lovers: 188
                    Lucky Charms: 415
                    White Ovals: 210
                    Artist Palettes: 21
                    Queen's Emeralds: 206
                    Silver Drops: 822
                    Flower Power: 366
                    Pearls: 833
                    Spikes: 776
                    Yam's Fave: 381
                    Classic Hoops: 780
                    Ocean Hoops: 770
                    Triple Rings: 823
                    60s Fantasy: 399
                    Lightning Bolts: 226
                    Empresses Of Darkness: 103
                  Necklace:
                    WoW Coin: 481
                    Malka: 494
                    Amazonite Energy: 490
                    Satin Choker: 721
                    Back To The 90s: 706
                    Empress of Darkness: 58
                    Spike Choker: 449
                    Golden Bib: 251
                    Golden Flakes: 710
                    Art Lover: 29
                    Rainbow: 474
                    Gold Ruler: 477
                    Wolf Pendant: 229
                    Tutti Frutti Beads: 691
                    Sun Keeper: 730
                  Eyes:
                    Purple To The Left: 158
                    Heterochromia To The Left: 57
                    Brown To The Right: 455
                    Black Eye Roll: 881
                    Yellow To The Left: 141
                    Purple Eye Roll: 145
                    Green Straight: 433
                    Blue To The Left: 407
                    Green To The Right: 410
                    Black Straight: 794
                    Purple To The Right: 145
                    Black To The Right: 870
                    Green Eye Roll: 413
                    Yellow Straight: 128
                    Brown To The Left: 465
                    Brown Eye Roll: 416
                    Heterochromia To The Right: 76
                    Blue Straight: 415
                    Black To The Left: 877
                    Heterochromia Eye Roll: 85
                    Purple Straight: 158
                    Brown Straight: 434
                    Yellow Eye Roll: 141
                    Heterochromia Straight: 77
                    Yellow To The Right: 139
                    Blue To The Right: 416
                    Green To The Left: 424
                    Blue Eye Roll: 440
                  Background:
                    Green Purple: 905
                    Purple Pink: 905
                    Dark Emerald: 924
                    Yellow Pink: 896
                    Pink Pastel: 849
                    Blue Green: 924
                    Soft Purple: 983
                    Green Orange: 907
                    Dark Purple: 876
                    Red Turquoise: 914
                    Orange Yellow: 917
                  Mouth:
                    Cigarette: 502
                    Whistle: 868
                    Slight Smile: 1666
                    Stern: 1733
                    Countryside: 927
                    Huh: 506
                    Slightly Open: 1661
                    Bubble Gum: 404
                    Surprised: 1733
                  Clothes:
                    80s Silk Shirt: 400
                    70s Shirt: 421
                    Fantasy Shirt: 542
                    Adventurer: 583
                    Striped Tee: 567
                    Naiade: 98
                    Tunic: 193
                    Checkmate: 396
                    Painter's Overall: 550
                    Witch Dress: 198
                    Little Red Dress: 437
                    Cabaret Corset: 535
                    Polka Dot Top: 573
                    Freedom Is Power Tee: 368
                    Warrior Armor: 177
                    Emerald Elven Cape: 117
                    Faux Fur Coat: 404
                    Red Leather Jacket: 374
                    White Tee: 533
                    Tuxedo: 100
                    Steampunk Octopus Top: 186
                    Queen's Dress: 391
                    Cherry Tee: 590
                    NFT Goddesses Top: 189
                    Gala Dress: 192
                    Psychedelic Dress: 492
                    Futuristic Dress: 394
                  Facial Features:
                    Nose Piercing: 598
                    Red Eyeliner: 608
                    Leader: 224
                    Neck Tattoo: 227
                    Pearl Eyes: 207
                    Red Blue Bolt: 97
                    Rose Tattoo: 286
                    Feline Eyes: 590
                    Elven Warrior: 99
                    Marilyn: 633
                    Freckles: 581
                    Flashy Blue: 304
                    Sunset: 297
                    Heart Tattoo: 591
                    Rainbow: 578
                    Eyebrow Tattoo MMXXI: 303
                    Eye Scar: 308
                    Treble Bass Clef Tattoo: 210
                    Crystal Queen: 221
                    Antoinette: 582
                    Cyber Warrior: 120
                    Eyebrow Piercing: 619
                    Claw Scar: 236
                  Hairstyle:
                    Badass Bob: 178
                    Curly Ponytail: 390
                    Finger Waves: 398
                    Colorful: 186
                    Fuchsia: 562
                    Retro: 408
                    Royal: 227
                    Boy Cut: 566
                    Bob: 653
                    Bun: 607
                    Long Dark: 416
                    Curly Pearl Updo: 122
                    Lucky Green: 417
                    Lioness: 600
                    Natural Red: 608
                    Double Buns: 182
                    Cotton Candy: 228
                    Rose Hair: 388
                    Purple Rainbow: 187
                    Lollipop: 612
                    Silver: 205
                    Braided Ponytail: 561
                    Platinum Pixie: 570
                    Black And White: 110
                    Feeling Turquoise: 412
                  Lips Color:
                    Space: 195
                    Gold: 622
                    Purple: 1967
                    Burgundy: 1995
                    Party Pink: 1114
                    Passion Red: 3008
                    Flashy Blue: 1099
                  Skin Tone:
                    Rainbow Bright: 197
                    Light Warm Yellow: 1021
                    Burning Red: 497
                    Cyber Green: 511
                    Night Goddess: 85
                    Deep Warm Gold: 1026
                    Light Medium Warm Gold: 997
                    Deep Bronze: 1047
                    Medium Olive: 976
                    Deep Neutral: 996
                    Medium Gold: 937
                    Light Warm Olive: 1031
                    Cool Blue: 486
                    Golden: 193
                  Face Accessories:
                    Oversized Statement Sunglasses: 396
                    Psychedelic Sunglasses: 390
                    Resting Butterfly: 83
                    Red Round Sunglasses: 695
                    Classic Aviator WoW: 414
                    Black Mask: 398
                    Cateye Sunglasses: 221
                    On Fire: 116
                    70s Feels: 718
                    3D Glasses: 216
                    Round Glasses: 704
                    Black Round Retro: 403
                    Hypnotic Glasses: 209
                totalSupply: '10000'
                contractAddress: '0xe785E82358879F061BC3dcAC6f0444462D4b5330'
  operationId: summarizeNFTAttributes-v3
```
