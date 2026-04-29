//! iGame runtime crate.

/// Returns a greeting string.
pub fn hello() -> &'static str {
    "Hello from iGame runtime!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from iGame runtime!");
    }
}
