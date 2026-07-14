# SPEC: модули M1 mu-core, M2 mu-omega, M3 mu-delta, M7b/c stubs, X mu-common — v1.0 (MVP)

Статус: Draft для реализации • Дата: 2026-07-12 • Система: μ-agentic MVP v3
Формат RISK-секций: 🔴 нарушение = деньги/несанкционированные операции; 🟡 нарушение = отказ/слепота.
Связанные документы: SPEC_M4_M7a_M8_v1, SPEC_M6_M5_M9_v1, SPEC_Delta-Composer_v1.
Этим документом комплект спек модулей закрывается полностью (M1–M9 + Composer + сквозной слой).

Особенность группы: M1/M2/M3 — модули с **громким отказом** (halt/Deny — безопасный режим), их риск не в сложности, а в том, что они — фундамент, на который ссылаются гарантии остальных. M7b/c — риск «симуляция просочилась в реальность». X — риски, разлитые по всем модулям (деньги-арифметика, время, ошибки).

---

# ЧАСТЬ I. M1 mu-core — владелец формата μ

## 1. Назначение
Единственный модуль, который сериализует/десериализует/пишет μ-объект. Все прочие получают типизированные структуры. Ранг №7 (fail-stop), но: битый M1 = недоверие ко всему, что подписано форматом.

## 2. Публичный API

```rust
pub fn load(path: &Path, vault: &dyn Vault) -> Result<Mu, CoreErr>;
pub fn save(mu: &Mu, path: &Path, vault: &dyn Vault) -> Result<(), CoreErr>;
pub fn verify(mu: &Mu) -> Result<(), CoreErr>;
pub fn reissue(old: &Mu, new_omega: Omega, vault: &dyn Vault) -> Result<Mu, CoreErr>;
pub fn apply_delta(mu: &Mu, new: SignedDelta) -> Result<Mu, CoreErr>;  // единственный путь смены Δ

#[non_exhaustive]
pub enum CoreErr { Io(..), CborMalformed, SigMu, SigOwner, ChecksumMismatch,
                   VersionUnsupported { found: u16 }, OmegaImmutable, SizeExceeded }
```

## 3. Формат (нормативный, canonical CBOR RFC 8949 §4.2)

```
mu.file = COSE_Sign1(alg=ES256, key=vault.sign_mu, domain="mu.core.v1")
payload = { 1: Core, 2: Omega, 3: COSE_Sign1(Delta, key=owner, domain="mu.delta.v1"),
            4: ValueRef, 5: log_head 32B }
Core    = { 1: uuid_v7 16B, 2: owner_pubkey 33B, 3: created_at u64, 4: version u16 }
Ограничения: |mu.file| ≤ 3072 B; неизвестные ключи map → CborMalformed (закрытая схема, не игнор)
```

Запись на диск: `mu.tmp` → fsync(file) → rename(`mu.cbor`) → fsync(dir). Резервная копия предыдущей версии `mu.prev` (1 поколение) — для диагностики, НЕ для авто-отката (см. RISK-M1-3).

## 4. RISK-секции M1

### RISK-M1-1 🔴 Мутация Ω в обход reissue
- **Инвариант:** байты Ω-слота после выпуска μ неизменяемы; единственный легальный путь — reissue (новый ID, version+1, подпись заново, архив старого лога).
- **Механизм:** тип Mu не даёт `&mut Omega` (поле приватно, только `omega(&self) -> &Omega`); `apply_delta` пересобирает payload, побайтно сверяя старый Ω-подмассив с новым перед подписью (defense-in-depth от бага сборщика); save для Mu с изменённым Ω без флага reissue → `OmegaImmutable`.
- **Тест:** compile-fail на попытку мутации; мутационный тест сборщика: порча байта Ω внутри apply_delta → OmegaImmutable до вызова vault.

### RISK-M1-2 🔴 Даунгрейд/подмена файла μ (rollback-атака)
- **Инвариант:** подписанный, но устаревший μ (старая Δ с большими лимитами) не принимается вместо текущего.
- **Механизм:** тройная связка версий: μ.log_head должен совпадать с последним hash цепи M5 (проверка в boot-шаге 2–3); подмена μ на старую копию → log_head не сходится с логом → halt. Симметрично: откат лога ловится через μ (RISK-M5-2). Обе точки откатить согласованно можно только с правом записи в оба файла + знанием, что HEAD тоже надо откатить — а Pending-развязка через reconcile сверяется с ЦЕПЬЮ (внешний якорь): исчезнувшие Settled → Alert.
- **Тест:** матрица откатов: только μ / только лог / μ+лог+HEAD согласованно → первые два = halt; третий = reconcile-Alert по данным цепи (мок-коннектор «помнит» tx).

