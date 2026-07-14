# SPEC: модули M4 mu-vault, M7a crypto-connector, M8 mu-gate — v1.0 (MVP)

Статус: Draft для реализации • Дата: 2026-07-12 • Система: μ-agentic MVP v3
Фокус документа: **рисковые места** — каждая секция RISK содержит инвариант, механизм принуждения и тест, доказывающий инвариант.

Обозначения: 🔴 инвариант, нарушение = потеря денег/секретов; 🟡 инвариант, нарушение = отказ в обслуживании/слепота.

---

# ЧАСТЬ I. M4 mu-vault — граница секретов

## 1. Назначение
Единственный модуль, где существуют секреты (ключ подписи μ, ключ кошелька). Наружу выходят только подписи и scoped-хэндлы. Ранг чувствительности: №1 (необратимая потеря/кража средств).

## 2. Публичный API

```rust
pub trait Vault: Send + Sync {
    fn sign_mu(&self, digest: &[u8; 32]) -> Result<P256Sig, VaultErr>;
    fn owner_sign(&self, digest: &[u8; 32]) -> Result<P256Sig, VaultErr>; // требует UserAuth (биометрия)
    fn tx_signer(&self) -> Result<TxSigner, VaultErr>;
    fn attest(&self) -> VaultInfo; // backend, hw_backed: bool, key_ids
}

pub struct TxSigner(Zeroizing<SecretKey>);       // поле приватно, тип не Clone, не Debug
impl TxSigner {
    pub fn sign(&self, sighash: &[u8; 32]) -> Secp256k1Sig;  // RFC 6979 детерминированный
}
impl Drop for TxSigner { /* zeroize гарантирован типом Zeroizing */ }

#[non_exhaustive]
pub enum VaultErr { HwUnavailable, UserAuthRequired, UserAuthFailed,
                    WrapCorrupted, KeyMissing, Backend(String) }
```

Контракт API:
- `sign_mu` — ключ в железе, байты наружу не выходят никогда.
- `owner_sign` — обязан требовать свежую (≤30 с) аутентификацию пользователя средствами ОС; вызывается только из M9-потока.
- `tx_signer` — единственный путь к ключу кошелька; хэндл живёт в пределах одного вызова `Connector.execute` (см. Часть II §4).

## 3. Бэкенды

| Бэкенд | Ключ μ / owner | Ключ кошелька (secp256k1) | Особенности |
|---|---|---|---|
| AppleEnclave | SecKey P-256, kSecAttrTokenIDSecureEnclave; owner-ключ с kSecAccessControlBiometryCurrentSet | AES-256-GCM wrap ключом-KEK из Enclave | biometryCurrentSet: смена биометрии инвалидирует owner-ключ — задокументированное поведение восстановления |
| AndroidStrongBox | Keystore, setIsStrongBoxBacked(true), owner: setUserAuthenticationRequired + CryptoObject | То же: wrap KEK'ом Keystore | Fallback на TEE-Keystore при отсутствии StrongBox — с флагом hw_backed=false в attest, решение о допуске принимает конфиг демона |
| Tpm2 | ESAPI, persistent handle, PolicyAuthValue | Wrap TPM-KEK | Owner-auth = Windows Hello через NCrypt при наличии, иначе PIN-policy |
| SoftVault | В памяти процесса | В памяти | ТОЛЬКО тесты: `#[cfg(feature="softvault")]`; release-профиль содержит `compile_error!` при включённой фиче |

## 4. Хранение ключа кошелька

```
wallet.wrap (файл, 0600):
  { v:1, kek_id, nonce:12B, ct: AES-256-GCM(privkey32B, kek, aad=mu_id‖chain_id), created_at }
```
- AAD (Additional Authenticated Data — аутентифицируемые, но не шифруемые данные) связывает wrap с конкретным μ и цепью: подсунуть wrap от другого μ невозможно.
- Генерация ключа: CSPRNG ОС (SecRandomCopyBytes / getrandom); в MVP без BIP-39 seed-фразы — бэкап не предусмотрен by design (ограничение №3 архитектуры v3), файл wrap бэкапить бессмысленно без KEK.

