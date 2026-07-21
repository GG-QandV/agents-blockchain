//! RISK-X-2: separation of time sources.
//! now_unix — for record timestamps and 24h window.
//! monotonic — for deadlines/TTL (not affected by clock adjustments).
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub trait Clock: Send + Sync {
    fn now_unix(&self) -> u64;
    fn monotonic(&self) -> Instant;
}

pub struct SysClock;

impl Clock for SysClock {
    fn now_unix(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
    fn monotonic(&self) -> Instant {
        Instant::now()
    }
}
