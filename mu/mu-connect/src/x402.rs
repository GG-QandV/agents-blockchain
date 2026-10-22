//! M7x x402 connector — gasless USDC via EIP-3009 `TransferWithAuthorization`.
//!
//! x402 (version 1, scheme `exact`): the server answers 402 with `accepts[]`;
//! the agent signs an EIP-712 `TransferWithAuthorization` and retries with the
//! `X-PAYMENT` header; the server's facilitator submits it on chain.
//!
//! RISK-M7-1: Unknown ≠ Failed. `execute()` returns `TxRef::Authorization`;
//!   `status()` never fails a payment while its authorization can still settle.
//! RISK-M7-5: nothing the server says can move money elsewhere. The token
//!   contract is a per-network constant baked into the binary; the challenge
//!   must name exactly the intent's recipient, amount, network and that token,
//!   or nothing is signed.
//! RISK-M4-1: the payer address is derived from the signing key itself — a
//!   wallet configured wrong refuses before any signature leaves.

use crate::{ConnErr, Connector, Fee, Intent, RejectReason, TxRef, TxStatus};
use mu_common::{Amount, Hash32};
use mu_vault::TxSigner;
use sha3::{Digest, Keccak256};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// The only x402 version and scheme this connector speaks.
pub const X402_VERSION: u64 = 1;
pub const SCHEME_EXACT: &str = "exact";

const EIP712_DOMAIN_TYPE: &[u8] =
    b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
const TRANSFER_WITH_AUTH_TYPE: &[u8] = b"TransferWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)";

/// One network x402 pays on: its USDC contract and EIP-712 domain are
/// constants (RISK-M7-5), never taken from a 402 answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct X402Network {
    pub chain_id: u64,
    /// x402 network name.
    pub name: &'static str,
    pub usdc: [u8; 20],
    /// USDC's EIP-712 domain name on this network.
    pub domain_name: &'static str,
    pub domain_version: &'static str,
}

/// Base mainnet — real money.
pub const BASE: X402Network = X402Network {
    chain_id: 8453,
    name: "base",
    usdc: hex20_const(b"833589fcd6edb6e08f4c7c32d4f71b54bda02913"),
    domain_name: "USD Coin",
    domain_version: "2",
};

/// Base Sepolia — test network, no real money (keysingate stand).
pub const BASE_SEPOLIA: X402Network = X402Network {
    chain_id: 84532,
    name: "base-sepolia",
    usdc: hex20_const(b"036cbd53842c5426634e7929541ec2318f3dcf7e"),
    domain_name: "USDC",
    domain_version: "2",
};

/// USDC on Base mainnet (kept for callers of 0.2.1).
pub const USDC_BASE: [u8; 20] = BASE.usdc;
pub const CHAIN_ID_BASE: u64 = BASE.chain_id;

/// The network of a chain id; `None` for one x402 is not configured for.
pub fn network_for_chain(chain_id: u64) -> Option<X402Network> {
    [BASE, BASE_SEPOLIA].into_iter().find(|n| n.chain_id == chain_id)
}

/// One entry of `accepts[]` from a 402 answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct X402Challenge {
    pub scheme: String,
    pub network: String,
    pub max_amount_required: String,
    pub pay_to: String,
    pub asset: String,
    pub resource: String,
    pub max_timeout_seconds: u64,
    pub domain_name: String,
    pub domain_version: String,
}

impl X402Challenge {
    /// The `exact` entry for `network` from a 402 body.
    pub fn from_402_body(body: &str, network: &X402Network) -> Result<Self, ConnErr> {
        let bad = |m: &str| ConnErr::Config(format!("x402 402 body: {m}"));
        let v: serde_json::Value = serde_json::from_str(body).map_err(|_| bad("not JSON"))?;
        if v.get("x402Version").and_then(serde_json::Value::as_u64) != Some(X402_VERSION) {
            return Err(bad("x402Version is not 1"));
        }
        let accepts = v.get("accepts").and_then(|a| a.as_array()).ok_or_else(|| bad("no accepts[]"))?;
        let a = accepts
            .iter()
            .find(|a| {
                a.get("scheme").and_then(|s| s.as_str()) == Some(SCHEME_EXACT)
                    && a.get("network").and_then(|s| s.as_str()) == Some(network.name)
            })
            .ok_or_else(|| bad("no exact entry for this network"))?;
        let s = |k: &str| -> Result<String, ConnErr> {
            a.get(k).and_then(|x| x.as_str()).map(str::to_owned).ok_or_else(|| bad(k))
        };
        let extra = |k: &str, d: &str| -> String {
            a.get("extra")
                .and_then(|e| e.get(k))
                .and_then(|x| x.as_str())
                .unwrap_or(d)
                .to_owned()
        };
        Ok(Self {
            scheme: s("scheme")?,
            network: s("network")?,
            max_amount_required: s("maxAmountRequired")?,
            pay_to: s("payTo")?,
            asset: s("asset")?,
            resource: a.get("resource").and_then(|x| x.as_str()).unwrap_or("").to_owned(),
            max_timeout_seconds: a
                .get("maxTimeoutSeconds")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| bad("maxTimeoutSeconds"))?,
            domain_name: extra("name", network.domain_name),
            domain_version: extra("version", network.domain_version),
        })
    }

    /// RISK-M7-5: the challenge must ask for exactly what the intent pays.
    fn matches(&self, i: &Intent, net: &X402Network) -> Result<(), ConnErr> {
        let no = || ConnErr::Rejected(RejectReason::ChallengeMismatch);
        if self.scheme != SCHEME_EXACT || self.network != net.name {
            return Err(no());
        }
        if parse_addr(&self.asset).ok_or_else(no)? != net.usdc {
            return Err(no());
        }
        if parse_addr(&self.pay_to).ok_or_else(no)? != *i.recipient.bytes() {
            return Err(no());
        }
        let amount: u128 = self.max_amount_required.parse().map_err(|_| no())?;
        if amount != i.amount.minor() {
            return Err(no());
        }
        // The token's EIP-712 domain is ours, not the server's.
        if self.domain_name != net.domain_name || self.domain_version != net.domain_version {
            return Err(no());
        }
        if self.max_timeout_seconds == 0 || self.max_timeout_seconds > MAX_VALIDITY_SECONDS {
            return Err(no());
        }
        Ok(())
    }
}

