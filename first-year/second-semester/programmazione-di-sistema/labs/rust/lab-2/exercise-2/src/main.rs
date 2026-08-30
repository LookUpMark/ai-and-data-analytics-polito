use clap::Parser;

// Constants for character substitution
// SUBS_I contains the accented characters that need to be replaced
const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
// SUBS_O contains the corresponding non-accented replacements for characters in SUBS_I
// Each character in SUBS_O at position i corresponds to the character at position i in SUBS_I
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzzz";

/// Convert a character to its non-accented version or to a dash if not allowed
/// This function implements the character conversion rules:
/// - Lowercase letters and digits are kept as is
/// - Accented characters are converted to their non-accented equivalents
/// - All other characters are converted to dashes
fn conv(c: char) -> char {
    // Convert to lowercase first - returning first char from iterator
    // to_lowercase() returns an iterator since in some languages, 
    // a single uppercase letter can map to multiple lowercase characters
    let c_lower = c.to_lowercase().next().unwrap_or(c);
    
    // If it's already a valid character (lowercase letter or digit), return it as is
    if c_lower.is_ascii_lowercase() || c_lower.is_ascii_digit() {
        return c_lower;
    }
    
    // Convert SUBS_I and SUBS_O to vectors of characters for indexing
    // We need to do this because Rust strings are UTF-8 encoded and can't be indexed directly
    let subs_i: Vec<char> = SUBS_I.chars().collect();
    let subs_o: Vec<char> = SUBS_O.chars().collect();
    
    // Look for the character in the SUBS_I vector
    // If found, return the corresponding character from SUBS_O
    for (i, &accent_char) in subs_i.iter().enumerate() {
        if c_lower == accent_char {
            return subs_o[i];
        }
    }
    
    // If not found in the substitution tables, return a dash
    '-'
}

/// Convert a string to a slug
/// This function applies all the slugification rules:
/// - Convert all characters using the conv() function
/// - Prevent consecutive dashes
/// - Remove trailing dashes unless it's the only character
fn slugify(s: &str) -> String {
    // Handle empty strings by returning an empty string
    if s.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    let mut last_was_dash = false;
    
    // Process each character in the input string
    for c in s.chars() {
        let converted = conv(c);
        
        // Handle consecutive dashes - we only keep the first one
        if converted == '-' {
            if last_was_dash {
                // Skip this dash if the previous character was also a dash
                continue;
            }
            last_was_dash = true;
        } else {
            last_was_dash = false;
        }
        
        // Add the converted character to the result
        result.push(converted);
    }
    
    // Remove trailing dash if it's not the only character
    // This implements the rule that a final dash is not allowed
    // unless it's the only character in the slug
    if result.len() > 1 && result.ends_with('-') {
        result.pop();
    }
    
    result
}

/// Command line arguments
/// This struct defines the parameters that can be passed to the program
#[derive(Parser, Debug)]
struct Args {
    /// Input string to convert to slug
    slug_in: String,

    /// Number of times to repeat the slug
    #[arg(long, default_value_t = 1)]
    repeat: usize,

    /// Enable verbose output
    #[arg(long, default_value_t = false)]
    verbose: bool,
    
    /// Multiple words (alternative way to read input as vector)
    /// This shows how to read multiple words into a vector
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    words: Vec<String>,
}

/// Main function - entry point of the program
fn main() {
    // Parse command line arguments using the Args struct
    let args = Args::parse();
    
    // If verbose mode is enabled, print additional information
    if args.verbose {
        println!("Input: \"{}\"", args.slug_in);
        
        if !args.words.is_empty() {
            println!("Words: {:?}", args.words);
        }
    }
    
    // Convert the input string to a slug
    let slug = slugify(&args.slug_in);
    
    // Print the slug the requested number of times
    for _ in 0..args.repeat {
        println!("slug: {}", slug);
    }
    
    // Process additional words if provided
    if !args.words.is_empty() {
        println!("\nProcessing words as vector:");
        for word in &args.words {
            println!("word: {} -> slug: {}", word, slugify(word));
        }
    }
}

