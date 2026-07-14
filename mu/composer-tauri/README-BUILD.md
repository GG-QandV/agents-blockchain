# Сборка Δ-Composer (часть 2)

## Требования
- Rust ≥ 1.77 (Tauri 2)
- Linux: libwebkit2gtk-4.1-dev, libgtk-3-dev; macOS: Xcode CLT; Windows: WebView2 Runtime
- `cargo install tauri-cli --version ^2`

## Сборка
```
cd composer-tauri
cargo tauri dev      # разработка
cargo tauri build    # дистрибутивы (deb/dmg/msi)
```

## Переменные окружения
- MU_HOME — каталог черновика (draft.bin)
- MU_POLICY_SOCK — unix socket policy-endpoint демона (без него — offline-режим)
- MU_CEILING — потолок Ω для offline-валидации (в проде приходит из GetPolicy)

## Ограничения выполнения (часть 2)
1. composer-tauri НЕ собран в контейнере: Tauri 2 требует Rust 1.77+ и системный
   webkit — здесь Rust 1.75. Rust-glue написан по API Tauri 2; JS проверен node --check.
2. UI — vanilla JS (не TS из N-02): без сборщика, аудит по одному файлу; типы JSDoc.
3. Формат proposal — hardened-TLV (семейство mu-wire), не CBOR из §5.1 спеки:
   один парсер на все входы демона (RISK-M8-3); мост на CBOR — механическая замена
   encode/decode в composer-core/proposal.rs.
4. GetPolicy-канал (F-01/F-07) в glue упрощён: Ω из env, история не показана —
   подключается тем же кадрированием client.rs при появлении endpoint'а в демоне.
5. Экраны S5 (ожидание TTL-таймер) и S6 (история) — заготовки: S5 сведён к строке
   результата, S6 требует GetPolicy. Ядро экранов S1–S4 полное.
