//! P11e (Workflow μ Gate §3): the daemon's side of anchors — what a capsule's reader asks and what
//! the owner's rules answer.
//!
//! One JSON line in, one JSON line out, on a unix socket only its user opens (`serve_unix`):
//!
//! | request | answer |
//! |---|---|
//! | `{"anchor": {"memo", "cluster", "capsule", "fee_cap_lamports"?}}` | `{"sent": {"signature"}}` — paid and handed to the network |
//! | | `{"denied": {"reason": "budget" \| "cap" \| "not_allowed" \| "rejected"}}` — **not** paid: nothing was sent |
//! | | `{"unknown": {"signature"?}}` — the outcome is not known: ask the network about it, send nothing again |
//! | `{"status": {"signature"}}` | `{"landed": {"slot"}}` \| `{"pending": {}}` \| `{"failed": {}}` (both nodes say the blockhash is gone) |
//! | `{"address": {}}` | `{"address": "<base58>"}` — the one wallet that pays for every capsule, to fund it |
//!
//! One wallet pays for all the capsules (owner's decision 07.11, variant B; the sub-wallet per capsule
//! and its refill, P11f, are gone): what keeps one capsule from spending what belongs to the others is
//! the owner's rules — the budget of the day per capsule, the budget in all, the list of capsules that
//! may be paid for. When the wallet cannot pay what an anchor may cost, the owner is **told** (the
//! notifier, a log line) and the anchor is denied as `budget`.
//!
//! Every decision is a line of the log (a chain of hashes, signed by the vault's μ key, like the
//! payments'): sent (the **hash** of the signature, not the signature), denied (the reason), unknown.
//! A denial is not a failure of the page: the reader writes the page anyway and owes the anchor (S6).

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use mu_common::Hash32;
use mu_connect::anchor::AnchorIntent;
use mu_connect::anchor_rules::{decide, AnchorRules, Denied, Ledger};
use mu_connect::solana::{base58_public, execute, reconcile, AnchorErr, Balances, RpcNode, Settled, SolanaCfg};
use mu_log::{Kind, Log};
use mu_vault::Vault;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// The longest request line.
const LINE_LIMIT: usize = 16 * 1024;

fn sha(parts: &[&[u8]]) -> Hash32 {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p);
        h.update([0u8]);
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&h.finalize());
    Hash32(out)
}

/// What a request for an anchor is answered by, and the state the answers rest on.
pub struct AnchorService<'a> {
    rules: AnchorRules,
    ledger: Ledger,
    log: Log,
    vault: &'a dyn Vault,
    a: Box<dyn RpcNode + 'a>,
    b: Box<dyn RpcNode + 'a>,
    balances: Box<dyn Balances + 'a>,
    notify: Box<dyn FnMut(&str) + 'a>,
    cfg: SolanaCfg,
}

