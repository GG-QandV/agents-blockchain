# SPEC: модули M6 mu-runtime, M5 mu-log, M9 mu-human — v1.0 (MVP)

Статус: Draft для реализации • Дата: 2026-07-12 • Система: μ-agentic MVP v3
Формат RISK-секций: 🔴 инвариант, нарушение = деньги/несанкционированные операции; 🟡 нарушение = слепота/отказ. Каждая секция: инвариант → механизм принуждения → доказывающий тест.
Связанные документы: SPEC_M4_M7a_M8_v1 (граница секретов, коннектор, периметр), SPEC_Delta-Composer_v1.

Особенность тройки: M6/M5 — модули **тихих отказов** (система выглядит рабочей, но считает/действует неверно), M9 — модуль **последней мили доверия** (то, что видит человек, должно быть тем, что исполнится).

---

# ЧАСТЬ I. M6 mu-runtime — оркестратор

## 1. Назначение
Держатель всех инвариантов последовательности. Сам не хранит состояния денег и не содержит бизнес-правил — только порядок, таймауты и жизненный цикл. Ранг чувствительности №2: ошибка порядка обнуляет гарантии M3/M5/M7.

## 2. Публичный API

```rust
pub fn start(cfg: Config) -> Result<RuntimeHandle, StartErr>;   // полный boot-протокол §4
impl RuntimeHandle {
    pub fn submit(&self, i: SignedIntent) -> Ticket;            // неблокирующий
    pub fn status(&self, t: Ticket) -> IntentStatus;
    pub fn shutdown(self, grace: Duration) -> ShutdownReport;   // §6
}
pub enum IntentStatus { Queued, AwaitingHuman { deadline: u64 }, Executing,
                        Settled { tx_hash: B256 }, Denied { code: DenyCode },
                        Failed { code: FailCode }, ReconcilePending }
```

## 3. Машина состояний intent'а (нормативная)

```
                 ┌──────────── Denied(Ω) ──────────────┐
Queued → ΩCheck ─┤                                      ├→ terminal
                 └→ ΔReserve ─┬─ Denied(Δ) ─────────────┘
                              ├─ needs_human → AwaitingHuman ─┬─ Timeout/Deny → Rollback → Denied
                              │                               └─ Approved ↓
                              └────────────────────────────────→ WalPending   ← ЕДИНСТВЕННЫЙ вход к деньгам
                                                                     ↓
                                                                Executing (M7a.execute)
                                                                     ↓
                                    ┌── Ok(TxRef) → PollStatus ──┬─ Settled → Commit → terminal
                                    │                            └─ >finality_ms → ReconcilePending
                                    └── Rejected → Failed → Rollback → terminal
                                        Unknown  → ReconcilePending (резерв ДЕРЖИТСЯ)
```

**Типовое принуждение (typestate):** переходы кодируются типами-состояниями; функция `execute_payment(w: WalWritten) -> Executing` принимает доказательство-тип `WalWritten`, который порождается только `write_wal()`. Вызов execute без WAL **не компилируется**. Аналогично `Commit(SettledProof)` требует тип, порождаемый только консенсусом status() из M7a.

## 4. Boot-протокол (строгий порядок)

```
1. M1.load + verify(μ)            ── ✗ → exit(2) + отчёт          [никакого частичного старта]
2. M5.open + verify_chain          ── ✗ → exit(3)
3. M5.pending() → M7a.status(...) → reconcile-résolution (§ M5 RISK-5-3)
   сеть недоступна → цикл ретраев с backoff; M8 НЕ открыт
4. восстановление nonce-окон агентов из лога (для M8)
5. только теперь M8.listen + policy-endpoint
```

## 5. RISK-секции M6

### RISK-M6-1 🔴 Нарушение порядка «WAL до денег»
- **Инвариант:** M7a.execute вызывается только после успешного fsync записи Pending в M5.
- **Механизм:** typestate `WalWritten` (§3); линт-правило CI: прямой вызов `connector.execute` разрешён единственному файлу pipeline.rs (clippy-запрет по пути + CODEOWNERS).
- **Тест:** compile-fail: попытка вызвать execute из другого модуля/без WalWritten; chaos: kill −9 сразу после fsync-Pending и до execute → рестарт → reconcile: nonce не занят → Failed+rollback, денег нет, учёт сходится.