## 5. RISK-секции M4

### RISK-M4-1 🔴 Утечка plaintext-ключа из памяти
- **Инвариант:** plaintext secp256k1-ключа существует только внутри `TxSigner` между unwrap и Drop; время жизни ≤ длительности `execute()`.
- **Принуждение типами:** `TxSigner` — `!Clone`, `!Debug`, `!Serialize`; поле приватно; конструктор только внутри M4; `Zeroizing` обнуляет при Drop; линт CI: `forbid(derive(Debug))` на модуле секретов; запрет `mem::forget` через clippy::mem_forget=deny.
- **Принуждение процессом:** `mlock`/`VirtualLock` на страницу ключа (best-effort, ошибка не фатальна, но логируется); core dumps отключены (`setrlimit(RLIMIT_CORE,0)` / Windows WER opt-out) при старте демона.
- **Тест:** integration: после Drop сканировать heap-снимок процесса (тест-хук) на 32-байтный паттерн ключа — не найден; unit: попытка использовать TxSigner после move в execute не компилируется (compile-fail тест).

### RISK-M4-2 🔴 Подмена/порча wrap-файла
- **Инвариант:** любое изменение wrap → `WrapCorrupted`, демон останавливается, подписи невозможны.
- **Механизм:** GCM-тег + AAD; отдельно checksum файла в μ.Value.key_ref — двойная проверка (M1 и M4 независимо).
- **Реакция:** `WrapCorrupted` — фатальна: runtime → halt, статус «средства недоступны, требуется восстановление»; НИКАКИХ автоматических пере-генераций ключа.
- **Тест:** бит-флип каждого поля wrap → WrapCorrupted; wrap от другого mu_id (AAD) → WrapCorrupted.

### RISK-M4-3 🔴 Обход биометрии для owner-операций
- **Инвариант:** `owner_sign` без успешной свежей UserAuth невозможен на уровне ключа, не на уровне кода.
- **Механизм:** ключ создаётся с аппаратным флагом auth-required (kSecAccessControl / setUserAuthenticationRequired / TPM policy) — проверка выполняется железом/ОС, демон не может её пропустить даже при баге.
- **Тест:** на устройстве: вызов owner_sign без auth → UserAuthRequired от платформы; мок-платформа в CI отклоняет подпись без auth-токена.

### RISK-M4-4 🟡 Недоступность железа (обновление ОС, сброс биометрии)
- **Инвариант:** деградация всегда fail-stop, никогда fail-open (без тихого перехода на софт-ключи).
- **Механизм:** attest() при старте; если hw_backed=false, а конфиг требует hw → halt с отчётом. biometryCurrentSet-инвалидация owner-ключа: документированная процедура re-enroll (новый owner-ключ, перевыпуск подписи Δ) — требует физического владельца.
- **Тест:** конфиг require_hw=true + SoftVault → демон не стартует.

### RISK-M4-5 🔴 Кросс-использование ключей
- **Инвариант:** ключ μ подписывает только структуры μ/лога; ключ кошелька — только sighash транзакций; owner — только Δ/подтверждения. Домены не пересекаются.
- **Механизм:** доменная сепарация на уровне digest: `digest = SHA256(domain_tag ‖ payload)`, теги: `"mu.core.v1"`, `"mu.log.v1"`, `"mu.delta.v1"`, `"mu.human.v1"`; для tx — sighash строится только в M7a. sign_mu/owner_sign принимают уже-хешированное значение вместе с enum DomainTag и сами добавляют тег — сырой digest без тега подписать нельзя.
- **Тест:** подпись с тегом A не верифицируется под тегом B (unit по всем парам).