/// The longest an authorization may stay valid.
pub const MAX_VALIDITY_SECONDS: u64 = 3600;

/// Identity of one 402 offer: what the seller asked for, in canonical bytes.
fn offer_digest(c: &X402Challenge) -> [u8; 32] {
    let mut s = String::new();
    for part in [
        &c.scheme,
        &c.network,
        &c.max_amount_required,
        &c.pay_to,
        &c.asset,
        &c.resource,
        &c.domain_name,
        &c.domain_version,
    ] {
        s.push_str(part);
        s.push('\u{1f}');
    }
    s.push_str(&c.max_timeout_seconds.to_string());
    keccak(&[s.as_bytes()])
}
const DEFAULT_VALIDITY_SECONDS: u64 = 300;
/// Clock skew allowance before now.
const VALID_AFTER_SKEW: u64 = 60;

/// A signed EIP-3009 authorization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedAuthorization {
    pub network: &'static str,
    pub from: [u8; 20],
    pub to: [u8; 20],
    pub value: u128,
    pub valid_after: u64,
    pub valid_before: u64,
    pub nonce: [u8; 32],
    /// 27 or 28.
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl SignedAuthorization {
    /// `X-PAYMENT`: base64 of the x402 version 1 payload.
    pub fn to_header(&self) -> String {
        use base64::Engine;
        let mut sig = String::with_capacity(132);
        sig.push_str("0x");
        sig.push_str(&hex(&self.r));
        sig.push_str(&hex(&self.s));
        sig.push_str(&format!("{:02x}", self.v));
        let json = serde_json::json!({
            "x402Version": X402_VERSION,
            "scheme": SCHEME_EXACT,
            "network": self.network,
            "payload": {
                "signature": sig,
                "authorization": {
                    "from": format!("0x{}", hex(&self.from)),
                    "to": format!("0x{}", hex(&self.to)),
                    "value": self.value.to_string(),
                    "validAfter": self.valid_after.to_string(),
                    "validBefore": self.valid_before.to_string(),
                    "nonce": format!("0x{}", hex(&self.nonce)),
                }
            }
        });
        base64::engine::general_purpose::STANDARD.encode(json.to_string().as_bytes())
    }
}

/// x402 connector: signs EIP-3009 `TransferWithAuthorization` for one wallet
/// on one network.
pub struct X402Connector {
    pub chain_id: u64,
    pub network: X402Network,
    /// The wallet this connector pays from; the signing key must be its key.
    pub wallet_addr: [u8; 20],
    /// Digests of the offers already paid. One 402 answer pays once; a second
    /// payment needs a new offer. Bounded ring, so a long-running process does
    /// not grow.
    used_offers: Mutex<Vec<[u8; 32]>>,
}

/// `AuthorizationUsed(address,bytes32)`: emitted by the USDC contract when an
/// EIP-3009 authorization is spent. Topic verified on the live Base Sepolia tx.
pub const AUTHORIZATION_USED_TOPIC: &str =
    "0x98de503528ee59b575ef0c0a2576a82497bfc029a5685b209e9ec333479b10a5";

/// What the chain says about one authorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Usage {
    Unused,
    Used { block: u64 },
    Unconfirmed,
}

/// The two nodes the chain is read from. One node is never enough to close a
/// reserve, so a missing one is `Unknown`.
fn rpc_urls() -> Result<Vec<String>, ConnErr> {
    let urls: Vec<String> = ["MU_X402_RPC_1", "MU_X402_RPC_2"]
        .iter()
        .filter_map(|k| std::env::var(k).ok())
        .filter(|v| !v.trim().is_empty())
        .collect();
    if urls.len() < 2 {
        return Err(ConnErr::Unknown(
            "the chain is read from two RPC nodes: set MU_X402_RPC_1 and MU_X402_RPC_2".into(),
        ));
    }
    Ok(urls)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn parse_u64(s: Option<&str>) -> Result<u64, ConnErr> {
    let s = s.ok_or_else(|| ConnErr::Unknown("rpc: expected a quantity".into()))?;
    u64::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(|_| ConnErr::Unknown(format!("rpc: bad quantity {s}")))
}

fn parse_h32(s: Option<&str>) -> Result<[u8; 32], ConnErr> {
    let s = s.ok_or_else(|| ConnErr::Unknown("rpc: expected a hash".into()))?;
    let h = s.trim_start_matches("0x");
    if h.len() != 64 {
        return Err(ConnErr::Unknown(format!("rpc: bad hash {s}")));
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&h[2 * i..2 * i + 2], 16)
            .map_err(|_| ConnErr::Unknown(format!("rpc: bad hash {s}")))?;
    }
    Ok(out)
}

impl X402Connector {
    /// Base mainnet (0.2.1 behaviour).
    pub fn new(wallet_addr: [u8; 20]) -> Self {
        Self::on(BASE, wallet_addr)
    }

    /// On `network`.
    pub fn on(network: X402Network, wallet_addr: [u8; 20]) -> Self {
        X402Connector {
            chain_id: network.chain_id,
            network,
            wallet_addr,
            used_offers: Mutex::new(Vec::new()),
        }
    }