### RISK-M1-3 🟡 Полусохранённый μ / потеря при записи
- **Инвариант:** на диске в любой момент есть ровно одна валидная версия μ (старая или новая), никогда «полверсии»; авто-восстановления из mu.prev нет.
- **Механизм:** tmp+fsync+rename+fsync(dir); load читает только `mu.cbor`; битый mu.cbor → halt с указанием на mu.prev как диагностический артефакт — решение о восстановлении принимает владелец руками (авто-откат = канал для RISK-M1-2).
- **Тест:** kill −9 в каждой из 4 точек записи (до fsync-file / до rename / до fsync-dir / после) × 100 → load всегда Ok(старая) или Ok(новая).

### RISK-M1-4 🟡 Расхлябанный парсер формата
- **Инвариант:** load принимает только строго каноничный CBOR закрытой схемы; любые вольности (indefinite-length, дубликаты ключей, неизвестные поля, неканоничный порядок) → CborMalformed.
- **Механизм:** тот же hardened-декодер mu-wire (общий crate с M8/policy-endpoint — одна реализация на все входы); лимиты глубины/размеров; строгая схема через типизированный decode, не через Value-дерево.
- **Тест:** fuzz mu-wire покрывает и схему μ (отдельный fuzz-таргет); векторы неканоничности ×10 → CborMalformed.

## 5. Тест-матрица M1
Roundtrip property (encode∘decode = id); бит-флип каждого байта файла → verify=Err; kill-матрица записи; матрица откатов; fuzz-таргет. Гейт: 100% вариантов CoreErr достижимы тестами.

---

# ЧАСТЬ II. M2 mu-omega — статичный фильтр возможностей

## 1. Назначение
Чистая stateless-функция «может ли μ вообще». Ранг №9 по сложности — и это его главное достоинство, которое нужно защитить от эрозии.

## 2. Публичный API

```rust
pub fn check(i: &Intent, o: &Omega, gas_estimate: Amount) -> Decision;
pub enum Decision { Allow, Deny(OmegaDeny) }
#[non_exhaustive]
pub enum OmegaDeny { OpUnsupported, ConnectorUnknown,
                     CeilingExceeded { asked: Amount, ceiling: Amount } }
```
Контракт: без I/O, без Clock, без паник, без аллокаций сверх Deny-структуры; polynomial-time от размеров入 (фактически O(len(lists))).

## 3. RISK-секции M2

### RISK-M2-1 🔴 Переполнение при сложении суммы и газа
- **Инвариант:** `amount + gas_estimate` вычисляется без wrap-around; переполнение u128 = Deny(CeilingExceeded), не паника и не пропуск.
- **Механизм:** `Amount::checked_add` (тип из mu-common, см. Часть V) — арифметика `+` на Amount не реализована вообще (нет impl Add), только checked_*; clippy: arithmetic_side_effects=deny на весь workspace.
- **Тест:** границы: (u128::MAX, 1), (ceiling, 0), (ceiling−gas, gas), (ceiling−gas+1, gas); property: check(a,g) == check(g,a)-симметрия по перестановке слагаемых.

### RISK-M2-2 🟡 Эрозия чистоты (feature creep)
- **Инвариант:** M2 остаётся чистой функцией; появление в нём состояния/I/O/времени — архитектурная регрессия, ломающая тестируемость перебором.
- **Механизм:** crate no_std (кроме alloc); CI-гейт: список зависимостей M2 = {mu-common}; ревью-правило: новые «возможности» Ω добавляются полем + ветвью Deny, не колбэком/трейтом наружу.
- **Тест:** сам факт no_std-сборки — тест; полный перебор решётки входов на малых доменах (exhaustive для operations×connectors ≤ 8×8).

### RISK-M2-3 🟡 Рассинхрон списков Ω с реальностью коннекторов
- **Инвариант:** connectors в Ω ссылаются только на ConnectorId, реально зарегистрированные в M7-реестре данной сборки; «мертвый» id в Ω не создаёт ложного Allow-пути.
- **Механизм:** ConnectorId — закрытый enum (не строка) в mu-common; сериализация неизвестного значения → CborMalformed на уровне M1; Allow по коннектору, который M6 не найдёт в реестре, невозможен по типу (M6 резолвит enum, не строку).
- **Тест:** compile-level: строкового пути от Ω к выбору коннектора нет; decode Ω с неизвестным id → ошибка M1.

