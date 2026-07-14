# μ-Agentic Blockchain MVP

**μ-daemon + Δ-Composer** — gasless USDC-платежи на Sui (post-EVM migration).

## Directory structure

```
├── mu/                        # Rust workspace (Cargo workspace)
│   ├── mu-common/             #   X — Amount, Clock, идентификаторы
│   ├── mu-core/               #   M1 — μ-объект (формат, сериализация)
│   ├── mu-vault/              #   M4 — ключи, подпись, бэкенды
│   ├── mu-log/                #   M5 — WAL + hash-chain, reconcile
│   ├── mu-runtime/            #   M6 — конвейер Ω→Δ→WAL→execute
│   ├── mu-connect/            #   M7 — Sui-коннектор (gasless) + стабы
│   ├── mu-gate/               #   M8 — вход: unix socket, подпись, allowlist
│   ├── mu-human/              #   M9 — диалог владельца
│   ├── mu-policy/             #   Политики Δ, валидация (Composer)
│   ├── mu-daemon/             #   Бинарь демона (boot-протокол + socket)
│   ├── mu-fixtures/           #   Генератор dev-фикстур (softvault)
│   ├── composer-core/         #   Логика Composer (proposal, drafts)
│   ├── composer-cli/          #   CLI: mu-compose (set-limit, wl-add, ...)
│   ├── composer-tauri/        #   GUI: Tauri 2 (vanilla JS UI)
│   └── sui-smoke/             #   Live-harness testnet
├── docs/
│   ├── agent/                 # AGENT-* build manifests
│   ├── architecture/          # Архитектура, спеки модулей
│   ├── design/                # Заметки, оценки объёмов
│   ├── specs/                 # SPEC_* — полные риски
│   └── smoke/                 # Smoke-тесты + инструкции
├── README.md
└── .gitignore
```

## Build & Test

```bash
# Требования: Rust ≥ 1.75
cd mu
cargo test --workspace --features mu-vault/softvault   # 104 tests, all green

# Сборка демона (debug)
cargo build -p mu-daemon --features softvault

# Сборка CLI
cargo build -p composer-cli

# Сборка GUI (требует libwebkit2gtk-4.1-dev + Rust 1.77+)
cd composer-tauri && cargo tauri build --bundles deb,rpm
```

## Smoke-test (dev-фикстуры)

```bash
export MU_HOME=/tmp/mu-smoke && rm -rf $MU_HOME
cargo run -p mu-fixtures
./target/debug/mu-daemon    # boot → socket listen
```

## Networks

- **Base (EVM)**: оригинальный коннектор (crypto.rs) — сохранён для совместимости
- **Sui (gasless)**: основной коннектор (sui.rs + sui_jsonrpc.rs) — P-256, Blake2b, Address-Balances