### RISK-M6-2 🔴 Commit без финальности / Rollback после отправки
- **Инвариант:** `Δ.commit` только при `TxStatus::Settled` (консенсус 2 RPC, см. M7-3); `Δ.rollback` только для терминальных исходов, при которых tx гарантированно не в сети (Denied, Rejected, reconcile-доказательство «nonce не занят»).
- **Механизм:** rollback принимает enum `RollbackCause` с закрытым списком; вариант «таймаут/Unknown» в этом enum отсутствует типом.
- **Тест:** таблица переходов: для каждого не-терминального состояния попытка commit/rollback — не компилируется или паникует в debug; сценарий Unknown → резерв держится, window_sum включает Pending.

### RISK-M6-3 🔴 Возврат конкурентности при рефакторинге
- **Инвариант:** между ΔReserve и Commit/Rollback одного intent'а не начинается ΔReserve другого.
- **Механизм:** однопоточный worker (mpsc); дополнительно M3 держит `in_flight`-флаг и возвращает Busy (второй эшелон, см. спек M3); стресс-инвариант закреплён тестом, а не только архитектурой.
- **Тест:** 100 конкурентных submit по 60% лимита на SoftVault+stub → сумма Settled+Pending ≤ daily_limit во всех прогонах (property, 10³ итераций CI).

### RISK-M6-4 🟡 Частичный старт / работа на битом состоянии
- **Инвариант:** приём intent'ов невозможен, пока шаги boot 1–4 не завершены успешно; любой ✗ — fail-stop с кодом и человекочитаемым отчётом.
- **Механизм:** M8.listen вызывается последним и только из успешной ветки; отсутствие «режима деградации с приёмом» by design.
- **Тест:** матрица порчи: битый μ / битый хвост лога / RPC down → сокет-файл не создан (connect → ENOENT).

### RISK-M6-5 🟡 Подвисший intent навсегда (утечка резерва)
- **Инвариант:** каждый intent достигает терминального состояния или ReconcilePending за ограниченное время; резерв не «протухает» молча.
- **Механизм:** таймауты: human TTL (конфиг, 15 мин) → rollback; finality_ms → ReconcilePending; ReconcilePending-записи ревизируются фоновым тикером reconcile каждые 60 с; в status() владельцу виден возраст записи.
- **Тест:** симуляция вечного Pending от RPC → через finality_ms статус ReconcilePending, резерв держится, тикер продолжает опрос; после «появления» receipt → Settled+Commit.

### RISK-M6-6 🟡 Shutdown посреди конвейера
- **Инвариант:** graceful shutdown не создаёт состояний, отличных от достижимых при крэше (крэш-эквивалентность).
- **Механизм:** shutdown = стоп M8 → дожидание текущего intent'а до ближайшей WAL-границы (не до финальности) → снапшот nonce-окон → fsync → exit. Никаких «специальных» состояний.
- **Тест:** chaos-раннер сравнивает множество достижимых состояний после kill −9 и после shutdown в тех же точках — множества совпадают.

## 6. Наблюдаемость (минимум MVP)
Статусы и возрасты: queued_len, awaiting_human, reconcile_pending{age}, rpc_divergence, last_boot_report. Экспорт — только в M9 (никаких сетевых метрик).

## 7. Тест-матрица M6 (сводно)
Compile-fail (typestate) ×4; property конкурентности; chaos kill-points: 9 точек конвейера × (kill, рестарт, сверка инвариантов денег/лога); крэш-эквивалентность shutdown. Гейт CI: chaos-сьют зелёный 50 прогонов подряд.

---

# ЧАСТЬ II. M5 mu-log — WAL и источник истины

## 1. Назначение
Единственный источник истины для: резервов/окна Δ, nonce-восстановления, reconcile и аудита владельца. Ранг №3; главный враг — **тихая порча**: система работает, но считает неверно.

## 2. Формат хранения

```
файл:  $MU_HOME/log/segment-<n>.mulog   (append-only, ротация по 8 МиБ)
запись: len:u32 LE ‖ cbor(Entry) ‖ pad→8B
Entry = { 1:seq u64, 2:prev_hash 32B, 3:ts u64, 4:kind Kind, 5:sig 64B }
   sig = P256(vault.sign_mu, domain="mu.log.v1", SHA256(seq‖prev_hash‖ts‖kind))
Kind: DeniedOmega, DeniedDelta, Pending{intent_hash, recipient, amount, gas_res, chain_nonce},
      Settled{tx_hash, effective_gas}, Failed{code}, Simulated{connector},
      DeltaChanged{old_hash,new_hash}, HumanDecision{approved, intent_hash},
      TailTruncated{lost_from_seq}, Alert{code}, NonceSnapshot{agent_id,nonce}
head-файл: $MU_HOME/log/HEAD = {last_seq, last_hash} (атомарная перезапись, копия в μ.log_head при M1.save)
```

