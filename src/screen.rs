//! Interfaces with the rest of the library. Each intrface can be compiled
//! independently. Supported Interfaces:
//!     - stdout, stdin
//!     - readline

#[cfg(feature = "pretty")]
extern crate rustyline;

use error::MaeveError;
#[cfg(feature = "pretty")]
use self::rustyline::Editor;
#[cfg(feature = "pretty")]
use std::fs::OpenOptions;
#[cfg(feature = "pretty")]
use std::path::Path;

#[cfg(not(feature = "pretty"))]
use std;

pub trait Interfaceable {
    fn new() -> Self;
    fn print(&self, &str);
    fn prompt(&mut self) -> Result<String, MaeveError>;
    fn confirm(&mut self, string: &str) -> Result<bool, MaeveError> {
        self.print(&format!(
            "{}:\
             \n\t1 - Yes\
             \n\t2 - No",
            string
        ));
        loop {
            match self.prompt()?.parse() {
                Ok(1) => return Ok(true),
                Ok(2) => return Ok(false),
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
    fn confirm_history() -> Result<(), MaeveError> {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(Path::new(".history.txt"))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(MaeveError::Write(err)),
        }
    }
}

#[cfg(feature = "pretty")]
impl Interfaceable for PrettyPrompt {
    fn new() -> PrettyPrompt {
        let mut editor = Editor::<()>::new();
        let history = match (
            PrettyPrompt::confirm_history(),
            editor.load_history(".history.txt"),
        ) {
            (Ok(_), Ok(_)) => true,
            _ => false,
        };
        return PrettyPrompt { editor, history };
    }

    fn print(&self, string: &str) {
        println!("{}", string);
    }

    fn prompt(&mut self) -> Result<String, MaeveError> {
        let readline = match self.editor.readline(">> ") {
            Ok(line) => {
                if self.history {
                    self.editor.add_history_entry(&line);
                }
                line
            }
            Err(_) => String::from("quit"),
        };
        match (
            PrettyPrompt::confirm_history(),
            self.editor.save_history(".history.txt"),
        ) {
            (Ok(_), Ok(_)) => (),
            _ => return Err(MaeveError::WriteHistory),
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

    fn prompt(&mut self) -> Result<String, MaeveError> {
        let mut choice = String::new();
        if let Err(err) = std::io::stdin().read_line(&mut choice) {
            return Err(MaeveError::Io(err));
        };
        return Ok(String::from(choice.trim()));
    }
}

#[cfg(not(feature = "pretty"))]
pub type Screen = BasicPrompt;

#[cfg(feature = "pretty")]
pub type Screen = PrettyPrompt;