## 6. Тест-матрица M4 (сводно)
Unit: доменные теги ×5², RFC 6979 детерминизм (векторы), wrap roundtrip. Platform-smoke (реальные устройства, ручной прогон перед релизом): enclave-подпись, auth-required, перезагрузка→unwrap. CI: SoftVault-конвейер, compile-fail тесты, heap-скан. Гейт: 100% ветвей VaultErr покрыты.

---

# ЧАСТЬ II. M7a crypto-connector (USDC / EVM L2)

## 1. Назначение
Единственная точка движения реальных денег. Ранг №4, но с самым коварным классом бага: **неверная трактовка неизвестности сети как отказа → двойная трата**.

## 2. Публичный API (реализация trait Connector)

```rust
impl Connector for CryptoConnector {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr>;              // gas_estimate для Ω-check
    fn execute(&self, i: &Intent, s: TxSigner) -> Result<TxRef, ConnErr>;
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr>;
}
pub struct TxRef { pub tx_hash: B256, pub chain_nonce: u64, pub raw_tx_hash: B256 }
pub enum TxStatus { Pending, Settled { block: u64, effective_gas: u128 }, Failed { reason: FailReason } }
#[non_exhaustive]
pub enum ConnErr {
    Rejected(RejectReason),   // достоверный отказ ДО попадания tx в сеть — можно Failed
    Unknown(String),          // сеть/таймаут/расхождение — ТОЛЬКО Pending, решает reconcile
    Config(String),           // неверная конфигурация — halt
}
pub enum RejectReason { InsufficientFunds, NonceTooLow, InvalidRecipient, FeeCapTooLow }
```

**Контракт ошибок (главный в модуле):** `Rejected` разрешён ТОЛЬКО если получен детерминированный отказ от обоих RPC до/при отправке И tx гарантированно не могла попасть в mempool. Всё остальное — `Unknown` → M6 оставляет PENDING → reconcile. Возврат `Failed` из execute запрещён сигнатурой (нет такого варианта) — Failed выносит только status()/reconcile по данным цепи.

## 3. Конфигурация

```
chain_id (Base=8453), usdc_contract (адрес, захардкожен per chain_id в бинаре, не в конфиге),
rpc: [url1, url2] (обязательно разные провайдеры), confirmations: u8 (=1 для Base, конфиг),
fee: { max_base_multiplier: 2, tip_gwei, absolute_fee_cap },
timeouts: { send_ms: 10_000, receipt_poll_ms: 3_000, finality_ms: 120_000 }
```
USDC-адрес не конфигурируем намеренно: подмена адреса контракта в конфиге = кража (см. RISK-M7-5).

## 4. Поток execute (нормативный)

```
1. sanity: intent.chain_id == cfg.chain_id; recipient канонизирован (иначе Rejected(InvalidRecipient))
2. nonce = max(local_next_nonce_из_M5, rpc1.getTransactionCount(addr,"pending"), rpc2.…)
   расхождение rpc1≠rpc2 больше чем на 1 → Unknown (не гадать)
3. calldata = ERC20.transfer(recipient, amount);  simulate: eth_call на обоих RPC
   оба вернули revert → Rejected(по декоду причины); один — Unknown
4. fee: EIP-1559, maxFee=min(base*2+tip, absolute_fee_cap); превышение cap → Rejected(FeeCapTooLow)
5. sighash → s.sign() (TxSigner передан по move — второй раз подписать этим хэндлом нельзя)
6. raw_tx → eth_sendRawTransaction ПАРАЛЛЕЛЬНО в оба RPC
   ≥1 принял (или вернул "already known") → Ok(TxRef)
   оба вернули детерминированный reject из списка RejectReason → Rejected
   иначе (таймаут, неизвестная ошибка, разногласие) → Unknown  ← tx МОГЛА уйти
7. M6 пишет Settled только после status()==Settled{confirmations}
```

## 5. RISK-секции M7a

