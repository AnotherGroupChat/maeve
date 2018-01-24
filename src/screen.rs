//! Interfaces with the rest of the library. Each intrface can be compiled
//! independently. Supported Interfaces:
//!     - stdout, stdin
//!     - readline

#[cfg(feature = "pretty")]
extern crate rustyline;
#[cfg(feature = "pretty")]
use self::rustyline::Editor;

#[cfg(not(feature = "pretty"))]
use std;

pub trait Interfaceable {
    fn new() -> Self;
    fn print(&self, &str);
    fn prompt(&mut self) -> String;
}

#[cfg(feature = "pretty")]
pub struct PrettyPrompt {
    editor: Editor<()>,
    history: bool,
}

#[cfg(feature = "pretty")]
impl Interfaceable for PrettyPrompt {
    fn new() -> PrettyPrompt {
        let mut rl = Editor::<()>::new();
        let history = match rl.load_history(".history.txt") {
            Err(_) => false,
            _ => true,
        };
        return PrettyPrompt {
            editor: rl,
            history: history,
        };
    }

    fn print(&self, string: &str) {
        println!("{}", string);
    }

    fn prompt(&mut self) -> String {
        let readline = self.editor.readline(">> ");
        return match readline {
            Ok(line) => {
                if self.history {
                    self.editor.add_history_entry(&line);
                }
                return line;
            }
            Err(_) => String::from("quit"),
        };
    }
}

#[cfg(not(feature = "pretty"))]
pub struct BasicPrompt {}

#[cfg(not(feature = "pretty"))]
impl Interfaceable for BasicPrompt {
    fn new() -> BasicPrompt {
        return BasicPrompt {};
    }

    fn print(&self, string: &str) {
        println!("{}", &string);
    }

    fn prompt(&mut self) -> String {
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");
        return String::from(choice.trim());
    }
}

#[cfg(not(feature = "pretty"))]
pub type Screen = BasicPrompt;

#[cfg(feature = "pretty")]
pub type Screen = PrettyPrompt;