impl<'a> AnchorService<'a> {
    /// A service under the owner's `rules`, paying through the vault, on two nodes.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rules: AnchorRules,
        log: Log,
        vault: &'a dyn Vault,
        a: Box<dyn RpcNode + 'a>,
        b: Box<dyn RpcNode + 'a>,
        balances: Box<dyn Balances + 'a>,
        notify: Box<dyn FnMut(&str) + 'a>,
        cfg: SolanaCfg,
    ) -> Self {
        Self {
            rules,
            ledger: Ledger::default(),
            log,
            vault,
            a,
            b,
            balances,
            notify,
            cfg,
        }
    }

    /// The log, for the tests (the owner reads the file).
    #[cfg(test)]
    pub fn log(&self) -> &Log {
        &self.log
    }

    fn note(&mut self, kind: Kind, now: u64) {
        // A decision that cannot be written down is not made: the answer becomes an error.
        let _ = self.log.append(kind, now, self.vault);
    }

    /// One request line → one answer line.
    pub fn handle_line(&mut self, line: &str, now: u64) -> String {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            return json!({"error": "the request is not JSON"}).to_string();
        };
        if let Some(req) = v.get("anchor") {
            return self.anchor(req, now).to_string();
        }
        if let Some(req) = v.get("status") {
            let Some(sig) = req.get("signature").and_then(Value::as_str) else {
                return json!({"error": "status needs a signature"}).to_string();
            };
            return match reconcile(&*self.a, &*self.b, sig) {
                Settled::Landed { slot } => json!({"landed": {"slot": slot}}),
                Settled::Pending => json!({"pending": {}}),
                Settled::Failed => json!({"failed": {}}),
            }
            .to_string();
        }
        if v.get("address").is_some() {
            return match self.vault.tx_signer_ed25519() {
                Ok(s) => json!({"address": base58_public(&s.public_key())}),
                Err(_) => json!({"error": "the vault has no wallet"}),
            }
            .to_string();
        }
        json!({"error": "not a request of this service"}).to_string()
    }

    fn anchor(&mut self, req: &Value, now: u64) -> Value {
        let text = |k: &str| req.get(k).and_then(Value::as_str).map(str::to_owned);
        let (Some(memo), Some(cluster), Some(capsule)) = (text("memo"), text("cluster"), text("capsule")) else {
            return json!({"error": "an anchor needs memo, cluster and capsule"});
        };
        let intent_hash = sha(&[b"ksg-anchor-intent", memo.as_bytes(), cluster.as_bytes(), capsule.as_bytes()]);
        let day = now / 86_400;
        // The most this anchor may cost: what the reader allows itself, never above the owner's cap.
        let fee_cap = req
            .get("fee_cap_lamports")
            .and_then(Value::as_u64)
            .map_or(self.rules.per_anchor_cap_lamports, |c| c.min(self.rules.per_anchor_cap_lamports));

        // The owner's rules first, on the worst case: the fee cap. Nothing is built for a capsule
        // that may not pay, nothing is signed for a day whose budget is spent.
        if let Err(d) = decide(&self.rules, &self.ledger, &capsule, fee_cap, day) {
            return self.denied(intent_hash, &d, now);
        }
        let Ok(signer) = self.vault.tx_signer_ed25519() else {
            self.note(Kind::AnchorDenied { intent_hash, reason: 4 }, now);
            return json!({"denied": {"reason": "rejected", "detail": "the vault has no wallet"}});
        };
        // The wallet that pays must hold what the anchor may cost; a node that does not answer is not a
        // reason to refuse (the dry run of the anchor decides): Unknown is not Failed.
        if let Ok(held) = self.balances.balance(&base58_public(&signer.public_key())) {
            if held < fee_cap {
                (self.notify)(&format!(
                    "the wallet holds {held} lamports; an anchor of capsule {capsule} may cost {fee_cap}: top the main wallet up"
                ));
                self.note(Kind::Alert { code: 0x0A01 }, now);
                self.note(Kind::AnchorDenied { intent_hash, reason: 1 }, now);
                return json!({"denied": {"reason": "budget", "detail": "the wallet cannot pay what the anchor may cost"}});
            }
        }
        let intent = AnchorIntent { memo, fee_cap_lamports: fee_cap, cluster, capsule: capsule.clone() };
        match execute(&*self.a, &*self.b, &signer, &intent, &self.cfg) {
            Ok(sent) => {
                self.ledger.record(&capsule, day, sent.fee_lamports);
                self.note(
                    Kind::AnchorSent { intent_hash, signature_hash: sha(&[sent.signature.as_bytes()]) },
                    now,
                );
                json!({"sent": {"signature": sent.signature}})
            }
            Err(AnchorErr::Rejected(why)) => {
                // the connector's own check says the fee passed the cap: that is the owner's cap, `cap`
                let (reason, code) = if why.contains("passes the cap") { ("cap", 2) } else { ("rejected", 4) };
                self.note(Kind::AnchorDenied { intent_hash, reason: code }, now);
                json!({"denied": {"reason": reason, "detail": why}})
            }
            Err(AnchorErr::Unknown { reason, signature }) => {
                self.note(Kind::AnchorUnknown { intent_hash }, now);
                json!({"unknown": {"signature": signature, "reason": reason}})
            }
            Err(_) => {
                self.note(Kind::AnchorUnknown { intent_hash }, now);
                json!({"unknown": {"signature": null}})
            }
        }
    }

    fn denied(&mut self, intent_hash: Hash32, d: &Denied, now: u64) -> Value {
        let (reason, code) = match d {
            Denied::NotAllowed => ("not_allowed", 3),
            Denied::Cap { .. } => ("cap", 2),
            Denied::BudgetCapsule { .. } | Denied::BudgetTotal { .. } => ("budget", 1),
        };
        self.note(Kind::AnchorDenied { intent_hash, reason: code }, now);
        json!({"denied": {"reason": reason, "detail": d.to_string()}})
    }
}

