//! Who is on the other end of the socket.
//!
//! File permissions are the first wall; `SO_PEERCRED` is the second. The peer's
//! uid is checked against the daemon's own uid and an explicit allow-list
//! (`MU_AGENT_UIDS`), so another local user who somehow opens the socket still
//! gets nothing. The call itself goes through `nix`; our own crates stay
//! `forbid(unsafe_code)`.

use std::os::unix::net::UnixStream;
use std::path::Path;

/// The kernel's view of the peer uid. `None` when the platform cannot say —
/// then the caller must refuse, not guess.
pub fn peer_uid(stream: &UnixStream) -> Option<u32> {
    use std::os::fd::AsFd;
    nix::sys::socket::getsockopt(stream, nix::sys::socket::sockopt::PeerCredentials)
        .ok()
        .map(|c| c.uid())
}

/// Pure decision, so the rule is testable without sockets.
pub fn peer_allowed(uid: u32, daemon_uid: u32, allowed: &[u32]) -> bool {
    uid == daemon_uid || allowed.contains(&uid)
}

pub fn daemon_uid() -> u32 {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata("/proc/self")
        .map(|m| m.uid())
        .unwrap_or(0)
}

/// `MU_AGENT_UIDS=1001,1002` (empty or unset → nobody extra is allowed).
pub fn allowed_uids() -> Vec<u32> {
    std::env::var("MU_AGENT_UIDS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// The socket directory is private to the daemon (0700); the socket itself is
/// 0660 only when an agent group/uid list exists, 0600 otherwise.
pub fn prepare_socket(sock_path: &Path, shared: bool) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    if let Some(dir) = sock_path.parent() {
        std::fs::create_dir_all(dir)?;
        std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
    }
    let _ = std::fs::remove_file(sock_path);
    let mode = if shared { 0o660 } else { 0o600 };
    // The file is created by bind(); its mode is set by the caller after that.
    let _ = mode;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::peer_allowed;

    #[test]
    fn the_daemons_own_user_is_allowed() {
        assert!(peer_allowed(1000, 1000, &[]));
    }

    #[test]
    fn a_listed_agent_user_is_allowed() {
        assert!(peer_allowed(1001, 1000, &[1001, 1002]));
    }

    #[test]
    fn anyone_else_is_refused() {
        assert!(!peer_allowed(1003, 1000, &[1001, 1002]));
        assert!(!peer_allowed(0, 1000, &[]), "root is not special here");
    }
}