### RISK-M7-1 🔴 Двойная трата через ложный Failed
- **Инвариант:** платёж, чья судьба неизвестна, никогда не освобождает резерв Δ и никогда не помечается Failed.
- **Механизм:** тип ConnErr без Failed-варианта в execute; таксономия Rejected закрытым enum'ом с тестом на каждый вариант; всё неклассифицированное — Unknown.
- **Тест:** мок-RPC матрица ~20 сценариев (таймаут после send, "nonce too low" после успешного send в другой RPC, 502, обрыв TLS на полукадре) → ни один не даёт Rejected после того, как raw_tx ушла хотя бы в один сокет; chaos: kill демона между шагами 6 и 7 → после рестарта reconcile находит tx → Settled, повтора нет.

### RISK-M7-2 🔴 Nonce-коллизия / застрявшая tx
- **Инвариант:** на один chain_nonce система признаёт максимум одну Settled-транзакцию; застрявший nonce блокирует последующие платежи, а не обходится.
- **Механизм:** локальный next_nonce из M5 (Pending-записи содержат chain_nonce) имеет приоритет источника; правило шага 2. Замена застрявшей tx (speed-up с тем же nonce и большим fee) — НЕ в MVP: вместо этого статус StuckPending и ручная процедура (документирована), потому что автоматический replace — классический источник двойных списаний.
- **Тест:** сценарий «tx в mempool 10 мин» → новые intent'ы получают Denied(Busy) от M6 (очередь за nonce), не новый nonce мимо застрявшего.

### RISK-M7-3 🔴 Лживый / рассинхронизированный RPC
- **Инвариант:** Settled объявляется только при согласии обоих провайдеров о receipt (block, status=1) с ≥ confirmations.
- **Механизм:** status() опрашивает оба; расхождение → Pending + метрика rpc_divergence; один провайдер недоступен > N минут → деградация в режим «только reconcile, новые платежи Denied(Busy)» (конфиг allow_single_rpc=false по умолчанию).
- **Тест:** мок: rpc1 отдаёт receipt, rpc2 — null → Pending; rpc1 отдаёт status=0 (revert on-chain), rpc2 status=1 → Pending+alert (реорг/ложь), решение за reconcile после confirmations.

### RISK-M7-4 🟡 Реорганизация цепи (reorg)
- **Инвариант:** Settled необратим для учёта Δ; вероятность реорга минимизируется порогом confirmations.
- **Механизм:** для Base/Arbitrum confirmations=1 допустимо (быстрая финальность), но параметр конфигурируем; reconcile перепроверяет receipt по hash+blockNumber — исчезнувший receipt для уже-Settled записи = alert-инцидент (не автоматический откат: деньги могли уйти, вручную).
- **Тест:** мок-реорг: receipt исчезает после Settled → запись остаётся Settled, генерируется Alert-Kind в лог.

### RISK-M7-5 🔴 Подмена адреса контракта/получателя
- **Инвариант:** средства могут уйти только на адрес из whitelist Δ и только через захардкоженный USDC-контракт данного chain_id.
- **Механизм:** контракт — константа в бинаре с тестом на известные адреса; recipient в calldata собирается из intent, прошедшего Δ, единственной функцией сборки (никаких строковых конкатенаций); simulate (шаг 3) декодирует calldata обратно и сверяет recipient+amount с intent (self-check от бага сборщика).
- **Тест:** мутационный: порча любого байта recipient в сборщике → self-check ловит до подписи.

