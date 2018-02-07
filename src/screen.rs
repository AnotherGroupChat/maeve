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
    fn prompt(&mut self) -> Result<String, String>;
    fn confirm(&mut self, string: &str) -> bool {
        self.print(&format!(
            "{}:\
             \n\t1 - Yes\
             \n\t2 - No",
            string
        ));
        loop {
            match self.prompt().unwrap().parse() {
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
impl PrettyPrompt {
    fn confirm_history() -> Result<(), String> {
        match Path::new(".history.txt").exists() {
            false => match File::create(Path::new(".history.txt")) {
                Ok(_) => Ok(()),
                Err(_) => Err(String::from("Error confirming PrettyPrompt history")),
            },
            true => Ok(()),
        }
    }
}

#[cfg(feature = "pretty")]
impl Interfaceable for PrettyPrompt {
    fn new() -> PrettyPrompt {
        let mut editor = Editor::<()>::new();
        let mut history = false;
        if let Ok(_) = PrettyPrompt::confirm_history() {
                if let Ok(_) = editor.load_history(".history.txt") {
                    history = true
                }
        };
        return PrettyPrompt { editor, history };
    }

    fn print(&self, string: &str) {
        println!("{}", string);
    }

    fn prompt(&mut self) -> Result<String, String> {
        let readline = match self.editor.readline(">> ") {
            Ok(line) => {
                if self.history {
                    self.editor.add_history_entry(&line);
                }
                line
            }
            Err(_) => String::from("quit"),
        };
        match PrettyPrompt::confirm_history() {
            Ok(_) => {
                match self.editor.save_history(".history.txt") {
                    Ok(_) => (),
                    Err(_) => return Err(String::from("Error writing .history.txt.")),
                };
            },
            Err(err) => return Err(err),
        }
        return Ok(readline);
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

    fn prompt(&mut self) -> Result<String, String> {
        let mut choice = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut choice) {
            return Err(String::from("BasicPrompt readline error"));
        };
        return Ok(String::from(choice.trim()));
    }
}

#[cfg(not(feature = "pretty"))]
pub type Screen = BasicPrompt;

#[cfg(feature = "pretty")]
pub type Screen = PrettyPrompt;