/// Binds the socket at `path`, mode `0600`, in a directory only its user opens.
///
/// # Errors
///
/// The directory is open to the group or to others; something that is not a socket lies at `path`;
/// the bind or the chmod fails.
pub fn bind_unix(path: &Path) -> Result<UnixListener, String> {
    let dir = path.parent().filter(|d| !d.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    let mode = std::fs::metadata(dir).map_err(|e| format!("{}: {e}", dir.display()))?.permissions().mode();
    if mode & 0o077 != 0 {
        return Err(format!(
            "{} is open to the group or to others (mode {:o}): the daemon's socket goes in a directory only its user opens",
            dir.display(),
            mode & 0o777
        ));
    }
    match std::fs::symlink_metadata(path) {
        Ok(m) if std::os::unix::fs::FileTypeExt::is_socket(&m.file_type()) => {
            std::fs::remove_file(path).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        Ok(_) => return Err(format!("{} exists and is not a socket", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{}: {e}", path.display())),
    }
    let listener = UnixListener::bind(path).map_err(|e| format!("{}: {e}", path.display()))?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(listener)
}

/// Serves lines until `limit` connections have been served (`0` — forever); `now` names the moment.
pub fn serve_unix(
    listener: &UnixListener,
    service: &mut AnchorService<'_>,
    now: &dyn Fn() -> u64,
    limit: usize,
) {
    let mut served = 0usize;
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let _ = serve_connection(stream, service, now);
        served += 1;
        if limit != 0 && served >= limit {
            return;
        }
    }
}

fn serve_connection(stream: UnixStream, service: &mut AnchorService<'_>, now: &dyn Fn() -> u64) -> std::io::Result<()> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(std::time::Duration::from_secs(30)))?;
    let mut out = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        let n = reader.by_ref().take(LINE_LIMIT as u64 + 1).read_line(&mut line)?;
        if n == 0 {
            return Ok(());
        }
        let answer = if line.len() > LINE_LIMIT {
            json!({"error": "the request is too long"}).to_string()
        } else {
            service.handle_line(line.trim_end(), now())
        };
        writeln!(out, "{answer}")?;
        out.flush()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_connect::solana::{NodeErr, SigState, Sim};
    use std::collections::BTreeMap;
    use mu_vault::backend::SoftVault;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";
    const CAPSULE: &str = "1220cafe";
    const DAY0: u64 = 20_000 * 86_400;

    #[derive(Default)]
    struct World {
        sent: Vec<Vec<u8>>,
        fee: u64,
        send_fails: bool,
        landed: bool,
        /// Lamports by address (base58): the wallets the fake nodes know.
        wallets: BTreeMap<String, u64>,
        /// What the owner was told.
        notes: Vec<String>,
    }

    struct Node(Rc<RefCell<World>>);
    impl Balances for Node {
        fn balance(&self, address: &str) -> Result<u64, NodeErr> {
            Ok(self.0.borrow().wallets.get(address).copied().unwrap_or(0))
        }
    }
    impl RpcNode for Node {
        fn name(&self) -> &str {
            "n"
        }
        fn quote_fee(&self, _m: &[u8]) -> Result<u64, NodeErr> {
            Ok(self.0.borrow().fee)
        }
        fn latest_blockhash(&self) -> Result<[u8; 32], NodeErr> {
            Ok([5u8; 32])
        }
        fn simulate(&self, _t: &[u8]) -> Result<Sim, NodeErr> {
            Ok(Sim { ok: true, err: None })
        }
        fn send(&self, t: &[u8]) -> Result<String, NodeErr> {
            if self.0.borrow().send_fails {
                return Err(NodeErr::Unreachable("timed out".into()));
            }
            self.0.borrow_mut().sent.push(t.to_vec());
            Ok(String::new())
        }
        fn status(&self, _s: &str) -> Result<SigState, NodeErr> {
            Ok(if self.0.borrow().landed { SigState::Landed { slot: 9 } } else { SigState::NotFound })
        }
    }

    fn rules() -> AnchorRules {
        AnchorRules {
            per_anchor_cap_lamports: 20_000,
            daily_budget_per_capsule_lamports: 12_000,
            daily_budget_total_lamports: 50_000,
            allowed_capsules: vec![CAPSULE.into()],
        }
    }

    fn setup(name: &str) -> (Rc<RefCell<World>>, std::path::PathBuf) {
        let world = Rc::new(RefCell::new(World { fee: 5_000, ..World::default() }));
        // the one wallet of the test vault is funded
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        world.borrow_mut().wallets.insert(main_addr(&vault), 100_000_000);
        let dir = std::env::temp_dir().join(format!("ksg-mu-anchor-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a directory");
        (world, dir)
    }

    fn main_addr(vault: &SoftVault) -> String {
        base58_public(&vault.tx_signer_ed25519().expect("the main wallet").public_key())
    }

    fn service<'a>(world: &Rc<RefCell<World>>, vault: &'a SoftVault, dir: &Path, rules: AnchorRules) -> AnchorService<'a> {
        let log = Log::open(&dir.join("mu.log"), vault).expect("a log");
        let told = Rc::clone(world);
        AnchorService::new(
            rules,
            log,
            vault,
            Box::new(Node(Rc::clone(world))),
            Box::new(Node(Rc::clone(world))),
            Box::new(Node(Rc::clone(world))),
            Box::new(move |note: &str| told.borrow_mut().notes.push(note.to_owned())),
            SolanaCfg { cluster: "localnet".into() },
        )
    }

    fn ask(s: &mut AnchorService<'_>, capsule: &str, fee_cap: u64, at: u64) -> Value {
        let line = json!({"anchor": {"memo": SEAL, "cluster": "localnet", "capsule": capsule, "fee_cap_lamports": fee_cap}});
        serde_json::from_str(&s.handle_line(&line.to_string(), at)).expect("JSON")
    }

    fn kinds(s: &AnchorService<'_>) -> Vec<String> {
        s.log().entries().iter().map(|e| format!("{:?}", e.kind).split([' ', '{']).next().unwrap_or("").to_owned()).collect()
    }

    #[test]
    fn allowed_is_sent_and_the_log_holds_the_hash_of_the_signature_not_the_signature() {
        let (world, dir) = setup("sent");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        let a = ask(&mut s, CAPSULE, 10_000, DAY0);
        let sig = a["sent"]["signature"].as_str().expect("sent").to_owned();
        assert_eq!(world.borrow().sent.len(), 1, "one transaction went out");
        let last = s.log().entries().last().expect("an entry");
        let Kind::AnchorSent { signature_hash, .. } = &last.kind else { panic!("{:?}", last.kind) };
        assert_eq!(*signature_hash, sha(&[sig.as_bytes()]));
        assert!(!format!("{last:?}").contains(&sig), "the signature itself is not in the log");
        s.log().verify_chain().expect("the chain holds");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_stranger_capsule_is_not_allowed_and_nothing_is_sent() {
        let (world, dir) = setup("stranger");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        let a = ask(&mut s, "1220dead", 10_000, DAY0);
        assert_eq!(a["denied"]["reason"], "not_allowed");
        assert!(world.borrow().sent.is_empty());
        assert!(matches!(s.log().entries().last().map(|e| &e.kind), Some(Kind::AnchorDenied { reason: 3, .. })));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_budget_of_the_day_ends_in_a_denial_and_the_next_day_it_is_new() {
        let (world, dir) = setup("budget");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        // budget per capsule 12 000, fee 5 000: the first is paid; the second asks for a cap of
        // 10 000 against 5 000 spent — 15 000 > 12 000 — and is refused before anything is built
        assert!(ask(&mut s, CAPSULE, 10_000, DAY0)["sent"].is_object());
        let second = ask(&mut s, CAPSULE, 10_000, DAY0 + 60);
        assert_eq!(second["denied"]["reason"], "budget");
        assert_eq!(world.borrow().sent.len(), 1, "the refused one never reached a node");
        // a cap that fits the rest is paid; and the day after, the budget is new
        assert!(ask(&mut s, CAPSULE, 6_000, DAY0 + 120)["sent"].is_object());
        assert!(ask(&mut s, CAPSULE, 10_000, DAY0 + 86_400)["sent"].is_object());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_fee_above_the_owners_cap_is_a_denial_cap() {
        let (world, dir) = setup("cap");
        world.borrow_mut().fee = 30_000; // the nodes quote more than the owner lets one anchor cost
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, AnchorRules { daily_budget_per_capsule_lamports: 100_000, ..rules() });
        // the reader asks for more than the owner's cap: the owner's cap wins
        let a = ask(&mut s, CAPSULE, 90_000, DAY0);
        assert_eq!(a["denied"]["reason"], "cap", "{a}");
        assert!(world.borrow().sent.is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_send_that_times_out_is_unknown_with_the_signature_never_denied() {
        let (world, dir) = setup("unknown");
        world.borrow_mut().send_fails = true;
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        let a = ask(&mut s, CAPSULE, 10_000, DAY0);
        assert!(a["unknown"]["signature"].is_string(), "{a}");
        assert!(a.get("denied").is_none());
        assert!(matches!(s.log().entries().last().map(|e| &e.kind), Some(Kind::AnchorUnknown { .. })));
        // asked by the signature: not landed yet → pending; landed → landed
        let sig = a["unknown"]["signature"].as_str().expect("signature").to_owned();
        let q = json!({"status": {"signature": sig}}).to_string();
        assert_eq!(serde_json::from_str::<Value>(&s.handle_line(&q, DAY0 + 1)).expect("JSON")["pending"], json!({}));
        world.borrow_mut().landed = true;
        assert_eq!(serde_json::from_str::<Value>(&s.handle_line(&q, DAY0 + 2)).expect("JSON")["landed"]["slot"], 9);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn every_decision_is_a_line_of_the_log_in_order() {
        let (world, dir) = setup("log");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        ask(&mut s, CAPSULE, 10_000, DAY0); // sent
        ask(&mut s, "1220dead", 10_000, DAY0 + 1); // not allowed
        ask(&mut s, CAPSULE, 10_000, DAY0 + 2); // budget
        let k = kinds(&s);
        assert_eq!(k, ["AnchorSent", "AnchorDenied", "AnchorDenied"]);
        s.log().verify_chain().expect("the chain holds");
        // and it reads back from the file: the log is the owner's audit
        drop(s);
        let again = Log::open(&dir.join("mu.log"), &vault).expect("reopens");
        assert_eq!(again.entries().len(), 3);
        again.verify_chain().expect("the chain holds after reopening");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_address_is_the_one_wallet_that_pays_for_every_capsule() {
        let (world, dir) = setup("address");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        let a: Value = serde_json::from_str(&s.handle_line(&json!({"address": {}}).to_string(), DAY0)).expect("JSON");
        assert_eq!(a["address"], main_addr(&vault), "the main wallet, whichever capsule asks");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn one_wallet_pays_for_two_capsules() {
        let (world, dir) = setup("two-capsules");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(
            &world,
            &vault,
            &dir,
            AnchorRules { allowed_capsules: vec![CAPSULE.into(), "1220beef".into()], ..rules() },
        );
        assert!(ask(&mut s, CAPSULE, 10_000, DAY0)["sent"].is_object());
        assert!(ask(&mut s, "1220beef", 10_000, DAY0)["sent"].is_object());
        assert_eq!(world.borrow().sent.len(), 2, "two anchors, no refill between them");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_wallet_that_cannot_pay_is_denied_budget_and_the_owner_is_told() {
        let (world, dir) = setup("main-empty");
        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        world.borrow_mut().wallets.insert(main_addr(&vault), 1_000); // less than the anchor may cost
        let mut s = service(&world, &vault, &dir, rules());
        let a = ask(&mut s, CAPSULE, 10_000, DAY0);
        assert_eq!(a["denied"]["reason"], "budget", "{a}");
        assert!(world.borrow().sent.is_empty(), "nothing was sent");
        let told = world.borrow().notes.clone();
        assert_eq!(told.len(), 1, "{told:?}");
        assert!(told[0].contains("top the main wallet up"), "{told:?}");
        assert_eq!(kinds(&s), ["Alert", "AnchorDenied"], "the log says both");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_socket_is_0600_in_a_private_directory_and_answers_a_line() {
        let (world, dir) = setup("socket");
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).expect("chmod");
        let open = dir.join("open");
        std::fs::create_dir(&open).expect("dir");
        std::fs::set_permissions(&open, std::fs::Permissions::from_mode(0o755)).expect("chmod");
        assert!(bind_unix(&open.join("s.sock")).expect_err("open directory").contains("open to the group or to others"));

        let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
        let mut s = service(&world, &vault, &dir, rules());
        let sock = dir.join("mu-anchor.sock");
        let listener = bind_unix(&sock).expect("binds");
        assert_eq!(std::fs::metadata(&sock).expect("the socket").permissions().mode() & 0o777, 0o600);
        let client = {
            let sock = sock.clone();
            std::thread::spawn(move || {
                let mut c = UnixStream::connect(&sock).expect("connects");
                writeln!(c, "{}", json!({"anchor": {"memo": SEAL, "cluster": "localnet", "capsule": "1220dead"}})).expect("writes");
                let mut answer = String::new();
                BufReader::new(c).read_line(&mut answer).expect("an answer");
                answer
            })
        };
        serve_unix(&listener, &mut s, &|| DAY0, 1);
        let answer: Value = serde_json::from_str(&client.join().expect("the client")).expect("JSON");
        assert_eq!(answer["denied"]["reason"], "not_allowed");
        std::fs::remove_dir_all(&dir).ok();
    }
}