    /// How the chain sees this authorization: unused, used and confirmed, or
    /// used but still short of confirmations. Two nodes must agree on the
    /// transaction; confirmations are the smaller count of the two.
    fn usage(&self, nonce: [u8; 32]) -> Result<Usage, ConnErr> {
        let urls = rpc_urls()?;
        let window = env_u64("MU_X402_LOG_WINDOW", 5_000);
        let a = self.logs_once(&urls[0], nonce, window)?;
        let b = self.logs_once(&urls[1], nonce, window)?;
        if a != b {
            return Err(ConnErr::Unknown(
                "two RPC nodes disagree on AuthorizationUsed".into(),
            ));
        }
        let Some((hash, block)) = a else { return Ok(Usage::Unused) };
        let need = env_u64("MU_X402_CONFIRMATIONS", 12);
        let mut conf = u64::MAX;
        for url in urls.iter().take(2) {
            if !self.receipt_ok(url, hash)? {
                return Err(ConnErr::Unknown("the receipt is not successful".into()));
            }
            conf = conf.min(self.block_number(url)?.saturating_sub(block) + 1);
        }
        if conf < need {
            return Ok(Usage::Unconfirmed);
        }
        Ok(Usage::Used { block })
    }

    /// The `AuthorizationUsed` log for this nonce, if any, on one node.
    fn logs_once(&self, url: &str, nonce: [u8; 32], window: u64) -> Result<Option<([u8; 32], u64)>, ConnErr> {
        let head = self.block_number(url)?;
        let from = head.saturating_sub(window);
        let logs = self.rpc(url, "eth_getLogs", serde_json::json!([{
            "address": format!("0x{}", hex(&self.network.usdc)),
            "fromBlock": format!("0x{from:x}"),
            "toBlock": format!("0x{head:x}"),
            "topics": [
                AUTHORIZATION_USED_TOPIC,
                format!("0x{}{}", "0".repeat(24), hex(&self.wallet_addr)),
                format!("0x{}", hex(&nonce)),
            ],
        }]))?;
        let arr = logs.as_array().ok_or_else(|| ConnErr::Unknown("logs: not an array".into()))?;
        if arr.is_empty() {
            return Ok(None);
        }
        if arr.len() > 1 {
            return Err(ConnErr::Unknown("several AuthorizationUsed logs for one nonce".into()));
        }
        let l = &arr[0];
        let hash = parse_h32(l.get("transactionHash").and_then(|x| x.as_str()))?;
        let block = parse_u64(l.get("blockNumber").and_then(|x| x.as_str()))?;
        Ok(Some((hash, block)))
    }

    fn receipt_ok(&self, url: &str, hash: [u8; 32]) -> Result<bool, ConnErr> {
        let r = self.rpc(url, "eth_getTransactionReceipt", serde_json::json!([format!("0x{}", hex(&hash))]))?;
        let status = r
            .get("status")
            .and_then(|s| s.as_str())
            .ok_or_else(|| ConnErr::Unknown("receipt without status".into()))?;
        Ok(status.eq_ignore_ascii_case("0x1"))
    }

    fn block_number(&self, url: &str) -> Result<u64, ConnErr> {
        let v = self.rpc(url, "eth_blockNumber", serde_json::json!([]))?;
        parse_u64(v.as_str())
    }

    fn rpc(&self, url: &str, method: &str, params: serde_json::Value) -> Result<serde_json::Value, ConnErr> {
        let body = serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).to_string();
        let resp = ureq::post(url)
            .timeout(std::time::Duration::from_secs(10))
            .set("Content-Type", "application/json")
            .send_string(&body)
            .map_err(|e| ConnErr::Unknown(format!("rpc {url}: {e}")))?;
        let text = resp.into_string().map_err(|e| ConnErr::Unknown(format!("rpc {url}: {e}")))?;
        let v: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| ConnErr::Unknown(format!("rpc {url}: {e}")))?;
        if let Some(err) = v.get("error") {
            return Err(ConnErr::Unknown(format!("rpc {url}: {err}")));
        }
        v.get("result").cloned().ok_or_else(|| ConnErr::Unknown(format!("rpc {url}: no result")))
    }

    /// The EIP-712 digest of an authorization on this connector's network.
    pub fn digest(
        &self,
        from: &[u8; 20],
        to: &[u8; 20],
        value: u128,
        valid_after: u64,
        valid_before: u64,
        nonce: &[u8; 32],
    ) -> Hash32 {
        let mut chain = [0u8; 32];
        chain[24..].copy_from_slice(&self.network.chain_id.to_be_bytes());
        let domain = keccak(&[
            &keccak(&[EIP712_DOMAIN_TYPE]),
            &keccak(&[self.network.domain_name.as_bytes()]),
            &keccak(&[self.network.domain_version.as_bytes()]),
            &chain,
            &word20(&self.network.usdc),
        ]);
        let s = keccak(&[
            &keccak(&[TRANSFER_WITH_AUTH_TYPE]),
            &word20(from),
            &word20(to),
            &word_u128(value),
            &word_u128(u128::from(valid_after)),
            &word_u128(u128::from(valid_before)),
            nonce,
        ]);
        Hash32(keccak(&[&[0x19, 0x01], &domain, &s]))
    }

    /// Signs one authorization with every field given — `execute` supplies
    /// the time window and a random nonce; tests supply fixed ones.
    pub fn authorize(
        &self,
        signer: &TxSigner,
        to: [u8; 20],
        value: u128,
        valid_after: u64,
        valid_before: u64,
        nonce: [u8; 32],
    ) -> Result<SignedAuthorization, ConnErr> {
        let from = address_of(signer)?;
        if from != self.wallet_addr {
            return Err(ConnErr::Rejected(RejectReason::WalletMismatch));
        }
        let digest = self.digest(&from, &to, value, valid_after, valid_before, &nonce);
        let sig = signer
            .sign(&digest)
            .map_err(|e| ConnErr::Unknown(format!("sign: {e:?}")))?;
        let auth = SignedAuthorization {
            network: self.network.name,
            from,
            to,
            value,
            valid_after,
            valid_before,
            nonce,
            v: 27 + sig.v,
            r: sig.r,
            s: sig.s,
        };
        // Self-check: the signature recovers to the wallet over this digest.
        if recover(&digest, &auth) != Some(from) {
            return Err(ConnErr::Unknown("x402 self-check: signature does not recover".into()));
        }
        Ok(auth)
    }
}

