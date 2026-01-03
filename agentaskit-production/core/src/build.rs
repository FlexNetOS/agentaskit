//! Build Information Module
//!
//! Provides compile-time build information and version tracking utilities.
//! Note: The actual build script is at core/build.rs (crate root).

/// Build metadata
pub struct BuildInfo {
    /// Build timestamp
    pub build_time: &'static str,
    /// Package version from Cargo.toml
    pub version: &'static str,
    /// Target architecture
    pub target: &'static str,
    /// Build profile (debug/release)
    pub profile: &'static str,
}

impl BuildInfo {
    /// Get current build information
    pub const fn current() -> Self {
        Self {
            build_time: env!("BUILD_TIME"),
            version: env!("CARGO_PKG_VERSION"),
            target: env!("TARGET"),
            profile: if cfg!(debug_assertions) { "debug" } else { "release" },
        }
    }
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self::current()
    }
}

/// Get the build timestamp
pub fn build_time() -> &'static str {
    env!("BUILD_TIME")
}

/// Get the package version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info() {
        let info = BuildInfo::current();
        assert!(!info.version.is_empty());
    }
}
