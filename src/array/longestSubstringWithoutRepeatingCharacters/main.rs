use clap::Parser;
use std::string::String;
use anyhow::Result;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Define a struct to hold the command-line arguments
#[derive(Debug, Parser)]
struct Args {
    // Add your command-line arguments here:
    input: String,
}

fn init_tracing() {
    // Only run once; protects against multiple initialization attempts
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env()) // respects RUST_LOG
            .with(fmt::layer())                  // pretty logging
            .init();                             // installs as global default
    });
}

fn main() -> Result<()> {
    init_tracing();
    let args = Args::parse();
    
    let value = longest_substring_without_repeating_characters(args.input.as_str())?;
    info!("Longest substring without repeating characters: {}", value);

    Ok(())
}

/*
    3. Longest Substring Without Repeating Characters
    Solved
    Medium
    Topics
    conpanies iconCompanies
    Hint

    Given a string s, find the length of the longest

    without duplicate characters.

    

    Example 1:

    Input: s = "abcabcbb"
    Output: 3
    Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.

    Example 2:

    Input: s = "bbbbb"
    Output: 1
    Explanation: The answer is "b", with the length of 1.

    Example 3:

    Input: s = "pwwkew"
    Output: 3
    Explanation: The answer is "wke", with the length of 3.
    Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

    

    Constraints:

        0 <= s.length <= 5 * 10^4
        s consists of English letters, digits, symbols and spaces.

 */

fn longest_substring_without_repeating_characters(input: &str) -> Result<u16> {
    let mut seen = std::collections::HashSet::<char>::new();
    let characters: Vec<char> = input.chars().collect();

    let mut max_length  = 0u16;
    let mut window_start= 0usize;
    let mut window_end  = 0usize;
    
    while window_end < characters.len(){
        // Condition 1: the current character is already in our window
        while seen.contains(&characters[window_end]){
            // remove characters from the front of the window until no longer so.
            seen.remove(&characters[window_start]);
            window_start = window_start + 1;
        }

        // Condition 2: the current character is not already in our window
        // Add the current character to the window, and compute longest window length.
        seen.insert(characters[window_end]); 
        window_end = window_end + 1;
        max_length = u16::max(max_length,(window_end - window_start) as u16);
    }

    Ok(max_length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case("", 0)]
    fn test_longest_substring_without_repeating_characters(#[case]input: &str, #[case] expected: u16) {
        assert_eq!(expected, longest_substring_without_repeating_characters(input).unwrap());
    }
}