// This program checks if a text file contains a pangram (a string that contains all letters of the alphabet)
// and counts the frequency of each letter in the text.

// The stats function counts how many times each letter (a-z) appears in the text
// It returns an array of 26 elements where each index corresponds to a letter (0=a, 1=b, etc.)
fn stats(text: &str) -> [u32; 26] {
    // Initialize an array with 26 zeros to count each letter
    let mut counts = [0; 26];
    
    // Iterate through each character in the text
    for c in text.chars() {
        // Only process alphabetic characters
        if c.is_alphabetic() {
            // Convert character to lowercase and calculate its index (0-25)
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            // Ensure the character is a valid English letter
            if index < 26 {
                // Increment the counter for this letter
                counts[index] += 1;
            }
        }
    }
    counts
}

// The is_pangram function checks if all 26 letters of the alphabet appear at least once
fn is_pangram(counts: &[u32]) -> bool {
    // A pangram must have exactly 26 letters, and each count must be greater than 0
    counts.len() == 26 && counts.iter().all(|&count| count > 0)
}

// This function handles file I/O and displays results
pub fn run_pangram() {
    use std::env;
    use std::fs;

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    // Get the filename from arguments
    let filename = &args[1];
    // Read the file contents
    let contents = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Count the frequency of each letter in the text
    let counts = stats(&contents);
    // Check if the text is a pangram
    if is_pangram(&counts) {
        println!("\"{}\" is a pangram", contents.trim());
    } else {
        println!("\"{}\" is not a pangram", contents.trim());
    }

    // Print the frequency of each letter
    for (i, count) in counts.iter().enumerate() {
        let c = (b'a' + i as u8) as char; // Convert index to corresponding letter
        println!("{} {}", c, count);
    }
}

// The code below contains test functions to verify program correctness

#[cfg(test)] // This annotation marks the code below as test-only
mod tests
{   
    // Import the functions from the parent module
    use super::*;
    
    // Test if a pangram is correctly identified when all letters appear exactly once
    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    // Test if a non-pangram is correctly identified when some letters are missing
    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }
    
    // Test if a pangram is correctly identified with varying letter frequencies
    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    // Test if an undersized array is correctly identified as not a pangram
    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }    
    
    // Test if stats correctly counts each letter in the full alphabet
    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    // Test if stats correctly handles an empty string
    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    // Test if stats correctly identifies a missing letter
    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);
    }

    // Test if stats correctly counts letters in a full sentence
    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    // Test if stats correctly handles punctuation by ignoring it
    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    // Test if stats correctly identifies a specific missing letter
    #[test] 
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 { // 'y' is missing
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
        }
    }

    // Test the complete pangram checking functionality
    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

// Program entry point
fn main() {
    // Call the function that handles file I/O and displays results
    run_pangram();
}