//! Interfaces with the rest of the library. Each intrface can be compiled
//! independently. Supported Interfaces:
//!     - stdout, stdin
//!     - readline

#[cfg(feature = "pretty")]
extern crate rustyline;
#[cfg(feature = "pretty")]
use self::rustyline::Editor;
#[cfg(feature = "pretty")]
use std::fs::File;
#[cfg(feature = "pretty")]
use std::path::Path;

#[cfg(not(feature = "pretty"))]
use std;

pub trait Interfaceable {
    fn new() -> Self;
    fn print(&self, &str);
    fn prompt(&mut self) -> String;
    fn confirm(&mut self, string: &str) -> bool {
        self.print(&format!(
            "{}:\
             \n\t1 - Yes\
             \n\t2 - No",
            string
        ));
        loop {
            match self.prompt().parse() {
                Ok(1) => return true,
                Ok(2) => return false,
                _ => self.print("Invalid option."),
            }
        }
    }
}

#[cfg(feature = "pretty")]
pub struct PrettyPrompt {
    editor: Editor<()>,
    history: bool,
}

#[cfg(feature = "pretty")]
impl Interfaceable for PrettyPrompt {
    fn new() -> PrettyPrompt {
        let mut editor = Editor::<()>::new();
        let mut history = false;
        if confirm_history() {
            history = match editor.load_history(".history.txt") {
                Err(_) => false,
                _ => true,
            };
        };
        return PrettyPrompt { editor, history };
    }

    fn print(&self, string: &str) {
        println!("{}", string);
    }

    fn prompt(&mut self) -> String {
        let readline = match self.editor.readline(">> "){
            Ok(line) => {
                if self.history {
                    self.editor.add_history_entry(&line);
                }
                return line;
            }
            Err(_) => String::from("quit"),
        };
        if confirm_history() {
            match self.editor.save_history(".history.txt") {
                Ok(_) => (),
                Err(_) => return String::from("Error writing .history.txt."),
            };
        }
        return readline;
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

#[cfg(feature = "pretty")]
fn confirm_history() -> bool {
    if !Path::new(".history.txt").exists() {
        match File::create(Path::new(".history.txt")) {
            Ok(_) => return true,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        };
    }
    return false;
}