## 3. Публичный API

```rust
fn append(&mut self, k: Kind) -> Result<Hash, LogErr>;          // fsync до возврата
fn verify_chain(&self) -> Result<ChainReport, LogErr>;           // весь лог при boot
fn pending(&self) -> Vec<EntryRef>;                              // Pending без парного Settled/Failed
fn window_sum(&self, now: u64) -> Amount;                        // Σ amount+gas: Settled∪Pending, ts>now−86400
fn reconcile(&mut self, c: &dyn ConnectorStatus) -> ReconcileReport;
fn iter(&self, f: Filter) -> impl Iterator<Item=EntryView>;      // для M9, read-only
```

## 4. RISK-секции M5

### RISK-M5-1 🔴 Потеря Pending при крэше (пробитие лимита)
- **Инвариант:** если M7a.execute мог быть вызван, Pending-запись durable (пережила крэш).
- **Механизм:** append = write → fsync(file) → обновить HEAD → fsync; возврат Ok только после обоих fsync; WalWritten-тип (M6-1) порождается этим Ok. На платформах с ложным fsync (некоторые FS) — O_DSYNC-открытие + документированное требование к ФС.
- **Тест:** chaos с эмуляцией отключения питания (dm-flakey / CrashMonkey-подход): 10³ циклов «append→kill в случайной микроточке» → после рекавери либо запись целиком есть, либо её нет И execute не вызывался (сверка с мок-коннектором, считающим вызовы).