## 4. Тест-матрица M2
Exhaustive-перебор малых доменов; границы checked_add; no_std-гейт; тесты Deny-детерминизма на мусорных Intent (property: никогда паника).

---

# ЧАСТЬ III. M3 mu-delta — политики момента и резерв

## 1. Назначение
Правила владельца + защита лимита. Ранг №6: логика проста, но она — последний программный рубеж перед деньгами после Ω. Все сложные риски M3 вынесены соседям (гонки → M6-3, окно-данные → M5-4, подпись Δ → M1/M4); здесь фиксируются собственные.

## 2. Публичный API

```rust
pub fn check_and_reserve(i: &Intent, d: &Delta, log: &dyn LogRead, clock: &dyn Clock)
    -> Result<Reservation, DeltaDeny>;
pub fn commit(r: Reservation, log: &mut dyn LogWrite);
pub fn rollback(r: Reservation, log: &mut dyn LogWrite);
pub fn needs_human(i: &Intent, d: &Delta) -> bool;
pub struct Reservation { intent_hash: Hash, amount_total: Amount, /* !Clone, !Copy */ }
#[non_exhaustive]
pub enum DeltaDeny { NotWhitelisted, WindowExceeded { spent: Amount, limit: Amount }, Busy }
```

## 3. RISK-секции M3

### RISK-M3-1 🔴 Утечка/дублирование Reservation
- **Инвариант:** каждая Reservation завершается ровно одним commit ИЛИ одним rollback; забытая Reservation не «испаряется», освобождая окно.
- **Механизм:** Reservation — линейный тип: `!Clone/!Copy`, commit/rollback принимают по move (второй вызов не компилируется); Drop без commit/rollback → `debug_assert!` + в release запись Alert{LeakedReservation} в лог через drop-хук (слабая ссылка на LogWrite) — резерв при этом ОСТАЁТСЯ учтённым, т.к. окно считается по Pending-записям M5, а не по объекту в памяти (двойная бухгалтерия: объект — для типового принуждения, лог — источник истины).
- **Тест:** compile-fail: commit дважды / commit+rollback; drop-тест: уроненная Reservation → Alert в логе, window_sum не уменьшился.

### RISK-M3-2 🔴 Сравнение получателя не в канонической форме
- **Инвариант:** membership в whitelist проверяется только над CanonAddress (lowercase-20-байт + chain_id); intent с адресом в любой иной форме не может дать ложный Allow или ложный Deny.
- **Механизм:** тип CanonAddress конструируется единственной функцией mu-policy::canon_address (общей с Composer — та же нормализация на входе и на проверке); Intent после M8 содержит уже CanonAddress (парсинг строки — в M8/mu-wire), M3 строк не видит.
- **Тест:** property: ∀ валидных представлений одного адреса (mixed-case EIP-55, lower, upper) → одинаковый результат membership; тип-гейт: в M3 нет ни одного &str-поля адреса.

### RISK-M3-3 🟡 needs_human рассинхронизирован с резервом
- **Инвариант:** решение «нужен человек» принимается над той же величиной amount_total (amount+gas), что и резерв — иначе платёж на threshold+gas обходит подтверждение.
- **Механизм:** обе функции считают через один приватный `fn total(i) -> Amount`; порог сравнивается с total, что зафиксировано в доке и тесте.
- **Тест:** граничный: amount=threshold, gas>0 → needs_human=true; amount+gas=threshold → true (строгое >сравнение только выше total).

### RISK-M3-4 🟡 Busy-флаг как second line, а не first line
- **Инвариант:** in_flight-защита (Busy) не становится основным механизмом сериализации — она страхует от рефакторинга M6, но однопоточность M6 остаётся нормативной.
- **Механизм:** Busy покрыт тестом, но метрика busy_hits в проде должна быть ≈0; ненулевая — сигнал, что кто-то сломал M6-3.
- **Тест:** unit Busy; алерт-правило в наблюдаемости M6 (§6 спек M6): busy_hits > 0 → warn.

## 4. Тест-матрица M3
Compile-fail линейности ×3; property канонизации; таблица границ threshold/window (совместно с M5-4 property); drop-тест.

---

# ЧАСТЬ IV. M7b bank_stub / M7c card_stub — заглушки

