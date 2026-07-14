# Миграция Base→Sui — полный лог изменений

**Дата:** 2026-07-14  
**Артефакт:** mu-v2-sui.tar.gz + ETAP2 механическая миграция  
**Тесты:** 87 → 104 (все зелёные)

---

## 1. CanonAddress: 20B → 32B (Ш1)

**Файл:** `mu-common/src/ids.rs`

| Было | Стало |
|------|-------|
| `addr: [u8; 20]` | `addr: [u8; 32]` |
| hex-длина 40 символов | hex-длина 64 символа |
| EIP-55 checksum (keccak256) | нема чексумми (Sui: будь-який регістр) |
| `AddrErr::BadChecksum` | видалено |
| `chain_id: u64` | залишено (транспортний рівень) |

**Зачеплені файли (~13):** mu-connect, mu-core, mu-gate, mu-policy, mu-runtime, mu-daemon, mu-fixtures, sui-smoke, composer-*

**Сумісність EVM:** `crypto.rs::build_transfer` використовує `&to.bytes()[..20]` для зворотньої сумісності.

---

## 2. FORMAT_VERSION: 1 → 2 (Ш3)

**Файл:** `mu-core/src/format.rs`

Старий формат (v1) з 20B-адресами відкидається `VersionUnsupported`.  
Міграція існуючих μ-об'єктів — через `reissue`, не автоматичний апгрейд.

---

## 3. eip55.rs → sui_addr.rs (Ш2)

**Файл:** `mu-policy/src/eip55.rs` → видалено  
**Новий файл:** `mu-policy/src/sui_addr.rs`

| Було | Стало |
|------|-------|
| Перевірка EIP-55 (keccak256 + регістр hex) | Тільки довжина/hex (без чексумми) |
| `sha3` залежність | Видалена |
| W-ADR-01: "адрес без EIP-55 чексумми" | "звірте identicon" (чексумми в Sui нема) |
| Тест-вектори: EIP-55 офіційні | Тест-вектори: адреса з `sui keytool` |

---

## 4. omega_check: gasless (Ш4)

**Файл:** `mu-runtime/src/policy.rs`

```diff
- pub fn omega_check(amount, gas_estimate, connector, o) -> Result<Amount, OmegaDeny>
+ pub fn omega_check(amount, connector, o) -> Result<Amount, OmegaDeny>
```

- `total = amount + gas_estimate` → `total = amount` (Sui gasless)
- RISK-M7-6-логіка запасу видалена
- `needs_human` без змін (порівнює total з threshold)

---

## 5. M9 PayConfirm: gas_est видалено (Ш5)

**Файл:** `mu-human/src/lib.rs`

```diff
  pub struct PayConfirm {
      pub recipient: CanonAddress,
      pub wl_label: Option<String>,
      pub amount: Amount,
-     pub gas_est: Amount,
      pub agent_id: String,
```

Pipeline: виклик `connector.quote()` видалено (був потрібен тільки для gas_estimate).

---

## 6. Sui-коннектор (Фаза A)

**Нові файли:**
- `mu-connect/src/sui.rs` — SuiConnector + SuiRpc trait + signing/address derivation
- `mu-connect/src/sui_jsonrpc.rs` — HTTP-транспорт, base58, JSON-RPC
- `mu-connect/tests/sui_risks.rs` — 12 тестів RISK-M7-1 для Sui
- `sui-smoke/` — live-testnet harness

**Ключові відмінності від EVM-коннектора:**

| Аспект | EVM (crypto.rs) | Sui (sui.rs) |
|--------|-----------------|--------------|
| Адреса | 20B | 32B |
| Підпис | k256/secp256k1 (ECDSA+recovery) | P-256/secp256r1, RFC 6979 |
| Хеш | SHA-256 (EVM sighash) | Blake2b-256(intent ‖ tx_bytes) |
| Gas | model: gas_estimate + fee_cap | protocol-level gasless |
| nonce | account-nonce (on-chain counter) | digest-детермінований (ідемпотентність) |
| RPC методи | eth_call, eth_sendRaw, eth_getTxReceipt | sui_dryRun, sui_execute, sui_getTx |
| Digest | tx_hash = keccak256(RLP) | digest = base58(blake2b) |
| Self-check | decode calldata → recipient+amount | dry-run обох нод → звірка |
| Reconcile | nonce-state (ConsumedByOther=nonce з'їдено) | object-version (ConsumedByOther=версія coin зросла) |

---

## 7. M4 vault: TxSignerP256

**Файли:** `mu-vault/src/signer.rs`, `lib.rs`, `backend.rs`

- Додано `TxSignerP256` — P-256 (secp256r1) хендл для Sui (flag 0x02)
- `tx_signer_p256()` метод тpeйту Vault (default: `KeyMissing`)
- SoftVault реалізує обидва: `tx_signer()` (k256/EVM) та `tx_signer_p256()` (P-256/Sui)

**Чому P-256?** Enclave (Secure Enclave, TEE) мають нативну підтримку P-256, тоді як k256 частіше в софті. Sui приймає flag 0x02 (Secp256r1) нативно.

---

## 8. Proposal format (Ш8)

**Файл:** `composer-core/src/proposal.rs`

- Розмір адреси в proposal: 20B → 32B (encode/decode)
- `VERSION = 1` → залишено (протокол proposal оновлено в межах тієї ж версії, оскільки демон ще не в проді)

---

## 9. M8 gate (Ш7)

**Файл:** `mu-gate/src/wire.rs`

`MAX_RECIPIENT = 64` → `80` (0x + 64 hex-символи = 66 + запас)

---

## 10. M5 reconcile (Ш6)

**Файл:** `mu-log/src/reconcile.rs`

- NonceState зберіг варіанти (ConsumedByOther, NotReached, Unknown)
- **Sui семантика:**
  - `ConsumedByOther`: coin-об'єкт витрачено іншою tx (версія зросла), digest не знайдено
  - `NotReached`: версія об'єкта не змінилась (tx не могла виконатись)
- `RpcReceipt` зберігається з mu-connect::crypto (EVM-коннектор ще активний)

---

## 11. Fixtures (Ш9, оновлено)

**Файл:** `mu-fixtures/src/main.rs`

- Усі адресні літерали: 40 → 64 hex
- whitelist: 32B Sui-адреса
- `mu.bin` тепер FORMAT_VERSION=2

---

## 12. DoD перевірки

```bash
# Чистота міграції — жодних слідів Base/EIP-55/gas в src/
grep -rn 'keccak\|KECCAK\|Keccak\|sha3' --include='*.rs' src/   # → пусто
grep -rn 'gas_estimate\|gas_est' --include='*.rs' src/          # → пусто
grep -rn 'EIP-55\|eip55' --include='*.rs' src/                  # → пусто (крім docs/)
```
