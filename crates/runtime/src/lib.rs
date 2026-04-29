//! igame-runtime: the iGame game engine runtime.

pub use igame_shared::schema_version;

/// Returns the engine version string.
pub fn engine_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_version_is_non_empty() {
        assert!(!engine_version().is_empty());
    }

    #[test]
    fn schema_version_accessible_from_runtime() {
        assert!(!schema_version().is_empty());
    }
}
