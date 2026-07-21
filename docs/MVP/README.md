# Alchemy Documentation

> Scraped from [https://www.alchemy.com/docs](https://www.alchemy.com/docs)

## Sections

| Раздел | Файлов | Размер | Описание |
|--------|--------|--------|----------|
| [Consolidated Templates](00-consolidated/) | 5 | 104K | Общие шаблоны + per-chain переменные (заменяют 178 файлов) |
| [Chains](01-chains/) | 91 | 916K | Chain APIs, RPC, Trace, Debug, Subscriptions |
| [Data](02-data/) | 88 | 1.1M | Token, NFT, Portfolio, Transfers, Prices, Webhooks, Simulation |
| [Wallets](03-wallets/) | 5 | 60K | Account Kit, Gas, Bundler, Smart Accounts |
| [Build with AI](04-build-with-ai/) | 9 | 112K | Agent Skills, CLI, MCP Server, Claude Plugin |
| [Tools & Resources](05-tools-resources/) | 224 | 1.6M | Pricing, Tutorials, Blockchain Basics, Quickstarts |

## Consolidated Templates

| Документ | Заменяет | Размер | Экономия |
|----------|----------|--------|----------|
| [api-quickstart](00-consolidated/api-quickstart.md) | 68 per-chain quickstarts | 8K + 19K data | 91% |
| [api-faq](00-consolidated/api-faq.md) | 63 per-chain FAQs | 7K + 16K data | 88% |
| [api-overview](00-consolidated/api-overview.md) | 44 per-chain overviews | 6K + 12K data | 93% |
| [utxo-websockets](00-consolidated/utxo-websockets.md) | BTC/BCH/LTC websockets | 3K | 87% |
| [utxo-overview](00-consolidated/utxo-overview.md) | BTC/BCH/LTC UTXO | 1K | 83% |

**Структура консолидированных docs:**
- `api-quickstart.md` + `api-quickstart_data.json` — общий шаблон + per-chain переменные (RPC URL, chain ID и т.д.)
- Переносимые файлы помечены `⚠️` со ссылкой на consolidated doc

## Deduplication Stats

- Найдено **44 группы дубликатов** (одинаковые блоки)
- **205 замен** в **145 файлах** → дубликаты заменены ссылками на каноничный источник
- **178 файлов-шаблонов** → **5 consolidated template** + JSON data

---
**Total: 432 pages (422 individual + 5 consolidated + 5 JSON) · 4.9 MB**
