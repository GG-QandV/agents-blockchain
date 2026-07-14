//! RISK-X-2: разделение источников времени.
//! now_unix — для timestamp'ов записей и окна 24ч.
//! monotonic — для дедлайнов/TTL (не подвержен переводу часов).
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
