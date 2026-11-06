//! P11a (Spec S5 §3.1 step 1): the intent to anchor a page of a keysingate
//! capsule on Solana, and the check that runs **before** anything is signed.
//!
//! The memo is the whole point of an anchor, and the only thing a wallet that
//! pays for it must be sure of: it signs a transaction that carries exactly the
//! memo of the core v2 (`ksg:bs:v1` blind seal, `ksg:st:v1` status record) and
//! nothing else. The grammar below is the one `ksg-verify-v2` reads
//! (`SealMemo::parse`, `StatusMemo::parse`) — a memo it would not read is not
//! worth a fee. The old memo of the core v1 (`ksg:cp:v1`) is refused.
//!
//! Nothing here touches the network or a key.

/// The tag of a blind seal memo (core v2).
pub const SEAL_MEMO_TAG: &str = "ksg:bs:v1";
/// The tag of a status record memo (core v2).
pub const STATUS_MEMO_TAG: &str = "ksg:st:v1";
/// Bytes of each blind field of a seal memo.
pub const SEAL_BLIND_LEN: usize = 16;
/// A sha-256 multihash in hex: the `1220` prefix and 32 bytes.
const MULTIHASH_HEX_LEN: usize = 4 + 64;

/// What a wallet is asked to pay for: one memo, on one cluster, for one capsule,
/// under a cap on the fee.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorIntent {
    /// The memo text to put on the chain.
    pub memo: String,
    /// The most the owner lets one anchor cost, in lamports.
    pub fee_cap_lamports: u64,
    /// The cluster the anchor goes to (`devnet`, `mainnet-beta`, `localnet`).
    pub cluster: String,
    /// The capsule the anchor belongs to (its container's multihash): the rules
    /// of the owner are per capsule.
    pub capsule: String,
}

/// What kind of memo the intent carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoKind {
    /// `ksg:bs:v1`: a blind seal of the journal.
    Seal,
    /// `ksg:st:v1`: a status record.
    Status,
}

/// Why an intent is refused before signing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentError {
    /// The memo is not a memo of the core v2: anything else is refused whole.
    NotOurMemo,
    /// The intent names another cluster than the one μ Gate is configured for.
    WrongCluster {
        /// The cluster μ Gate pays on.
        configured: String,
        /// The cluster the intent asks for.
        intent: String,
    },
    /// The quoted fee passes the cap the intent carries.
    FeeAboveCap {
        /// The fee the node quoted, lamports.
        fee: u64,
        /// The cap, lamports.
        cap: u64,
    },
}

impl core::fmt::Display for IntentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotOurMemo => f.write_str("the memo is not a memo of the keysingate core v2"),
            Self::WrongCluster { configured, intent } => {
                write!(f, "the intent names cluster {intent}, μ Gate is configured for {configured}")
            }
            Self::FeeAboveCap { fee, cap } => {
                write!(f, "the fee {fee} lamports passes the cap {cap}")
            }
        }
    }
}

impl std::error::Error for IntentError {}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The kind of a memo of the core v2, or [`IntentError::NotOurMemo`].
///
/// Strict on purpose: exactly three single-space-separated fields, the tag and two
/// fields of the exact width in lower-case hex; no other whitespace, no line
/// break, nothing after. A memo `ksg-verify-v2` would parse is the only one that
/// passes.
///
/// # Errors
///
/// [`IntentError::NotOurMemo`] for anything else.
pub fn memo_kind(memo: &str) -> Result<MemoKind, IntentError> {
    let fields: Vec<&str> = memo.split(' ').collect();
    let [tag, a, b] = fields.as_slice() else {
        return Err(IntentError::NotOurMemo);
    };
    let kind = match *tag {
        SEAL_MEMO_TAG => {
            if is_lower_hex(a, 2 * SEAL_BLIND_LEN) && is_lower_hex(b, 2 * SEAL_BLIND_LEN) {
                Some(MemoKind::Seal)
            } else {
                None
            }
        }
        STATUS_MEMO_TAG => {
            let multihash = |s: &str| s.starts_with("1220") && is_lower_hex(s, MULTIHASH_HEX_LEN);
            if multihash(a) && multihash(b) {
                Some(MemoKind::Status)
            } else {
                None
            }
        }
        _ => None,
    };
    kind.ok_or(IntentError::NotOurMemo)
}

/// The check before signing: a memo of the core v2, the configured cluster, a
/// quoted fee within the cap. Returns the memo's kind.
///
/// # Errors
///
/// The first thing that does not hold, as [`IntentError`].
pub fn check_intent(
    intent: &AnchorIntent,
    configured_cluster: &str,
    quoted_fee_lamports: u64,
) -> Result<MemoKind, IntentError> {
    let kind = memo_kind(&intent.memo)?;
    if intent.cluster != configured_cluster {
        return Err(IntentError::WrongCluster {
            configured: configured_cluster.to_owned(),
            intent: intent.cluster.clone(),
        });
    }
    if quoted_fee_lamports > intent.fee_cap_lamports {
        return Err(IntentError::FeeAboveCap {
            fee: quoted_fee_lamports,
            cap: intent.fee_cap_lamports,
        });
    }
    Ok(kind)
}