// Explanation of the differences between 'for el in v' and 'for &el in v':
//
// 1. `for el in v`: Here, `el` is of type `&u32` (a reference to u32).
//    When we iterate over a slice with `for el in v`, we get references to
//    each element, not the elements themselves.
//
// 2. `for &el in v`: Here, `el` is of type `u32` (the actual value).
//    This is pattern matching that destructures/dereferences the reference
//    during iteration.
//
// The difference is in how we access the elements:
// - With `for el in v`, to use the value we need to dereference it with `*el`.
// - With `for &el in v`, we get the value directly as `el`.
//
// The operation happening in `&el` is pattern matching. It's saying "match a
// reference and bind the dereferenced value to `el`".
//
// Examples:
// ```
// let v = [1, 2, 3];
// for el in &v {
//     // Here el is of type &i32
//     println!("{}", *el); // Need to dereference
// }
//
// for &el in &v {
//     // Here el is of type i32
//     println!("{}", el);  // Already a value
// }
// ```
//
// This explains why in our `is_pangram` function, we use `|&count|` in the
// closure - we want to work with the actual values, not references.

/// Test module for unit testing the slugify functionality
/// This module contains all the test cases to verify that our implementation
/// behaves correctly according to the requirements
#[cfg(test)]
mod tests {
    // Import all items from the parent module to make them available in tests
    use super::*;
    
    /// Test conversion of accented characters
    #[test]
    fn test_conv_accented() {
        assert_eq!(conv('à'), 'a');
        assert_eq!(conv('é'), 'e');
        assert_eq!(conv('ö'), 'o');
    }
    
    /// Test conversion of non-accented characters
    #[test]
    fn test_conv_non_accented() {
        assert_eq!(conv('a'), 'a');
        assert_eq!(conv('z'), 'z');
        assert_eq!(conv('0'), '0');
        assert_eq!(conv('9'), '9');
    }
    
    /// Test conversion of non-allowed characters
    #[test]
    fn test_conv_non_allowed_unknown() {
        assert_eq!(conv('!'), '-');
        assert_eq!(conv('@'), '-');
        assert_eq!(conv(' '), '-');
    }
    
    /// Test conversion of accented characters not in our substitution list
    #[test]
    fn test_conv_accented_not_in_list() {
        // Greek omega character not in our substitution list
        assert_eq!(conv('ῶ'), '-');
    }
    
    /// Test slugify with multiple words
    #[test]
    fn test_slugify_multiple_words() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }
    
    /// Test slugify with accented characters
    #[test]
    fn test_slugify_accented_chars() {
        assert_eq!(slugify("Café Olé"), "cafe-ole");
    }
    
    /// Test slugify with an empty string
    #[test]
    fn test_slugify_empty_string() {
        assert_eq!(slugify(""), "");
    }
    
    /// Test slugify with consecutive spaces
    #[test]
    fn test_slugify_consecutive_spaces() {
        assert_eq!(slugify("Hello    World"), "hello-world");
    }
    
    /// Test slugify with consecutive invalid characters
    #[test]
    fn test_slugify_consecutive_invalid_chars() {
        assert_eq!(slugify("Hello!!!World"), "hello-world");
    }
    
    /// Test slugify with only invalid characters
    #[test]
    fn test_slugify_only_invalid_chars() {
        assert_eq!(slugify("!@#$%^&*()"), "-");
    }
    
    /// Test slugify with trailing space
    #[test]
    fn test_slugify_trailing_space() {
        assert_eq!(slugify("Hello World "), "hello-world");
    }
    
    /// Test slugify with trailing invalid characters
    #[test]
    fn test_slugify_trailing_invalid_chars() {
        assert_eq!(slugify("Hello World!!!!"), "hello-world");
    }
}
