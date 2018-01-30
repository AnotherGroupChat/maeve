//! An iterator is created by the split_whitespace method on str.

use std::str;

// Type alias defined for an existing type.
type TokenIterator<'a> = str::SplitWhitespace<'a>;

pub fn tokenize(text: &str) -> TokenIterator {
    return text.split_whitespace();
}
