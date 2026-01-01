#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_check_creation() {
        use windows_optimizer::types::{Check, CheckStatus};

        let check = Check::new("Test Check", "Value", CheckStatus::Optimal);
        assert_eq!(check.name, "Test Check");
        assert_eq!(check.value, "Value");
        assert_eq!(check.status, CheckStatus::Optimal);
    }

    #[test]
    fn test_check_with_description() {
        use windows_optimizer::types::{Check, CheckStatus};

        let check = Check::new("Test", "Val", CheckStatus::Info).with_description("This is a test");
        assert_eq!(check.description, "This is a test");
    }
}
