# Инструкция агенту: сборка Δ-Composer в рабочее репо

## Источник
`mu-v1.tar.gz` → каталоги: `mu-policy/`, `composer-core/`, `composer-cli/` (часть 1, внутри workspace) и `composer-tauri/` (часть 2, ВНЕ workspace — у него свой `[workspace]` в Cargo.toml, не убирать).

## Шаги

1. **Распаковать и проверить часть 1** (Rust ≥ 1.75):
```
tar -xzf mu-v1.tar.gz && cd mu
cargo test -p mu-policy -p composer-core          # ожидание: 20 passed
cargo build -p composer-cli                        # бинарь target/debug/mu-compose
```
2. **Смоук CLI**:
```
export MU_HOME=/tmp/mu && mkdir -p $MU_HOME
./target/debug/mu-compose set-limit 500
./target/debug/mu-compose wl-add 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed "API"
./target/debug/mu-compose show                     # лимит 500.000000, 1 адрес, без ERROR
```
3. **Часть 2 (GUI)** — на машине с GUI-окружением:
- Rust ≥ **1.77**, `cargo install tauri-cli --version ^2`
- Linux: `libwebkit2gtk-4.1-dev libgtk-3-dev`; macOS: Xcode CLT; Windows: WebView2
```
cd composer-tauri
cargo tauri dev        # проверка: форма, добавление адреса через диалог сверки
cargo tauri build      # дистрибутивы
```
4. **Переменные**: `MU_HOME` (черновик), `MU_POLICY_SOCK` (сокет демона; без него — offline), `MU_CEILING` (потолок Ω offline).

## Запрещено менять
- Логику валидации где-либо, кроме **mu-policy** (единственный источник правды: демон и Composer линкуют один crate).
- Версии крипто-крейтов в Cargo.toml без прогона тестов (пины под toolchain).

## Известные отклонения (уже задокументированы в README-BUILD)
Proposal = TLV (не CBOR §5.1) — мост в одном файле `composer-core/src/proposal.rs`; UI = vanilla JS (не TS); экраны S5/S6 — заготовки до GetPolicy-endpoint демона.

## Definition of Done
□ 20 тестов части 1 зелёные □ CLI-смоук пройден □ `cargo tauri build` даёт дистрибутив □ в UI битая EIP-55 чексумма отклоняется, кнопка «Отправить» заблокирована при ошибках валидации.
