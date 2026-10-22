//! v4: the answer to "give me the header of request N".
//!
//! The lookup is keyed by (agent, request nonce): two agents may pick the same
//! nonce, and one agent must never see another's header.

use mu_log::{Kind, Log};

/// The answer to a v4 header request.
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderAnswer {
    Found(String),
    Expired,
    Missing,
}

/// Two agents can pick the same request nonce, so the lookup is keyed by both.
pub fn header_answer(log: &Log, agent_id: &str, ref_nonce: u64, now: u64) -> HeaderAnswer {
    let rec = log.entries().iter().rev().find_map(|e| match &e.kind {
        Kind::AuthHeader { agent_id: a, req_nonce, header, valid_before, .. }
            if a == agent_id && *req_nonce == ref_nonce =>
        {
            Some((header.clone(), *valid_before))
        }
        _ => None,
    });
    match rec {
        None => HeaderAnswer::Missing,
        Some((_, valid_before)) if now > valid_before => HeaderAnswer::Expired,
        Some((header, _)) => HeaderAnswer::Found(header),
    }
}


#[cfg(test)]
mod tests {
    use super::{header_answer, HeaderAnswer};
    use mu_common::Hash32;
    use mu_log::{Kind, Log};
    use mu_vault::backend::SoftVault;

    fn vault() -> SoftVault {
        SoftVault::for_test([1; 32], [2; 32], [3; 32])
    }

    fn log_with_header(dir: &std::path::Path, v: &SoftVault) -> Log {
        let mut log = Log::open(&dir.join("l.mulog"), v).unwrap();
        log.append(
            Kind::AuthHeader {
                agent_id: "agent-1".into(),
                req_nonce: 7,
                intent_hash: Hash32([1; 32]),
                auth_nonce: [2; 32],
                valid_before: 2_000,
                header: "X-PAYMENT-abc".into(),
            },
            1_000,
            v,
        )
        .unwrap();
        log
    }

    #[test]
    fn a_header_request_replays_the_same_header() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let log = log_with_header(dir.path(), &v);
        assert_eq!(header_answer(&log, "agent-1", 7, 1_500), HeaderAnswer::Found("X-PAYMENT-abc".into()));
        assert_eq!(header_answer(&log, "agent-1", 7, 1_600), HeaderAnswer::Found("X-PAYMENT-abc".into()));
    }

    #[test]
    fn another_agent_cannot_read_the_header() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let log = log_with_header(dir.path(), &v);
        assert_eq!(header_answer(&log, "agent-2", 7, 1_500), HeaderAnswer::Missing);
        assert_eq!(header_answer(&log, "agent-1", 8, 1_500), HeaderAnswer::Missing);
    }

    #[test]
    fn an_expired_header_is_not_handed_out() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let log = log_with_header(dir.path(), &v);
        assert_eq!(header_answer(&log, "agent-1", 7, 2_001), HeaderAnswer::Expired);
    }

    #[test]
    fn a_header_survives_a_restart() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let path = dir.path().join("l.mulog");
        {
            let mut log = Log::open(&path, &v).unwrap();
            log.append(
                Kind::AuthHeader {
                    agent_id: "agent-1".into(),
                    req_nonce: 7,
                    intent_hash: Hash32([1; 32]),
                    auth_nonce: [2; 32],
                    valid_before: 2_000,
                    header: "X-PAYMENT-abc".into(),
                },
                1_000,
                &v,
            )
            .unwrap();
            // The daemon died before answering: the record is already on disk.
        }
        let log = Log::open(&path, &v).unwrap();
        assert_eq!(header_answer(&log, "agent-1", 7, 1_500), HeaderAnswer::Found("X-PAYMENT-abc".into()));
    }
}