impl Connector for X402Connector {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr> {
        if i.chain_id != self.chain_id {
            return Err(ConnErr::Rejected(RejectReason::ChainMismatch));
        }
        Ok(Fee { gas_estimate: Amount::ZERO })
    }

    fn pre_auth(&self) -> bool {
        true
    }

    /// The window is exactly what the 402 answer allows: no default, no
    /// minimum. Without an offer, or with one over the protocol ceiling, there
    /// is no window and the pipeline does not pay.
    fn auth_window_secs(&self, offer: Option<&X402Challenge>) -> Option<u64> {
        let c = offer?;
        if c.max_timeout_seconds == 0 || c.max_timeout_seconds > MAX_VALIDITY_SECONDS {
            return None;
        }
        Some(c.max_timeout_seconds)
    }

    /// x402 never invents a nonce: the pipeline must pass one that is already
    /// in the WAL, so a crash cannot produce a second authorization.
    fn execute(&self, _i: &Intent, _signer: TxSigner) -> Result<TxRef, ConnErr> {
        Err(ConnErr::Config(
            "x402 needs pre-authorized parameters: use execute_auth".into(),
        ))
    }

    fn execute_auth(
        &self,
        i: &Intent,
        signer: TxSigner,
        ctx: crate::ExecCtx,
    ) -> Result<crate::Executed, ConnErr> {
        if i.chain_id != self.chain_id || i.recipient.chain_id() != self.chain_id {
            return Err(ConnErr::Rejected(RejectReason::ChainMismatch));
        }
        let offer = ctx
            .offer
            .ok_or(ConnErr::Rejected(RejectReason::NoChallenge))?;
        let params = ctx.auth.ok_or_else(|| {
            ConnErr::Config("x402 requires a nonce written to the WAL".into())
        })?;
        // Network, asset, domain, recipient, amount and the timeout ceiling.
        offer.matches(i, &self.network)?;
        // One 402 answer pays once: the same offer cannot fund a second payment.
        let digest = offer_digest(&offer);
        {
            let mut used = self.used_offers.lock().unwrap();
            if used.contains(&digest) {
                return Err(ConnErr::Rejected(RejectReason::OfferAlreadyUsed));
            }
            if used.len() >= 256 {
                used.remove(0);
            }
            used.push(digest);
        }
        // The authorization may not outlive what the issuer accepts, and never
        // more than the protocol ceiling.
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ConnErr::Unknown(format!("clock: {e:?}")))?
            .as_secs();
        let window = params.valid_before.saturating_sub(now);
        if window > offer.max_timeout_seconds || window > MAX_VALIDITY_SECONDS {
            return Err(ConnErr::Rejected(RejectReason::ChallengeMismatch));
        }
        let auth = self.authorize(
            &signer,
            *i.recipient.bytes(),
            i.amount.minor(),
            params.valid_after,
            params.valid_before,
            params.nonce,
        )?;
        // RISK-M7-5: what was signed is what the intent says.
        if auth.to != *i.recipient.bytes() || auth.value != i.amount.minor() {
            return Err(ConnErr::Config("x402 self-check: auth mismatch".into()));
        }
        // The header is produced here and returned; the daemon writes it to the
        // journal before the agent may see it.
        let r = TxRef::Authorization { nonce: auth.nonce, valid_before: auth.valid_before };
        Ok(crate::Executed { tx: r, header: Some(auth.to_header()) })
    }

    /// Reads `authorizationState(authorizer, nonce)` from the chain. Two
    /// independent nodes must agree (RISK-M7-3); a disagreement or an
    /// unavailable node is `Unknown`, never a guess.
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        let (nonce, valid_before) = match r {
            TxRef::Authorization { nonce, valid_before } => (*nonce, *valid_before),
            TxRef::Real { .. } => return Err(ConnErr::Config("x402 got Real txref".into())),
            TxRef::Simulated { .. } => {
                return Ok(TxStatus::Failed { reason: crate::FailReason::Simulated })
            }
        };
        match self.usage(nonce)? {
            // The facilitator pays the gas, so the owner's effective gas is 0 by
            // design: this field is not a placeholder.
            Usage::Used { block } => return Ok(TxStatus::Settled { block, effective_gas: 0 }),
            Usage::Unconfirmed => return Ok(TxStatus::Pending),
            Usage::Unused => {}
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ConnErr::Unknown(format!("clock: {e:?}")))?
            .as_secs();
        if now >= valid_before {
            Ok(TxStatus::Failed { reason: crate::FailReason::NonceConsumedByOther })
        } else {
            Ok(TxStatus::Pending)
        }
    }
}

/// The EVM address of a signing key.
pub fn address_of(signer: &TxSigner) -> Result<[u8; 20], ConnErr> {
    let compressed = signer
        .verifying_key_bytes()
        .map_err(|e| ConnErr::Unknown(format!("key: {e:?}")))?;
    let vk = k256::ecdsa::VerifyingKey::from_sec1_bytes(&compressed)
        .map_err(|_| ConnErr::Unknown("key: not a secp256k1 point".into()))?;
    Ok(address_of_key(&vk))
}

