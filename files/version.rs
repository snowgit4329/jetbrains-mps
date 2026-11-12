//! Static build info for Jetbrains Mps.

pub const BUILD_VERSION: &str  = "v4.8.3";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "7f80b7523159";
pub const BUILD_TIMESTAMP: u64 = 1799986176;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Jetbrains Mps", BUILD_VERSION, BUILD_CHANNEL)
}