## 1. Назначение
Полный прогон конвейера без реальных денег + инъекция ошибок для chaos-тестов. Главный и почти единственный риск: **симуляция, неотличимая от реальности**.

## 2. Публичный API
Реализуют trait Connector (см. SPEC M7a §2). Конфиг: `latency_ms: (min,max)`, `fail_rate: f32`, `unknown_rate: f32` — параметры инъекции для chaos.

## 3. RISK-секции M7b/c

### RISK-M7S-1 🔴 Симулированный «платёж» попадает в реальный учёт
- **Инвариант:** результаты стабов никогда не влияют на window_sum, не создают Settled-записей и не отображаются владельцу как реальные.
- **Механизм:** тройной барьер: (a) тип — execute стаба возвращает `TxRef::Simulated(uuid)` (отдельный вариант enum, не поддельный B256-хэш); (b) M6 для Simulated-TxRef пишет Kind::Simulated, ветка Settled типом недостижима (match без wildcard); (c) M5.window_sum исключает Simulated (RISK-M5-4), M9 рендерит бейдж SIMULATED.
- **Тест:** property: любой сценарий со стабами → window_sum == сценарий без них; попытка сконструировать TxRef::Real в коде стабов — модульная видимость запрещает (конструктор Real приватен для крейта M7a); grep-гейт CI.

### RISK-M7S-2 🔴 Стаб в релизной сборке как активный коннектор
- **Инвариант:** в release-бинаре стабы присутствуют (для demo-режима), но не могут быть выбраны для intent'а без явного флага конфигурации `demo_mode=true`, который меняет заголовок UI M9 («ДЕМО») и пишет Alert{DemoMode} при каждом старте.
- **Механизм:** резолвер коннекторов M6: ConnectorId::BankStub|CardStub при demo_mode=false → Denied(ConnectorUnknown) несмотря на Ω; demo-режим визуально тотален (M9), а не по-платежный.
- **Тест:** матрица (release/debug × demo on/off × стаб/crypto) — 8 комбинаций, каждая закреплена; UI-снапшот заголовка ДЕМО.

### RISK-M7S-3 🟡 Стаб врёт лучше реальности (нерепрезентативный chaos)
- **Инвариант:** пространство ошибок стаба ⊇ пространство ConnErr реального M7a (иначе chaos-тесты M6 не покрывают реальные ветки).
- **Механизм:** инъекция генерирует все варианты Rejected(...) и Unknown по конфигурируемым вероятностям; таблица соответствия «реальный сценарий RPC → как эмулируется стабом» — часть кода стаба (enum-исчерпывающий match без wildcard: новый вариант ConnErr не скомпилируется без эмуляции).
- **Тест:** компиляция и есть тест (исчерпывающий match); покрытие веток M6 chaos-сьютом ≥ 95% по ConnErr-веткам.

## 4. Тест-матрица M7b/c
Property изоляции учёта; матрица demo-режима 8/8; исчерпывающий match. ~40 LOC кода / ~120 LOC тестов на стаб — тесты втрое больше кода, это норма для модуля, чей единственный риск — «просочиться».

---

# ЧАСТЬ V. X mu-common — сквозной слой (amount, clock, errors, ids)

## 1. Назначение
Типы и контракты, разлитые по всем модулям. Не «утилиты», а несущие конструкции: большинство 🔴-инвариантов других спек опираются на типы отсюда.

## 2. Состав

```rust
// amount.rs
pub struct Amount(u128);          // минорные единицы; NO impl Add/Sub/Mul
impl Amount { checked_add, checked_sub, saturating_display(...) }
pub fn parse_decimal(s: &str, decimals: u8) -> Result<Amount, AmtErr>;  // без float

// clock.rs
pub trait Clock { fn now_unix(&self) -> u64; fn monotonic(&self) -> Instant; }
pub struct SysClock; pub struct TestClock(/* управляемое время */);

// ids.rs
pub enum ConnectorId { Crypto, BankStub, CardStub }   // закрытый
pub struct CanonAddress([u8;20], u64 /*chain_id*/);    // конструктор — только mu-policy
pub struct Hash([u8;32]); pub struct Ticket([u8;16]);

// errors.rs — thiserror-иерархии; правило: все enum #[non_exhaustive] на границах crate'ов
```

## 3. RISK-секции X