fn address_of_key(vk: &k256::ecdsa::VerifyingKey) -> [u8; 20] {
    let point = vk.to_encoded_point(false);
    let h = keccak(&[&point.as_bytes()[1..]]);
    let mut a = [0u8; 20];
    a.copy_from_slice(&h[12..]);
    a
}

fn recover(digest: &Hash32, a: &SignedAuthorization) -> Option<[u8; 20]> {
    let mut rs = [0u8; 64];
    rs[..32].copy_from_slice(&a.r);
    rs[32..].copy_from_slice(&a.s);
    let sig = k256::ecdsa::Signature::from_slice(&rs).ok()?;
    if sig.normalize_s().is_some() {
        return None; // high-s: refused by sellers (EIP-2)
    }
    let id = k256::ecdsa::RecoveryId::from_byte(a.v.checked_sub(27)?)?;
    let vk = k256::ecdsa::VerifyingKey::recover_from_prehash(&digest.0, &sig, id).ok()?;
    Some(address_of_key(&vk))
}

fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Keccak256::new();
    for p in parts {
        h.update(p);
    }
    h.finalize().into()
}

fn word20(a: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(a);
    w
}

fn word_u128(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn parse_addr(s: &str) -> Option<[u8; 20]> {
    let h = s.strip_prefix("0x")?;
    if h.len() != 40 {
        return None;
    }
    let mut a = [0u8; 20];
    for (i, c) in h.as_bytes().chunks(2).enumerate() {
        a[i] = (nib(c[0])? << 4) | nib(c[1])?;
    }
    Some(a)
}

fn nib(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

const fn hex20_const(s: &[u8; 40]) -> [u8; 20] {
    const fn n(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => panic!("lowercase hex only"),
        }
    }
    let mut out = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        out[i] = (n(s[2 * i]) << 4) | n(s[2 * i + 1]);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TxRef;
    use mu_common::CanonAddress;
    use mu_vault::Vault;

    // Independent vector: eth-account 0.13.4 (Python), wallet key 0x11…11,
    // `sign_typed_data` on Base Sepolia USDC (same vector as keysingate
    // `ksg-pay-v2`, which accepts it).
    const ISSUER: &str = "0x209693bc6afc0c5328ba36faf03c514ef312287c";
    const PAYER: &str = "19e7e376e7c213b7e7e7e46cc70a5dd086daff2a";
    const DIGEST: &str = "e651273bbc3ba2b8cab7f0588dcb94aaf5fd04a1c7ac61512c0df2225dc52064";
    const SIG: &str = "2a895ae6947395e7203111cc46aae30fb17b34185d92e5dc3af39b1f0047a97a0c6e8b4874c9e32469ab305eae18a3e23895f84a878ed4d905652f33f43878421c";

    fn vault(wallet: [u8; 32]) -> mu_vault::backend::SoftVault {
        mu_vault::backend::SoftVault::for_test([1; 32], [2; 32], wallet)
    }

    fn payer() -> [u8; 20] {
        parse_addr(&format!("0x{PAYER}")).unwrap()
    }

    fn sepolia() -> X402Connector {
        X402Connector::on(BASE_SEPOLIA, payer())
    }

    fn intent(to: &str, amount: u128, chain: u64) -> Intent {
        Intent {
            recipient: CanonAddress::canon(to, chain).unwrap(),
            amount: Amount::from_minor(amount),
            chain_id: chain,
        }
    }

    fn params(n: u8) -> crate::AuthParams {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        crate::AuthParams { nonce: [n; 32], valid_after: now.saturating_sub(60), valid_before: now + 300 }
    }

    fn ctx(n: u8, ch: X402Challenge) -> crate::ExecCtx {
        crate::ExecCtx { auth: Some(params(n)), offer: Some(ch) }
    }

    fn challenge(pay_to: &str, amount: &str) -> X402Challenge {
        challenge_t(pay_to, amount, 300)
    }

    fn challenge_t(pay_to: &str, amount: &str, timeout: u64) -> X402Challenge {
        X402Challenge::from_402_body(
            &serde_json::json!({
                "x402Version": 1,
                "accepts": [{
                    "scheme": "exact", "network": "base-sepolia",
                    "maxAmountRequired": amount, "resource": "https://issuer.example/bare/issue",
                    "payTo": pay_to, "maxTimeoutSeconds": timeout,
                    "asset": "0x036cbd53842c5426634e7929541ec2318f3dcf7e",
                    "extra": {"name": "USDC", "version": "2"}
                }]
            })
            .to_string(),
            &BASE_SEPOLIA,
        )
        .unwrap()
    }

    #[test]
    fn type_strings_hash_to_the_known_constants() {
        assert_eq!(
            hex(&keccak(&[EIP712_DOMAIN_TYPE])),
            "8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f"
        );
        assert_eq!(
            hex(&keccak(&[TRANSFER_WITH_AUTH_TYPE])),
            "7c7c6cdb67a18743f49ec6fa9b35f50d52ed05cbed4cc592e13b44501c1a2267"
        );
    }

    #[test]
    fn digest_and_signature_match_an_independent_implementation() {
        let v = vault([0x11; 32]);
        let signer = v.tx_signer().unwrap();
        assert_eq!(hex(&address_of(&signer).unwrap()), PAYER);
        let c = sepolia();
        let to = parse_addr(ISSUER).unwrap();
        let nonce = [0xab; 32];
        assert_eq!(hex(&c.digest(&payer(), &to, 100_000, 1_760_000_000, 1_760_000_600, &nonce).0), DIGEST);
        let a = c.authorize(&signer, to, 100_000, 1_760_000_000, 1_760_000_600, nonce).unwrap();
        let sig = format!("{}{}{:02x}", hex(&a.r), hex(&a.s), a.v);
        assert_eq!(sig, SIG, "RFC 6979 is deterministic: same key, same digest, same signature");
    }

    use std::io::{Read as _, Write as _};
    use std::sync::Mutex;

    static RPC_ENV: Mutex<()> = Mutex::new(());

    /// A JSON-RPC node that routes by method: head, the AuthorizationUsed log
    /// (if any) and the receipt status.
    fn fake_rpc(log: Option<(&'static str, u64)>, receipt: &'static str, head: u64) -> String {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            for _ in 0..8 {
                let Ok((mut s, _)) = listener.accept() else { return };
                // Headers and body may arrive in separate reads.
                let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(200)));
                let mut buf = Vec::new();
                let mut chunk = [0u8; 4096];
                loop {
                    match s.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            buf.extend_from_slice(&chunk[..n]);
                            if String::from_utf8_lossy(&buf).contains("eth_") {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                let req = String::from_utf8_lossy(&buf).to_string();
                let result = if req.contains("eth_blockNumber") {
                    serde_json::json!(format!("0x{head:x}"))
                } else if req.contains("eth_getLogs") {
                    match log {
                        None => serde_json::json!([]),
                        Some((hash, block)) => serde_json::json!([{
                            "transactionHash": hash,
                            "blockNumber": format!("0x{block:x}"),
                        }]),
                    }
                } else if req.contains("eth_getTransactionReceipt") {
                    serde_json::json!({"status": receipt})
                } else {
                    serde_json::Value::Null
                };
                let body = serde_json::json!({"jsonrpc":"2.0","id":1,"result": result}).to_string();
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = s.write_all(resp.as_bytes());
            }
        });
        format!("http://{addr}")
    }

    const TX: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";
    const TX2: &str = "0x2222222222222222222222222222222222222222222222222222222222222222";

    fn with_nodes(a: &str, b: &str) {
        std::env::set_var("MU_X402_RPC_1", a);
        std::env::set_var("MU_X402_RPC_2", b);
    }

    fn status_of(nonce: [u8; 32], valid_before: u64) -> Result<TxStatus, ConnErr> {
        let c = sepolia();
        c.status(&TxRef::Authorization { nonce, valid_before })
    }

    #[test]
    fn the_authorization_used_topic_is_the_real_one() {
        // Pinned against the live Base Sepolia tx
        // 0x225f62faae475777f96927671eaf4121df38b5c91d48614079b1185b4dcf221e.
        let k = keccak(&[b"AuthorizationUsed(address,bytes32)"]);
        assert_eq!(format!("0x{}", hex(&k)), AUTHORIZATION_USED_TOPIC);
    }

    #[test]
    fn a_confirmed_authorization_settles_with_its_block() {
        let _g = RPC_ENV.lock().unwrap();
        // head 100, the log at 80: 21 confirmations, above the default 12.
        with_nodes(
            &fake_rpc(Some((TX, 80)), "0x1", 100),
            &fake_rpc(Some((TX, 80)), "0x1", 100),
        );
        assert_eq!(
            status_of([1; 32], u64::MAX).unwrap(),
            TxStatus::Settled { block: 80, effective_gas: 0 },
            "the facilitator pays the gas, so effective_gas is 0 by design"
        );
    }

    #[test]
    fn a_used_nonce_without_enough_confirmations_stays_pending() {
        let _g = RPC_ENV.lock().unwrap();
        // Same tx, but seen one block ago: not 12 confirmations yet.
        with_nodes(
            &fake_rpc(Some((TX, 99)), "0x1", 100),
            &fake_rpc(Some((TX, 99)), "0x1", 100),
        );
        assert_eq!(status_of([1; 32], u64::MAX).unwrap(), TxStatus::Pending);
    }

    #[test]
    fn a_failed_receipt_is_not_settled() {
        let _g = RPC_ENV.lock().unwrap();
        with_nodes(
            &fake_rpc(Some((TX, 10)), "0x0", 100),
            &fake_rpc(Some((TX, 10)), "0x0", 100),
        );
        assert!(matches!(status_of([1; 32], u64::MAX), Err(ConnErr::Unknown(_))));
    }

    #[test]
    fn an_unused_nonce_stays_pending_inside_the_window() {
        let _g = RPC_ENV.lock().unwrap();
        with_nodes(&fake_rpc(None, "0x1", 100), &fake_rpc(None, "0x1", 100));
        assert_eq!(status_of([2; 32], u64::MAX).unwrap(), TxStatus::Pending);
    }

    #[test]
    fn an_unused_nonce_expires_to_failed() {
        let _g = RPC_ENV.lock().unwrap();
        with_nodes(&fake_rpc(None, "0x1", 100), &fake_rpc(None, "0x1", 100));
        assert!(matches!(status_of([3; 32], 1).unwrap(), TxStatus::Failed { .. }));
    }

    #[test]
    fn nodes_that_disagree_on_the_tx_are_unknown() {
        let _g = RPC_ENV.lock().unwrap();
        with_nodes(
            &fake_rpc(Some((TX, 80)), "0x1", 100),
            &fake_rpc(Some((TX2, 80)), "0x1", 100),
        );
        assert!(matches!(status_of([4; 32], u64::MAX), Err(ConnErr::Unknown(_))));
    }

    #[test]
    fn one_node_is_unknown_not_a_guess() {
        let _g = RPC_ENV.lock().unwrap();
        std::env::remove_var("MU_X402_RPC_2");
        std::env::set_var("MU_X402_RPC_1", "http://127.0.0.1:9");
        assert!(matches!(status_of([5; 32], u64::MAX), Err(ConnErr::Unknown(_))));
        std::env::remove_var("MU_X402_RPC_1");
    }

    #[test]
    fn the_window_comes_only_from_the_offer() {
        let c = sepolia();
        assert_eq!(c.auth_window_secs(None), None, "no offer, no window");
        let ch = challenge_t(ISSUER, "100000", 60);
        assert_eq!(c.auth_window_secs(Some(&ch)), Some(60));
        let mut over = challenge_t(ISSUER, "100000", MAX_VALIDITY_SECONDS + 1);
        assert_eq!(c.auth_window_secs(Some(&over)), None, "over an hour is no window");
        over.max_timeout_seconds = 0;
        assert_eq!(c.auth_window_secs(Some(&over)), None);
    }

    #[test]
    fn a_window_longer_than_the_offer_is_refused() {
        // The issuer allows 60 s, the WAL carries 300 s: refuse instead of
        // signing an authorization the seller will reject.
        let i = intent(ISSUER, 100_000, 84532);
        let c = sepolia();
        match c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), ctx(0xac, challenge_t(ISSUER, "100000", 60))) {
            Err(ConnErr::Rejected(RejectReason::ChallengeMismatch)) => {}
            other => panic!("expected ChallengeMismatch, got {other:?}"),
        }
        // nothing is signed: the error came back instead of an Executed
    }

    #[test]
    fn without_an_offer_nothing_is_signed() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, 84532);
        let ctx = crate::ExecCtx { auth: Some(params(0xab)), offer: None };
        match c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), ctx) {
            Err(ConnErr::Rejected(RejectReason::NoChallenge)) => {}
            other => panic!("expected NoChallenge, got {other:?}"),
        }

    }

    #[test]
    fn an_offer_of_the_previous_intent_does_not_leak_into_the_next() {
        let c = sepolia();
        let v = vault([0x11; 32]);
        let first = intent(ISSUER, 100_000, 84532);
        let offer = challenge(ISSUER, "100000");
        // The first payment answers this offer.
        c.execute_auth(&first, v.tx_signer().unwrap(), ctx(0xab, offer.clone())).unwrap();
        // A different intent has to bring its own offer: the connector kept none.
        let second = intent("0x1111111111111111111111111111111111111111", 100_000, 84532);
        match c.execute_auth(&second, v.tx_signer().unwrap(), ctx(0xcd, offer)) {
            Err(ConnErr::Rejected(RejectReason::ChallengeMismatch)) => {}
            other => panic!("expected ChallengeMismatch, got {other:?}"),
        }

    }

    #[test]
    fn one_offer_pays_once() {
        let c = sepolia();
        let v = vault([0x11; 32]);
        let i = intent(ISSUER, 100_000, 84532);
        let offer = challenge(ISSUER, "100000");
        c.execute_auth(&i, v.tx_signer().unwrap(), ctx(0xab, offer.clone())).unwrap();
        // Same 402 answer, second payment: refused, a new offer is needed.
        match c.execute_auth(&i, v.tx_signer().unwrap(), ctx(0xcd, offer)) {
            Err(ConnErr::Rejected(RejectReason::OfferAlreadyUsed)) => {}
            other => panic!("expected OfferAlreadyUsed, got {other:?}"),
        }
    }

    #[test]
    fn execute_without_a_wal_nonce_is_refused() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, BASE_SEPOLIA.chain_id);
        let v = vault([0x11; 32]);
        match c.execute(&i, v.tx_signer().unwrap()) {
            Err(ConnErr::Config(m)) => assert!(m.contains("execute_auth"), "{m}"),
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn execute_auth_without_params_is_refused() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, BASE_SEPOLIA.chain_id);
        let v = vault([0x11; 32]);
        let ctx = crate::ExecCtx { auth: None, offer: Some(challenge(ISSUER, "100000")) };
        assert!(c.execute_auth(&i, v.tx_signer().unwrap(), ctx).is_err());
    }

    #[test]
    fn the_same_wal_nonce_gives_the_same_header() {
        use crate::AuthParams;
        let v = vault([0x11; 32]);
        let i = intent(ISSUER, 100_000, BASE_SEPOLIA.chain_id);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let p = AuthParams { nonce: [0xab; 32], valid_after: now.saturating_sub(60), valid_before: now + 300 };
        let mut headers = Vec::new();
        for _ in 0..2 {
            // A fresh process after a crash: the same WAL nonce signs the same
            // digest, so the headers match and the seller cannot charge twice.
            let c = sepolia();
            let c2 = crate::ExecCtx { auth: Some(p), offer: Some(challenge(ISSUER, "100000")) };
            let ex = c.execute_auth(&i, v.tx_signer().unwrap(), c2).expect("pay");
            headers.push(ex.header.expect("header"));
        }
        assert_eq!(headers[0], headers[1], "same nonce, same signature, no second payment");
    }

    #[test]
    fn header_is_x402_version_1() {
        let v = vault([0x11; 32]);
        let c = sepolia();
        let a = c
            .authorize(&v.tx_signer().unwrap(), parse_addr(ISSUER).unwrap(), 100_000, 1_760_000_000, 1_760_000_600, [0xab; 32])
            .unwrap();
        use base64::Engine;
        let raw = base64::engine::general_purpose::STANDARD.decode(a.to_header()).unwrap();
        let j: serde_json::Value = serde_json::from_slice(&raw).unwrap();
        assert_eq!(j["x402Version"], 1);
        assert_eq!(j["scheme"], "exact");
        assert_eq!(j["network"], "base-sepolia");
        assert_eq!(j["payload"]["signature"], format!("0x{SIG}"));
        let au = &j["payload"]["authorization"];
        assert_eq!(au["from"], format!("0x{PAYER}"));
        assert_eq!(au["to"], ISSUER);
        assert_eq!(au["value"], "100000");
        assert_eq!(au["validAfter"], "1760000000");
        assert_eq!(au["validBefore"], "1760000600");
        assert_eq!(au["nonce"], format!("0x{}", "ab".repeat(32)));
    }

    #[test]
    fn a_key_that_is_not_the_wallet_signs_nothing() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, 84532);
        let e = c
            .execute_auth(&i, vault([0x22; 32]).tx_signer().unwrap(), ctx(0xab, challenge(ISSUER, "100000")))
            .unwrap_err();
        assert_eq!(e, ConnErr::Rejected(RejectReason::WalletMismatch));
    }

    #[test]
    fn execute_answers_a_matching_challenge_once() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, 84532);
        let r = c
            .execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), ctx(0xab, challenge(ISSUER, "100000")))
            .unwrap();
        assert!(matches!(r.tx, TxRef::Authorization { .. }));
        assert!(r.header.is_some(), "the header comes back with the result");
    }

    #[test]
    fn a_challenge_for_another_recipient_is_refused_by_the_self_check() {
        // P0-9: the challenge names a different recipient than the intent.
        // The connector must refuse; it must not compare the intent with itself.
        let other = "0x1111111111111111111111111111111111111111";
        let i = intent(ISSUER, 100_000, 84532);
        let c = sepolia();
        match c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), ctx(0xab, challenge(other, "100000"))) {
            Err(ConnErr::Rejected(RejectReason::ChallengeMismatch)) => {}
            other => panic!("expected ChallengeMismatch, got {other:?}"),
        }
        // nothing is signed: the error came back instead of an Executed
    }

    #[test]
    fn a_challenge_that_differs_from_the_intent_signs_nothing() {
        let other = "0x1111111111111111111111111111111111111111";
        let i = intent(ISSUER, 100_000, 84532);
        for ch in [
            challenge(other, "100000"),           // another recipient
            challenge(ISSUER, "100001"),          // another amount
            {
                let mut c = challenge(ISSUER, "100000");
                c.asset = "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913".into(); // another token
                c
            },
            {
                let mut c = challenge(ISSUER, "100000");
                c.domain_name = "Fake USDC".into(); // another EIP-712 domain
                c
            },
            {
                let mut c = challenge(ISSUER, "100000");
                c.network = "base".into(); // another network
                c
            },
            {
                let mut c = challenge(ISSUER, "100000");
                c.domain_version = "1".into(); // another domain version
                c
            },
            {
                let mut c = challenge(ISSUER, "100000");
                c.max_timeout_seconds = MAX_VALIDITY_SECONDS + 1;
                c
            },
        ] {
            let c = sepolia();
            let e = c
                .execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), ctx(0xab, ch.clone()))
                .unwrap_err();
            assert_eq!(e, ConnErr::Rejected(RejectReason::ChallengeMismatch), "{ch:?}");
        }
    }

    #[test]
    fn a_402_body_for_another_network_or_version_is_refused() {
        let body = |v: u64, net: &str| {
            serde_json::json!({"x402Version": v, "accepts": [{"scheme": "exact", "network": net,
                "maxAmountRequired": "1", "payTo": ISSUER, "maxTimeoutSeconds": 60,
                "asset": "0x036cbd53842c5426634e7929541ec2318f3dcf7e"}]})
            .to_string()
        };
        assert!(X402Challenge::from_402_body(&body(1, "base-sepolia"), &BASE_SEPOLIA).is_ok());
        assert!(X402Challenge::from_402_body(&body(2, "base-sepolia"), &BASE_SEPOLIA).is_err());
        assert!(X402Challenge::from_402_body(&body(1, "base"), &BASE_SEPOLIA).is_err());
        assert!(X402Challenge::from_402_body("not json", &BASE_SEPOLIA).is_err());
    }

    #[test]
    fn the_nonce_comes_from_the_wal() {
        let c = sepolia();
        let i = intent(ISSUER, 1, 84532);
        let p = params(0xcd);
        let c1 = crate::ExecCtx { auth: Some(p), offer: Some(challenge(ISSUER, "1")) };
        match c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), c1).unwrap().tx {
            TxRef::Authorization { nonce, valid_before } => {
                assert_eq!(nonce, p.nonce);
                assert_eq!(valid_before, p.valid_before);
            }
            other => panic!("expected an authorization, got {other:?}"),
        }
    }

    #[test]
    fn chain_mismatch_is_refused() {
        let c = sepolia();
        let i = intent(ISSUER, 1, 8453);
        assert_eq!(
            c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), crate::ExecCtx::default()).unwrap_err(),
            ConnErr::Rejected(RejectReason::ChainMismatch)
        );
        assert_eq!(c.quote(&i).unwrap_err(), ConnErr::Rejected(RejectReason::ChainMismatch));
        assert_eq!(c.quote(&intent(ISSUER, 1, 84532)).unwrap().gas_estimate, Amount::ZERO);
    }

    #[test]
    fn status_without_two_nodes_is_unknown_not_a_time_guess() {
        let _g = RPC_ENV.lock().unwrap();
        std::env::remove_var("MU_X402_RPC_1");
        std::env::remove_var("MU_X402_RPC_2");
        let c = sepolia();
        // P0-8: the chain says whether the nonce is used; without two nodes the
        // answer is Unknown, so the reserve is kept.
        assert!(matches!(
            c.status(&TxRef::Authorization { nonce: [1; 32], valid_before: 1 }),
            Err(ConnErr::Unknown(_))
        ));
        assert!(matches!(
            c.status(&TxRef::Authorization { nonce: [1; 32], valid_before: u64::MAX }),
            Err(ConnErr::Unknown(_))
        ));
        assert!(matches!(
            c.status(&TxRef::Simulated { id: [0; 16] }).unwrap(),
            TxStatus::Failed { .. }
        ));
    }

    #[test]
    fn known_networks() {
        assert_eq!(network_for_chain(8453), Some(BASE));
        assert_eq!(network_for_chain(84532), Some(BASE_SEPOLIA));
        assert_eq!(network_for_chain(1), None);
        assert_eq!(USDC_BASE, parse_addr("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913").unwrap());
    }
}
