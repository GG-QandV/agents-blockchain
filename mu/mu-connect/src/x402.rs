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
    challenge: Mutex<Option<X402Challenge>>,
    last_auth: Mutex<Option<SignedAuthorization>>,
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
            challenge: Mutex::new(None),
            last_auth: Mutex::new(None),
        }
    }

    /// The `X-PAYMENT` header of the last authorization, once.
    pub fn take_header(&self) -> Option<String> {
        self.last_auth.lock().unwrap().take().map(|a| a.to_header())
    }

    /// The 402 challenge the next `execute` answers. Checked against the
    /// intent at `execute`, not here.
    pub fn set_challenge(&self, challenge: &X402Challenge) {
        *self.challenge.lock().unwrap() = Some(challenge.clone());
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
        auth: Option<crate::AuthParams>,
    ) -> Result<TxRef, ConnErr> {
        if i.chain_id != self.chain_id || i.recipient.chain_id() != self.chain_id {
            return Err(ConnErr::Rejected(RejectReason::ChainMismatch));
        }
        let params = auth.ok_or_else(|| {
            ConnErr::Config("x402 requires a nonce written to the WAL".into())
        })?;
        if let Some(c) = self.challenge.lock().unwrap().take() {
            c.matches(i, &self.network)?;
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ConnErr::Unknown(format!("clock: {e:?}")))?
            .as_secs();
        let auth = self.authorize(
            &signer,
            *i.recipient.bytes(),
            i.amount.minor(),
            now.saturating_sub(VALID_AFTER_SKEW),
            params.valid_before,
            params.nonce,
        )?;
        // RISK-M7-5: what was signed is what the intent says.
        if auth.to != *i.recipient.bytes() || auth.value != i.amount.minor() {
            return Err(ConnErr::Config("x402 self-check: auth mismatch".into()));
        }
        let r = TxRef::Authorization { nonce: auth.nonce, valid_before: auth.valid_before };
        *self.last_auth.lock().unwrap() = Some(auth);
        Ok(r)
    }

    fn take_header(&self) -> Option<String> {
        X402Connector::take_header(self)
    }

    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        let valid_before = match r {
            TxRef::Authorization { valid_before, .. } => *valid_before,
            TxRef::Real { .. } => return Err(ConnErr::Config("x402 got Real txref".into())),
            TxRef::Simulated { .. } => {
                return Ok(TxStatus::Failed { reason: crate::FailReason::Simulated })
            }
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ConnErr::Unknown(format!("clock: {e:?}")))?
            .as_secs();
        // Without a chain reader (authorizationState) the time window is all
        // we know: past validBefore the authorization can never settle.
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
        crate::AuthParams { nonce: [n; 32], valid_before: now + 300 }
    }

    fn challenge(pay_to: &str, amount: &str) -> X402Challenge {
        X402Challenge::from_402_body(
            &serde_json::json!({
                "x402Version": 1,
                "accepts": [{
                    "scheme": "exact", "network": "base-sepolia",
                    "maxAmountRequired": amount, "resource": "https://issuer.example/bare/issue",
                    "payTo": pay_to, "maxTimeoutSeconds": 300,
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

    #[test]
    fn execute_without_a_wal_nonce_is_refused() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, BASE_SEPOLIA.chain_id);
        let v = vault([0x11; 32]);
        c.set_challenge(&challenge(ISSUER, "100000"));
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
        assert!(c.execute_auth(&i, v.tx_signer().unwrap(), None).is_err());
    }

    #[test]
    fn the_same_wal_nonce_gives_the_same_header() {
        use crate::AuthParams;
        let v = vault([0x11; 32]);
        let i = intent(ISSUER, 100_000, BASE_SEPOLIA.chain_id);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let p = AuthParams { nonce: [0xab; 32], valid_before: now + 300 };
        let mut headers = Vec::new();
        for _ in 0..2 {
            let c = sepolia();
            c.set_challenge(&challenge(ISSUER, "100000"));
            c.execute_auth(&i, v.tx_signer().unwrap(), Some(p)).expect("pay");
            headers.push(c.take_header().expect("header"));
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
        let e = c.execute_auth(&i, vault([0x22; 32]).tx_signer().unwrap(), Some(params(0xab))).unwrap_err();
        assert_eq!(e, ConnErr::Rejected(RejectReason::WalletMismatch));
        assert!(c.take_header().is_none());
    }

    #[test]
    fn execute_answers_a_matching_challenge_once() {
        let c = sepolia();
        let i = intent(ISSUER, 100_000, 84532);
        c.set_challenge(&challenge(ISSUER, "100000"));
        let r = c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), Some(params(0xab))).unwrap();
        assert!(matches!(r, TxRef::Authorization { .. }));
        assert!(c.take_header().is_some());
        assert!(c.take_header().is_none(), "the header is handed out once");
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
                c.max_timeout_seconds = MAX_VALIDITY_SECONDS + 1;
                c
            },
        ] {
            let c = sepolia();
            c.set_challenge(&ch);
            let e = c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), Some(params(0xab))).unwrap_err();
            assert_eq!(e, ConnErr::Rejected(RejectReason::ChallengeMismatch), "{ch:?}");
            assert!(c.take_header().is_none());
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
        match c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), Some(p)).unwrap() {
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
            c.execute_auth(&i, vault([0x11; 32]).tx_signer().unwrap(), Some(params(0xab))).unwrap_err(),
            ConnErr::Rejected(RejectReason::ChainMismatch)
        );
        assert_eq!(c.quote(&i).unwrap_err(), ConnErr::Rejected(RejectReason::ChainMismatch));
        assert_eq!(c.quote(&intent(ISSUER, 1, 84532)).unwrap().gas_estimate, Amount::ZERO);
    }

    #[test]
    fn status_follows_the_time_window() {
        let c = sepolia();
        assert!(matches!(
            c.status(&TxRef::Authorization { nonce: [1; 32], valid_before: 1 }).unwrap(),
            TxStatus::Failed { .. }
        ));
        assert_eq!(
            c.status(&TxRef::Authorization { nonce: [1; 32], valid_before: u64::MAX }).unwrap(),
            TxStatus::Pending
        );
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
