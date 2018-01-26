//! An iterator is created by the split_whitespace method on str.

use std::str;

type TokenIterator<'a> = str::SplitWhitespace<'a>; // Type alias defined for an existing type.

pub fn tokenize(text: &str) -> TokenIterator {
    return text.split_whitespace();
}