### RISK-M5-2 🔴 Тихая порча/фальсификация истории
- **Инвариант:** удаление, правка или перестановка любой записи детектируется при boot; продолжение работы на неверифицированном логе невозможно.
- **Механизм:** hash-chain (prev_hash) + подпись каждой записи ключом μ (домен mu.log.v1 — см. RISK-M4-5) + seq-монотонность + сверка последнего hash с HEAD и с μ.log_head (три независимых точки: файл, HEAD, μ-объект). Расхождение HEAD↔цепь допустимо только «HEAD отстал на ≤1» (крэш между fsync'ами) — авто-починка HEAD; всё остальное → halt.
- **Тест:** матрица атак: удалить запись из середины / поменять amount в Pending / переставить два сегмента / откатить лог+HEAD на старый снапшот (rollback-атака ловится μ.log_head из M1) → verify_chain=Err во всех случаях.

### RISK-M5-3 🔴 Неверная résolution при reconcile
- **Инвариант:** Pending закрывается Failed только при доказательстве неисполнения: chain_nonce уже израсходован ДРУГОЙ транзакцией с ≥confirmations, либо tx нет в сети И nonce аккаунта < chain_nonce записи. Во всех прочих случаях Pending остаётся.
- **Механизм:** решающая функция — чистая, принимает (запись, ответы обоих RPC) → enum Resolution{ToSettled, ToFailed, Keep}; табличная реализация без «else → Failed»-веток (default = Keep).
- **Тест:** полный перебор таблицы входов (receipt есть/нет ×2 RPC, nonce занят той же/другой tx/свободен) — 18 комбинаций, каждая закреплена unit-тестом; расхождение RPC → Keep+Alert.

### RISK-M5-4 🟡 Ошибка окна 24 ч (недосчёт/пересчёт лимита)
- **Инвариант:** window_sum монотонно корректен относительно записей: включает все Settled и Pending с ts в окне, Simulated исключён, Failed исключён.
- **Механизм:** единственная реализация в M5 (Δ не считает сама); Clock-trait инжектится; ts записей — от Clock демона, не от агента.
- **Тест:** property: генерация случайных историй → window_sum == референс-модель на BTreeMap; границы ts=now−86400±1; Simulated-записи не влияют.

### RISK-M5-5 🟡 Деградация обрезкой хвоста
- **Инвариант:** повреждённый хвост сегмента (недописанная запись) обрезается до последней валидной записи, факт фиксируется записью TailTruncated, потерянный диапазон известен.
- **Механизм:** при boot скан хвоста: len-префикс за EOF / битый CBOR / битая подпись последней записи → truncate + TailTruncated{lost_from_seq}; если потерянная зона могла содержать Pending (по HEAD-разрыву) → принудительный полный reconcile до открытия M8.
- **Тест:** порча хвоста всеми способами (обрыв на len / на теле / на sig) → рекавери с корректным lost_from_seq; лог с TailTruncated проходит verify_chain.

### RISK-M5-6 🟡 Рост и производительность
- **Инвариант:** boot-verify и window_sum не деградируют неприемлемо с ростом лога.
- **Механизм:** сегментация 8 МиБ; verify_chain кэширует hash границ сегментов в HEAD (полная проверка — по флагу/раз в N бутов); window_sum — по последнему сегменту + индекс ts в памяти.
- **Тест:** бенч 10⁵ записей: boot < 2 с, window_sum < 10 мс, append p99 < 30 мс (с fsync, SSD).

## 5. Тест-матрица M5 (сводно)
CrashMonkey-цикл (гейт: 10³ clean); матрица фальсификаций ×6; таблица reconcile 18/18; property окна; бенчи. Ревью-правило: любые изменения в append/verify — только с повторным полным chaos-прогоном.

---

# ЧАСТЬ III. M9 mu-human — канал владельца

## 1. Назначение
Последняя миля доверия: биометрическое подтверждение платежей > threshold, применение Δ-изменений (в паре с Composer), просмотр журнала. Ранг №8 по LOC-весу, но **№1 по социальной поверхности**: всё, что обманет глаза владельца, обходит всю криптографию ниже.

## 2. Публичный API

```rust
fn confirm_payment(req: PayConfirm, ttl: Duration) -> HumanDecision;   // Approved{auth_proof} | Denied | Timeout
fn confirm_delta(req: DeltaConfirm, ttl: Duration) -> HumanDecision;
fn view_log(f: LogFilter) -> Vec<EntryView>;                            // M5.iter, read-only
struct PayConfirm { recipient: CanonAddress, wl_label: Option<String>, amount: Amount,
                    asset: AssetId, gas_est: Amount, agent_id: String,
                    remaining_window: Amount, intent_hash: Hash }
struct DeltaConfirm { diff: DeltaDiff /*old→new построчно*/, proposal_hash: Hash, warnings: Vec<VWarn> }
enum HumanDecision { Approved { auth_proof: AuthProof }, Denied, Timeout }
```

`AuthProof` — криптографическое доказательство платформенной аутентификации, привязанное к содержимому (см. RISK-M9-2), а не булев флаг.

## 3. Платформенные бэкенды диалога
LocalAuthentication (macOS/iOS: LAContext + evaluatedPolicyDomainState), BiometricPrompt+CryptoObject (Android), Windows Hello (NCrypt user-key). Общее требование: подпись owner-ключом возможна только при auth-required ключе (см. RISK-M4-3) — M9 не «проверяет биометрию сам», он инициирует платформенный обряд, разблокирующий ключ.

## 4. RISK-секции M9

### RISK-M9-1 🔴 Диалог показывает не то, что исполнится
- **Инвариант:** байты, отображённые владельцу, и байты, ушедшие в исполнение/подпись, — производные одной и той же структуры, полученной M9 из конвейера M6 (для платежей) или из DeltaProposal (для Δ); никаких строк от агента/Composer в диалоге.
- **Механизм:** PayConfirm собирается в M6 из intent'а, УЖЕ прошедшего Ω/Δ (тот же объект, что пойдёт в M7a; связка — intent_hash, который затем пишется в HumanDecision-запись лога и сверяется в pipeline перед execute); поле purpose агента в диалоге НЕ показывается (неверифицируемо, вектор соц. инженерии) — вместо него wl_label из Δ; рендер — нативные контролы, форматирование сумм — mu-policy (та же decimal-логика, что везде).
- **Тест:** мутационный: подмена recipient между confirm и execute → pipeline-сверка intent_hash из HumanDecision падает, исполнения нет; UI-снапшот-тесты: для эталонных PayConfirm рендер побайтово стабилен; тест «purpose со строкой 'Отправка самому себе, жмите ОК'» — строка нигде не отображается.

### RISK-M9-2 🔴 Approved без реального обряда (подделка решения)
- **Инвариант:** HumanDecision::Approved непроизводим программно — только через платформенную аутентификацию, криптографически связанную с intent_hash/proposal_hash.
- **Механизм:** auth_proof = подпись owner-ключом (auth-required, RISK-M4-3) над `domain="mu.human.v1" ‖ hash`; конвейер M6/применение Δ верифицируют подпись перед продолжением; таким образом даже полная компрометация кода M9 не порождает валидный Approved.
- **Тест:** мок-M9, возвращающий Approved с мусорным proof → M6 отклоняет; device-smoke: отмена биометрии → owner_sign даёт UserAuthFailed → Denied.

### RISK-M9-3 🔴 Спуфинг содержимого через label/Unicode
- **Инвариант:** wl_label и любые отображаемые строки не могут визуально фальсифицировать адрес/сумму/направление.
- **Механизм:** label прошёл E-LBL-01 при внесении в whitelist (запрет управляющих/bidi, NFC — см. спек Composer §7); в диалоге label всегда сопровождается укрупнёнными 6+6 символами канонического адреса + identicon; суммы рендерятся из Amount, не из строк.
- **Тест:** корпус гомоглифов/bidi в label → отклонены на входе в whitelist; снапшот диалога: адресная строка присутствует при любом label.

### RISK-M9-4 🟡 Усталость от подтверждений (rubber-stamping)
- **Инвариант (продуктовый):** частота диалогов не приучает владельца жать «да» не глядя.
- **Механизм:** порог — ответственность Δ (W-THR-01 предупреждает о threshold=0); в диалоге выделяется аномальность: новый получатель (первый платёж на адрес — бейдж «ПЕРВЫЙ ПЛАТЁЖ»), остаток окна < 20% — бейдж; никаких «подтвердить всё»/запоминания решения в MVP.
- **Тест:** UI-тест бейджей; ревью-чеклист: в кодовой базе нет путей мимо confirm при amount>threshold (grep-гейт + таблица переходов M6).

### RISK-M9-5 🟡 TTL и жизненный цикл диалога
- **Инвариант:** по истечении TTL решение = Timeout ровно один раз; поздний тап владельца после Timeout не превращается в Approved.
- **Механизм:** дедлайн вычисляется в M6 и передаётся в req; M9 закрывает диалог сам по монотонным часам; M6 игнорирует HumanDecision, пришедший после rollback (сверка по intent_hash со state-машиной — поздний Approved попадает в лог как Alert, не исполняется).
- **Тест:** гонка «тап на 899-й/901-й секунде»: до дедлайна — Approved исполняется; после — Alert, rollback уже совершён, исполнения нет.

### RISK-M9-6 🟡 view_log как канал утечки/искажения
- **Инвариант:** просмотр журнала read-only, без секретов, и отображает верифицированную цепь.
- **Механизм:** EntryView не содержит sig/prev_hash в редактируемом виде; рендер только записей, прошедших verify_chain текущего boot; экспорт журнала (файл для аудита) — сырые подписанные записи, проверяемые внешним верификатором (утилита в комплекте, ~80 LOC).
- **Тест:** EntryView-тип не содержит полей секретов (compile-level); экспорт → внешний верификатор подтверждает цепь.

## 5. Тест-матрица M9 (сводно)
Мутационная связка confirm↔execute (intent_hash); device-smoke биометрии ×3 платформы; корпус Unicode-спуфинга; гонки TTL; снапшоты диалогов. Гейт: RISK-M9-1/2 тесты — блокирующие для любого PR в M9 или pipeline.rs.

---

# Сквозная таблица трассируемости (дополнение к SPEC_M4_M7a_M8)

| RISK | Инвариант (кратко) | Доказывающий тест | Класс потерь |
|---|---|---|---|
| M6-1 | execute ⇐ WAL fsync | compile-fail + kill-point | двойная трата/потеря учёта |
| M6-2 | commit ⇐ финальность; rollback ⇐ доказанный неуход | таблица переходов | пробитие лимита |
| M6-3 | сериализация Reserve→Commit | property 10³ прогонов | пробитие лимита |
| M6-6 | shutdown ≡ crash | сравнение множеств состояний | скрытые состояния |
| M5-1 | Pending durable до денег | CrashMonkey 10³ | пробитие лимита |
| M5-2 | история нефальсифицируема | матрица атак + anti-rollback | слепой аудит |
| M5-3 | Failed ⇐ доказательство неисполнения | 18/18 таблица | двойная трата |
| M5-4 | окно 24ч корректно | property vs референс | пробитие лимита |
| M9-1 | видимое = исполняемое | мутационный intent_hash | несанкц. платёж |
| M9-2 | Approved непроизводим кодом | мусорный proof отклонён | обход владельца |
| M9-3 | label не спуфит адрес | корпус Unicode | несанкц. платёж |

Правило приёмки: как в SPEC_M4_M7a_M8 — PR в зоне RISK не мержится без зелёного соответствующего теста; для тройки M6/M5/M9 дополнительно: chaos-сьют (M6+M5) и мутационная связка M9-1 входят в ночной CI, красный запуск блокирует релизную ветку.
