//! `Validator` — legacy API wrapping the newer `validate` function.
//!
//! New code should call [`crate::validate::validate`] directly.  This module
//! exists for backwards-compatibility with existing tests.

use crate::{validate, MapPackage, MapPackageError};

/// Validation error (re-exported as the `Validation` variant of `MapPackageError`).
pub type ValidationError = MapPackageError;
/// `Ok(())` or `Err(MapPackageError)`.
pub type ValidationResult = Result<(), MapPackageError>;

/// Thin wrapper around [`validate`] for backwards-compatibility.
pub struct Validator;

impl Validator {
    /// Validate a [`MapPackage`], returning the first error found.
    pub fn validate(package: &MapPackage) -> ValidationResult {
        validate(package)
    }
}
