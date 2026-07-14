//! M1 mu-core — единственный владелец формата μ-объекта.
//!
//! RISK-M1-1: Ω неизменяем вне reissue — поле приватно (нет &mut), apply_delta
//!            побайтно сверяет Ω до/после пересборки (defense-in-depth).
//! RISK-M1-2: rollback-атака — verify_against_log сверяет μ.log_head с хвостом цепи M5.
//! RISK-M1-3: атомарная запись tmp+fsync+rename+fsync(dir); авто-отката из mu.prev НЕТ.
//! RISK-M1-4: строгий hardened-парсер (семейство mu-wire): лишний байт = CborMalformed.
#![forbid(unsafe_code)]

pub mod format;
pub mod object;

pub use object::{Mu, CoreErr, MU_MAX_SIZE};
