//! C1 Tauri-glue: тонкие команды над composer-core/mu-policy.
//! Вся логика — в проверенных крейтах части 1; здесь только маршалинг (спека §4.1).
//! Никакой сети: только draft-файл и unix socket демона (N-04).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use composer_core::client::{read_frame, write_frame};
use composer_core::drafts::{load_draft, save_draft};
use composer_core::proposal::{encode_proposal, DeltaProposal};
use mu_common::{parse_decimal, Amount, CanonAddress, Hash32};
use mu_policy::{canon_address_checked, delta_hash, validate, Delta, OmegaView, WlEntry};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const DECIMALS: u8 = 6;
const CHAIN: u64 = 8453;

// ── DTO для UI (без секретов; адреса — hex + redacted) ────────────────────
#[derive(Serialize, Deserialize, Clone)]
struct WlDto { address_hex: String, redacted: String, label: String }
#[derive(Serialize, Deserialize, Clone)]
struct DeltaDto { daily_limit: String, confirm_threshold: String, whitelist: Vec<WlDto> }
#[derive(Serialize)]
struct ReportDto { errors: Vec<String>, warnings: Vec<String> }
#[derive(Serialize)]
struct StateDto { delta: DeltaDto, report: ReportDto, delta_hash: String, daemon: bool }

fn draft_path() -> PathBuf {
    std::env::var("MU_HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(".")).join("draft.bin")
}
fn omega_view() -> OmegaView {
    // MVP: потолок из env; при живом демоне — из GetPolicy (F-01, фаза интеграции)
    let ceil = std::env::var("MU_CEILING").ok()
        .and_then(|s| parse_decimal(&s, DECIMALS).ok())
        .unwrap_or(Amount::from_minor(1_000_000_000));
    OmegaView { max_ceiling: ceil, supported_chain_ids: vec![CHAIN] }
}

fn to_dto(d: &Delta) -> DeltaDto {
    DeltaDto {
        daily_limit: mu_common::amount::display_minor(d.daily_limit, DECIMALS),
        confirm_threshold: mu_common::amount::display_minor(d.confirm_threshold, DECIMALS),
        whitelist: d.whitelist.iter().map(|e| WlDto {
            address_hex: format!("0x{}", e.address.to_hex()),
            redacted: e.address.redacted(),
            label: e.label.clone(),
        }).collect(),
    }
}
fn from_dto(d: &DeltaDto) -> Result<Delta, String> {
    let daily = parse_decimal(&d.daily_limit, DECIMALS).map_err(|e| format!("limit: {e:?}"))?;
    let thr = parse_decimal(&d.confirm_threshold, DECIMALS).map_err(|e| format!("threshold: {e:?}"))?;
    let mut wl = Vec::new();
    for e in &d.whitelist {
        let a = CanonAddress::canon(&e.address_hex, CHAIN).map_err(|x| format!("addr: {x:?}"))?;
        wl.push(WlEntry { address: a, label: e.label.clone() });
    }
    Ok(Delta { daily_limit: daily, whitelist: wl, confirm_threshold: thr })
}
fn report_dto(d: &Delta) -> ReportDto {
    let r = validate(d, &omega_view());
    ReportDto {
        errors: r.errors.iter().map(|e| format!("{e:?}")).collect(),
        warnings: r.warnings.iter().map(|w| format!("{w:?}")).collect(),
    }
}
fn hash_hex(d: &Delta) -> String {
    delta_hash(d).0.iter().map(|b| format!("{b:02x}")).collect()
}

// ── Tauri-команды ──────────────────────────────────────────────────────────
#[tauri::command]
fn get_state() -> StateDto {
    let p = load_draft(&draft_path()).ok().flatten().unwrap_or(DeltaProposal {
        new_delta: Delta { daily_limit: Amount::ZERO, whitelist: vec![], confirm_threshold: Amount::ZERO },
        base_delta_hash: Hash32([0; 32]),
        ts: 0,
    });
    StateDto {
        report: report_dto(&p.new_delta),
        delta_hash: hash_hex(&p.new_delta),
        delta: to_dto(&p.new_delta),
        daemon: std::env::var("MU_POLICY_SOCK").is_ok(),
    }
}

/// Валидация на каждый ввод (F-03) — UI шлёт всю форму, получает отчёт.
#[tauri::command]
fn validate_form(delta: DeltaDto) -> Result<ReportDto, String> {
    Ok(report_dto(&from_dto(&delta)?))
}

/// Канонизация адреса при вставке (S3): redacted + identicon-seed.
#[tauri::command]
fn check_address(input: String) -> Result<serde_json::Value, String> {
    match canon_address_checked(&input, CHAIN) {
        Ok((a, warn)) => Ok(serde_json::json!({
            "ok": true, "hex": format!("0x{}", a.to_hex()),
            "redacted": a.redacted(), "addr_warning": warn,
        })),
        Err(e) => Ok(serde_json::json!({ "ok": false, "error": format!("{e:?}") })),
    }
}

#[tauri::command]
fn save_form(delta: DeltaDto, base_hash: String) -> Result<String, String> {
    let d = from_dto(&delta)?;
    let mut base = [0u8; 32];
    if base_hash.len() == 64 {
        for i in 0..32 {
            base[i] = u8::from_str_radix(&base_hash[i * 2..i * 2 + 2], 16).map_err(|_| "bad hash")?;
        }
    }
    let p = DeltaProposal { new_delta: d, base_delta_hash: Hash32(base), ts: now() };
    save_draft(&p, &draft_path()).map_err(|e| format!("{e:?}"))?;
    Ok("saved".into())
}

/// Экспорт proposal демону (F-05). Ответ демона возвращается UI как есть.
#[tauri::command]
fn propose() -> Result<String, String> {
    let p = load_draft(&draft_path()).map_err(|e| format!("{e:?}"))?.ok_or("no draft")?;
    let rep = validate(&p.new_delta, &omega_view());
    if !rep.ok() {
        return Err(format!("blocked: {:?}", rep.errors)); // S4: экспорт только при 0 ошибок
    }
    let raw = encode_proposal(&p).map_err(|e| format!("{e:?}"))?;
    let sock = std::env::var("MU_POLICY_SOCK").map_err(|_| "daemon offline")?;
    #[cfg(unix)]
    {
        use std::os::unix::net::UnixStream;
        let mut s = UnixStream::connect(&sock).map_err(|e| e.to_string())?;
        write_frame(&mut s, &raw).map_err(|e| format!("{e:?}"))?;
        let resp = read_frame(&mut s).map_err(|e| format!("{e:?}"))?;
        Ok(String::from_utf8_lossy(&resp).to_string())
    }
    #[cfg(not(unix))]
    { Err("named pipe: платформенная ветка (Windows DACL)".into()) }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_state, validate_form, check_address, save_form, propose])
        .run(tauri::generate_context!())
        .expect("tauri run");
}
