//! igame-shared: common data structures and utilities shared across iGame crates.

/// Returns the schema version for map packages.
pub fn schema_version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_semver() {
        let v = schema_version();
        assert!(!v.is_empty());
        assert_eq!(v.split('.').count(), 3);
    }
}