### RISK-M7-6 🟡 Гонка quote→execute по газу
- **Инвариант:** сумма, зарезервированная Δ (amount+gas_estimate), ≥ фактически списанной.
- **Механизм:** gas_estimate в quote берётся с запасом (estimate×1.5, cap'ится absolute_fee_cap); effective_gas из receipt пишется в Settled; если effective > estimate (аномалия) — alert, но резерв уже покрыт запасом.
- **Тест:** статистический на testnet: 100 переводов, ни одного effective>reserved.

## 6. Тест-матрица M7a (сводно)
Unit: таксономия ошибок (каждый RPC-ответ → правильный ConnErr), сборка/декод calldata, fee-мат. Integration (Anvil-форк Base): happy path, revert-simulate, nonce-гонки, kill-points. Testnet acceptance: 50 переводов Base Sepolia, 0 расхождений журнал↔цепь. Гейт: mutation testing сборщика calldata ≥ 90% killed.

---

# ЧАСТЬ III. M8 mu-gate — периметр

## 1. Назначение
Единственный вход недоверенных данных. Ранг №5: пробитие ограничено Ω/Δ, но M8 обязан гарантировать, что до Ω/Δ доходит только аутентичный, свежий, неповторённый intent, и что парсер не является вектором RCE/DoS.

## 2. Протокол провода

```
Транспорт: unix socket $MU_HOME/agent.sock (0600) / named pipe (DACL user-only)
Кадр:      len:u32 LE (≤ 4096) ‖ body
body:      cbor(WireIntent) ‖ sig:64B (Ed25519 над cbor-байтами)
WireIntent = { 1:v=1, 2:recipient tstr≤128, 3:amount uint(u128), 4:asset tstr≤16,
               5:purpose tstr≤256B, 6:agent_id tstr≤64, 7:nonce uint, 8:ts uint }
Ответ:     cbor({ticket:16B}) | cbor({deny:u16})  — код без деталей
Poll:      cbor({q:ticket}) → cbor({state:u8, tx_hash?})
```

## 3. Конвейер проверки (строгий порядок, ранний выход)

```
0. accept: лимит одновременных соединений = 4; на соединение read-timeout 5 с
1. len ≤ 4096, иначе drop соединения (без ответа)
2. точное чтение len байт; body ≥ 65 байт
3. cbor-decode WireIntent hardened-парсером (лимиты глубины=4, полей=16)
4. agent_id ∈ allowlist → pubkey                       иначе deny(0x01)
5. Ed25519.verify(pubkey, body_без_sig, sig)           иначе deny(0x02)
6. |ts − now| ≤ 60 с                                   иначе deny(0x03)
7. nonce > last_nonce[agent_id] (окно в памяти + персист в M5 при shutdown)  иначе deny(0x04)
8. rate: token-bucket per agent (дефолт 10/мин, burst 3) иначе deny(0x05)
9. → SignedIntent в канал M6; переполнение канала → deny(0x06 Busy)
```
Порядок 4→5 сознателен: verify подписи — самая дорогая операция, до неё отсекается всё дешёвое; но ts/nonce ПОСЛЕ verify, чтобы deny-коды 0x03/0x04 нельзя было получить неаутентифицированно (иначе оракул для подбора).

## 4. RISK-секции M8

### RISK-M8-1 🔴 Replay intent'а
- **Инвариант:** один подписанный body исполняется максимум один раз за всю жизнь системы.
- **Механизм:** монотонный nonce на агента (правило 7); last_nonce переживает рестарт: при shutdown — снапшот в M5; при аварийном рестарте — восстановление max(nonce) из записей лога этого агента + требование ts-свежести (окно 60 с) закрывает щель между последним логом и крэшем.
- **Тест:** повтор того же кадра → deny(0x04); рестарт демона → повтор старого кадра → deny (лог-восстановление); property: при любой перестановке валидных кадров количество принятых = количеству уникальных nonce.

### RISK-M8-2 🔴 Обход аутентификации / путаница ключей
- **Инвариант:** intent принимается только если подпись верифицирована ключом именно того agent_id, что указан в кадре.
- **Механизм:** pubkey берётся строго из allowlist по agent_id (не из кадра); allowlist — файл 0600, редактируется только через M9 (биометрия) с записью в лог; подпись покрывает ВЕСЬ cbor включая agent_id (подмена id ломает подпись).
- **Тест:** валидная подпись агента A на кадре с agent_id=B → deny(0x02); мутация любого байта body → deny(0x02).

### RISK-M8-3 🔴 Уязвимость парсера (RCE/крэш на входе)
- **Инвариант:** никакой вход не вызывает панику, UB или аллокацию > 8 КБ.
- **Механизм:** CBOR-декодер с жёсткими лимитами (глубина, размер строк, запрет indefinite-length); выделения — только из body-буфера фиксированного размера; парсер — отдельный crate `mu-wire`, общий с policy-endpoint (Composer) — одна hardened-реализация на все входы демона.
- **Тест:** cargo-fuzz на mu-wire: 0 крэшей/утечек за ≥ 4 ч CI-джоб + корпус регрессий; ASAN/UBSAN-прогон интеграционных тестов.

### RISK-M8-4 🟡 DoS: slow-loris, флуд соединениями
- **Инвариант:** злоумышленный локальный процесс не может заблокировать обработку intent'ов легитимного агента дольше секунд.
- **Механизм:** read-timeout 5 с на кадр; ≤ 4 соединений (LRU-вытеснение старейшего idle); token-bucket per agent; глобальный bucket 60/мин.
- **Тест:** нагрузочный: 100 паразитных соединений по байту в секунду + легитимный агент → latency легитимного p99 < 500 мс.

### RISK-M8-5 🟡 Утечка информации через ответы
- **Инвариант:** неаутентифицированный отправитель не узнаёт из ответов ничего, кроме «нет» (deny 0x01/0x02 неразличимы по времени в пределах джиттера).
- **Механизм:** единый формат deny{code}; коды 0x01/0x02 возвращаются с констант-временным паддингом (грубое выравнивание, не криптографическая гарантия — угроза локальная); тексты причин Δ/Ω/балансов в ответах отсутствуют по типу (в структуре ответа нет таких полей).
- **Тест:** ревью типа ответа (compile-level: поля отсутствуют); тайминг-смок 0x01 vs 0x02: |Δt| < 20%.

### RISK-M8-6 🟡 Права на сокет
- **Инвариант:** к сокету имеет доступ только пользователь демона.
- **Механизм:** создание с umask 0177 → chmod 0600 → проверка владельца при старте; отказ стартовать, если каталог $MU_HOME групповой/мировой записи. Windows: явный DACL (текущий SID), запрет наследования.
- **Тест:** CI (Linux): попытка connect из-под другого uid (в контейнере) → EACCES.

## 5. Тест-матрица M8 (сводно)
Fuzz mu-wire (гейт: 4 ч clean). Property: nonce-множества. Integration: конвейер 0–9 по одному отрицательному тесту на шаг + happy path. Нагрузка: DoS-сценарий. Гейт: каждый deny-код имеет ровно один порождающий шаг (таблица трассируемости).

---

# Сквозная таблица трассируемости рисков

| RISK | Инвариант (кратко) | Доказывающий тест | Класс потерь |
|---|---|---|---|
| M4-1 | plaintext ключа ⊂ TxSigner lifetime | heap-скан + compile-fail | кража средств |
| M4-2 | порча wrap → halt | бит-флип матрица | недоступность |
| M4-3 | owner_sign ⇐ железная биометрия | device-smoke | несанкц. Δ |
| M4-5 | доменная сепарация подписей | пары тегов | подмена структур |
| M7-1 | Unknown ≠ Failed | RPC-матрица + kill-points | двойная трата |
| M7-2 | 1 nonce = ≤1 Settled | застрявшая tx | двойная трата |
| M7-3 | Settled ⇐ консенсус 2 RPC | расхождение receipt | ложный учёт |
| M7-5 | recipient/contract неподменяемы | mutation testing | кража средств |
| M8-1 | body исполняется ≤1 раза | replay + рестарт | повторный платёж |
| M8-2 | подпись ↔ agent_id | кросс-ключи | чужие платежи |
| M8-3 | вход не роняет демон | fuzz 4ч | DoS/RCE |

Правило приёмки модулей: PR, меняющий код в зоне RISK-секции, обязан ссылаться на неё и не может быть смержен без зелёного соответствующего теста (CODEOWNERS + CI-метки).
