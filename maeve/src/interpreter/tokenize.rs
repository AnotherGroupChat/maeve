//! An iterator is created by the split_whitespace method on str.

use std::str;

pub fn tokenize(text: &str) -> Vec<String> {
    return text
        .split_whitespace()
        .filter_map(|word| Some(String::from(word).to_lowercase()))
        .collect();
}
