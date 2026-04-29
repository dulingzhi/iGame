//! Registry — manages installed/available packages on disk (stub).

/// Stub registry; full implementation in M6.
#[derive(Debug, Default)]
pub struct Registry {
    pub root: std::path::PathBuf,
}

impl Registry {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }
}
