// Definition of the MySlug trait
pub trait MySlug {
    /// Checks if a string is already in slug format
    fn is_slug(&self) -> bool;

    /// Converts a string to slug format
    fn to_slug(&self) -> String;
}

// Generic implementation for any type that implements AsRef<str>
impl<T: AsRef<str>> MySlug for T {
    fn is_slug(&self) -> bool {
        let s = self.as_ref();
        if s.is_empty() {
            return false;
        }

        // A slug can only contain lowercase letters, numbers and hyphens
        s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }

    fn to_slug(&self) -> String {
        let s = self.as_ref();

        // Transform the string to lowercase characters
        let mut result = String::new();
        let mut last_was_whitespace = false;

        for c in s.chars() {
            if c.is_ascii_alphanumeric() {
                // Add alphanumeric characters in lowercase
                result.push(c.to_ascii_lowercase());
                last_was_whitespace = false;
            } else if (c.is_whitespace() || c == '-' || c == '_') && !last_was_whitespace && !result.is_empty() {
                // Replace spaces, hyphens and underscores with a single hyphen
                result.push('-');
                last_was_whitespace = true;
            }
        }

        // Remove any trailing hyphens
        if result.ends_with('-') {
            result.pop();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_slug() {
        assert!(String::from("hello-world").is_slug());
        assert!("test-123".is_slug());
        assert!("a-b-c".is_slug());

        assert!(!String::from("Hello World").is_slug());
        assert!(!"Test_123".is_slug());
        assert!(!"a b c".is_slug());
        assert!(!"".is_slug());
    }

    #[test]
    fn test_to_slug() {
        assert_eq!("Hello World".to_slug(), "hello-world");
        assert_eq!("Test_123".to_slug(), "test-123");
        assert_eq!("  Multiple   Spaces  ".to_slug(), "multiple-spaces");
        assert_eq!("Special @#$% Characters".to_slug(), "special-characters");
        assert_eq!("-leading-and-trailing-".to_slug(), "leading-and-trailing");
    }
}