### RISK-X-1 🔴 Float или небезопасная арифметика в денежном пути
- **Инвариант:** ни одно значение денег/газа не проходит через f32/f64 и через непроверяемую целочисленную арифметику нигде в workspace.
- **Механизм:** Amount без арифметических impl (checked-методы единственный путь); clippy workspace-уровня: `float_arithmetic=deny` в крейтах денежного пути, `arithmetic_side_effects=deny` везде; parse_decimal — конечный автомат по строке (переполнение → AmtErr).
- **Тест:** grep/линт-гейт CI: `f64|f32` в mu-{common,policy,omega,delta,log,runtime,connect} → фейл; property parse_decimal: roundtrip display∘parse, границы 10^38.

### RISK-X-2 🔴 Смешение источников времени
- **Инвариант:** дедлайны/TTL — только монотонные часы; timestamps записей/окна — только unix-время Clock демона; время из intent'а агента (поле ts) используется ТОЛЬКО для anti-replay в M8 и никогда — для окна/дедлайнов.
- **Механизм:** два разных типа возврата (Instant vs u64) не смешиваются без явной конверсии, которой в API нет; поле ts у WireIntent не копируется в Intent (типы различны: WireIntent → Intent теряет ts by design).
- **Тест:** compile-level: у Intent нет поля ts; unit: перевод системных часов назад в TestClock не продлевает TTL (монотоника) и не сдвигает окно (окно по ts записей — RISK-M5-4).

### RISK-X-3 🟡 Эрозия #[non_exhaustive] и обработки ошибок
- **Инвариант:** добавление варианта ошибки в любой модуль не может «тихо» провалиться в необработанную ветку у потребителя как успех; wildcard-match на критичных enum (ConnErr, TxStatus, Decision, Resolution) запрещён.
- **Механизм:** клиппи-правило wildcard_enum_match_arm=deny для перечисленных типов (allow-list файлов исключений пуст); non_exhaustive заставляет внешних потребителей писать явный catch-all, но ВНУТРИ workspace типы re-export'ятся как exhaustive (два лица типа через feature).
- **Тест:** CI-гейт линта; мутационный: добавление фиктивного варианта в ConnErr в тест-ветке → компиляция M6/M5 падает (доказательство исчерпывающести match).

### RISK-X-4 🟡 Логи приложения как канал утечки
- **Инвариант (сквозной, дублирует нормы других спек как единая точка):** в tracing/log-выводе всех крейтов отсутствуют: ключи, wrap-содержимое, полные адреса whitelist (только 6+6), сырые intent'ы (только intent_hash), тела кадров M8.
- **Механизм:** типы секретов без Debug/Display (RISK-M4-1); Amount/CanonAddress имеют редактирующие Display (адрес → `0xAbC…123`); обёртка `Redacted<T>` для полей структур логирования.
- **Тест:** снапшот-тест логов интеграционного прогона + grep-гейт по паттернам (32-байтный hex, полный адрес) в артефактах CI.

## 4. Тест-матрица X
Линт-гейты (float, arithmetic, wildcard, grep-секреты) — блокирующие; property parse_decimal; компайл-тесты «двух лиц» enum; TestClock-сценарии.

---

# Сводная трассируемость (закрывающая; вместе с двумя предыдущими документами — полная карта)

| RISK | Инвариант (кратко) | Доказывающий тест | Класс потерь |
|---|---|---|---|
| M1-1 | Ω неизменяем вне reissue | compile-fail + мутационный | пробитие потолка |
| M1-2 | rollback-атака на μ/лог ловится | матрица откатов ×3 | откат лимитов |
| M1-3 | μ атомарен на диске | kill-матрица ×4 точки | недоступность |
| M2-1 | checked-арифметика суммы | границы u128 | пробитие потолка |
| M3-1 | Reservation линейна | compile-fail + drop-тест | утечка резерва |
| M3-2 | membership по CanonAddress | property представлений | обход whitelist |
| M3-3 | human-порог = резервная сумма | границы threshold+gas | обход владельца |
| M7S-1 | Simulated ∉ реальный учёт | property изоляции | ложный учёт |
| M7S-2 | стаб недоступен вне demo | матрица 8/8 | «оплата» в никуда |
| X-1 | нет float в деньгах | линт+grep гейт | ошибки округления |
| X-2 | два времени не смешаны | compile-level + TestClock | обход TTL/окна |
| X-3 | нет wildcard на критичных enum | линт + мутационный | тихие провалы веток |

Итоговое правило комплекта (все 4 документа): карта RISK-секций (34 шт.) — это и есть чек-лист security-ревью MVP; внешний аудит проводится не «по коду вообще», а по трассируемости инвариант→механизм→тест, начиная с 🔴 (19 шт.).
