use regex::Regex;

struct Replacement {
    re: Regex,
    rep: &'static str,
}

impl Replacement {
    fn new<'r>(re: &'r str, rep: &'static str) -> Self {
        Replacement {
            re: Regex::new(re).unwrap(),
            rep: rep,
        }
    }
}

pub struct Fuzz {
    index: usize,
    permutes: usize,
    token: String,
    remainder: Vec<String>,
}
#[derive(Debug)]
pub struct FuzzItem {
    pub token: String,
    pub remainder: Vec<String>,
}

// Regex for for find and replace, mispellings etc..
// Ideally regex should be a const and not a function. But const allocation
// is still experimental in Rust. Looks like it might be rolled out at some
// point in the future, so keep posted.
lazy_static! {
    static ref REGEXES: [Replacement; 3] = [
        Replacement::new("^n$", "north"),
        Replacement::new("^s$", "south"),
        Replacement::new("ei", "ie")];
}

impl Fuzz {
    pub fn new(baseline: &Vec<String>) -> Self {
        let mut baseline = baseline.clone();
        let token = baseline.pop().unwrap_or(String::from(""));
        Fuzz {
            index: 0,
            permutes: 0,
            token: token,
            remainder: baseline,
        }
    }
}

// Would be better with a generator, However generators are currently only
// experimental in Rust. Currently does a permutation with all possible words.
// I should mention this is a massive hack. A better implementation of this
// would be creating something like a Chomsky Hierarchy and intelligently check
// tokens against the game definition.
impl Iterator for Fuzz {
    type Item = FuzzItem;
    fn next(&mut self) -> Option<Self::Item> {
        let refresh = self.permutes >= REGEXES.len();
        if self.index > self.remainder.len()
            || (self.index == self.remainder.len() && refresh)
        {
            return None;
        }

        if refresh {
            let (a, b) =
                (self.token.clone(), self.remainder[self.index].clone());
            self.remainder[self.index] = a;
            self.token = b;
            self.permutes = 0;
            self.index += 1;
        }

        let mut token = self.token.clone();
        while self.permutes < REGEXES.len() {
            self.permutes += 1;
            if REGEXES[self.permutes - 1].re.is_match(&self.token) {
                token = String::from(
                    REGEXES[self.permutes - 1]
                        .re
                        .replace(&token, REGEXES[self.permutes - 1].rep),
                );
                break;
            }
        }

        return Some(FuzzItem {
            token: token.to_uppercase(),
            remainder: self.remainder.clone(),
        });
    }
}
