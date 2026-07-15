//! C5 mu-compose — CLI-фронтенд Composer (спека F-09).
//! Команды: show | set-limit <v> | set-threshold <v> | wl-add <addr> <label> | wl-rm <addr>
//!          | diff | propose
//! Работает с локальным черновиком; propose шлёт кадр в unix socket демона.
//! В отсутствие демона (--socket) команды кроме propose работают offline (F-08).
use composer_core::client::{read_frame, write_frame};
use composer_core::drafts::{load_draft, save_draft};
use composer_core::proposal::{encode_proposal, DeltaProposal};
use mu_common::{Amount, parse_decimal};
use mu_policy::{canon_address_checked, delta_hash, validate, Delta, OmegaView, WlEntry};
use std::path::PathBuf;

const DECIMALS: u8 = 6; // USDC
const CHAIN: u64 = 8453;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = run(&args);
    std::process::exit(code);
}

fn draft_path() -> PathBuf {
    std::env::var("MU_HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(".")).join("draft.bin")
}

fn omega_stub() -> OmegaView {
    // offline-режим: потолок берётся из env (в проде — из GetPolicy демона)
    let ceil = std::env::var("MU_CEILING").ok()
        .and_then(|s| parse_decimal(&s, DECIMALS).ok())
        .unwrap_or(Amount::from_minor(1_000_000_000));
    OmegaView { max_ceiling: ceil, supported_chain_ids: vec![CHAIN] }
}

fn load_or_default() -> DeltaProposal {
    load_draft(&draft_path()).ok().flatten().unwrap_or(DeltaProposal {
        new_delta: Delta { daily_limit: Amount::ZERO, whitelist: vec![], confirm_threshold: Amount::ZERO },
        base_delta_hash: mu_common::Hash32([0; 32]),
        ts: 0,
    })
}

fn save(p: &DeltaProposal) -> i32 {
    match save_draft(p, &draft_path()) {
        Ok(()) => { println!("draft saved: {}", draft_path().display()); 0 }
        Err(e) => { eprintln!("draft save error: {e:?}"); 2 }
    }
}

fn fmt(a: Amount) -> String { mu_common::amount::display_minor(a, DECIMALS) }

fn run(args: &[String]) -> i32 {
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    match cmd {
        "show" => {
            let p = load_or_default();
            println!("daily_limit:       {}", fmt(p.new_delta.daily_limit));
            println!("confirm_threshold: {}", fmt(p.new_delta.confirm_threshold));
            println!("whitelist ({}):", p.new_delta.whitelist.len());
            for e in &p.new_delta.whitelist {
                println!("  {}  {}", e.address.redacted(), e.label);
            }
            let rep = validate(&p.new_delta, &omega_stub());
            for w in &rep.warnings { println!("warn: {w:?}"); }
            for e in &rep.errors { println!("ERROR: {e:?}"); }
            0
        }
        "set-limit" | "set-threshold" => {
            let Some(v) = args.get(1) else { eprintln!("usage: {cmd} <amount>"); return 1; };
            let Ok(a) = parse_decimal(v, DECIMALS) else { eprintln!("bad amount: {v}"); return 1; };
            let mut p = load_or_default();
            if cmd == "set-limit" { p.new_delta.daily_limit = a; } else { p.new_delta.confirm_threshold = a; }
            save(&p)
        }
        "wl-add" => {
            let (Some(addr), Some(label)) = (args.get(1), args.get(2)) else {
                eprintln!("usage: wl-add <address> <label>"); return 1;
            };
            // E-ADR-01/02: канонизация с EIP-55 (S3-логика: показать усечённо)
            let (canon, warn) = match canon_address_checked(addr, CHAIN) {
                Ok(x) => x,
                Err(e) => { eprintln!("address rejected: {e:?}"); return 1; }
            };
            if warn { println!("warn: адрес без EIP-55 чексуммы (W-ADR-01) — сверьте: {}", canon.redacted()); }
            println!("adding: {}", canon.redacted());
            let mut p = load_or_default();
            p.new_delta.whitelist.push(WlEntry { address: canon, label: label.clone() });
            let rep = validate(&p.new_delta, &omega_stub());
            if !rep.ok() { eprintln!("validation errors: {:?}", rep.errors); return 1; }
            save(&p)
        }
        "wl-rm" => {
            let Some(addr) = args.get(1) else { eprintln!("usage: wl-rm <address>"); return 1; };
            let Ok((canon, _)) = canon_address_checked(addr, CHAIN) else { eprintln!("bad address"); return 1; };
            let mut p = load_or_default();
            let before = p.new_delta.whitelist.len();
            p.new_delta.whitelist.retain(|e| e.address != canon);
            if p.new_delta.whitelist.len() == before { eprintln!("not found"); return 1; }
            save(&p)
        }
        "propose" => {
            let p = load_or_default();
            let rep = validate(&p.new_delta, &omega_stub());
            if !rep.ok() { eprintln!("blocked by validation: {:?}", rep.errors); return 1; }
            let Ok(sock_path) = std::env::var("MU_POLICY_SOCK") else {
                eprintln!("MU_POLICY_SOCK not set (демон недоступен → offline, черновик сохранён)");
                return 3;
            };
            let raw = match encode_proposal(&p) {
                Ok(r) => r,
                Err(e) => { eprintln!("encode: {e:?}"); return 2; }
            };
            #[cfg(unix)]
            {
                use std::os::unix::net::UnixStream;
                let mut s = match UnixStream::connect(&sock_path) {
                    Ok(s) => s,
                    Err(e) => { eprintln!("connect: {e}"); return 3; }
                };
                if let Err(e) = write_frame(&mut s, &raw) { eprintln!("send: {e:?}"); return 3; }
                match read_frame(&mut s) {
                    Ok(resp) => { println!("daemon: {}", String::from_utf8_lossy(&resp)); 0 }
                    Err(e) => { eprintln!("recv: {e:?}"); 3 }
                }
            }
            #[cfg(not(unix))]
            { eprintln!("named pipe transport: платформенная сборка"); 3 }
        }
        "hash" => {
            let p = load_or_default();
            let h = delta_hash(&p.new_delta);
            println!("delta_hash: {}", h.0.iter().map(|b| format!("{b:02x}")).collect::<String>());
            0
        }
        _ => {
            println!("mu-compose: show | set-limit <v> | set-threshold <v> | wl-add <a> <l> | wl-rm <a> | hash | propose");
            0
        }
    }
}
