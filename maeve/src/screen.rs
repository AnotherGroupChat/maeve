//! Interfaces with the rest of the library. Each intrface can be compiled
//! independently. Supported Interfaces:
//!     - stdout, stdin
//!     - readline

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
extern crate rustyline;

#[cfg(feature = "threaded")]
extern crate ctrlc;

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
use self::rustyline::Editor;
use error::MaeveError;
#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
use std::fs::OpenOptions;
#[cfg(any(feature = "threaded", feature = "pretty"))]
use std::path::Path;

#[cfg(feature = "threaded")]
use std::sync::mpsc::{channel, Receiver};
#[cfg(feature = "threaded")]
use std::time::Duration;

use std::fs;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

#[cfg(feature = "stdout")]
use std;

pub trait Interfaceable {
    fn print(&mut self, &str);
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
pub trait Constructable {
    fn new() -> Self;
}

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
pub struct PrettyPrompt {
    editor: Editor<()>,
    history: bool,
}

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
impl PrettyPrompt {
    fn confirm_history() -> Result<(), MaeveError> {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(Path::new(".history.txt"))
        {
            Ok(_) => Ok(()),
            Err(_) => Err(MaeveError::Write),
        }
    }
}

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
impl Constructable for PrettyPrompt {
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
}
#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
impl Interfaceable for PrettyPrompt {
    fn print(&mut self, string: &str) {
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

#[cfg(feature = "stdout")]
pub struct BasicPrompt {}

#[cfg(feature = "stdout")]
impl Constructable for BasicPrompt {
    fn new() -> BasicPrompt {
        return BasicPrompt {};
    }
}
#[cfg(feature = "stdout")]
impl Interfaceable for BasicPrompt {
    fn print(&mut self, string: &str) {
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

// We define an output type to let us configure other interfaces.
#[cfg(feature = "stdout")]
pub type Output = BasicPrompt;

#[cfg(all(not(feature = "stdout"), feature = "pretty"))]
pub type Output = PrettyPrompt;

#[cfg(feature = "threaded")]
pub struct ClientPrompt {
    socket: UnixStream,
    switch: Arc<AtomicBool>,
}

#[cfg(feature = "threaded")]
impl Interfaceable for ClientPrompt {
    fn print(&mut self, string: &str) {
        self.socket
            .write_all(string.as_bytes())
            .expect("Could not write to socket.");
    }

    fn prompt(&mut self) -> Result<String, MaeveError> {
        while !self.switch.load(Ordering::SeqCst) {
            let mut buf = String::new();
            self.socket
                .read_to_string(&mut buf);
            if !buf.is_empty() {
                return Ok(buf);
            }
        }
        Ok(String::from("exit"))
    }
}

#[cfg(feature = "threaded")]
pub type Spawn = ClientPrompt;

pub trait Spawnable {
    fn get_endpoint(&mut self) -> Result<Box<Spawn>, MaeveError>;
}

#[cfg(feature = "threaded")]
pub struct ServerPrompt {
    output: Output,
    rx: Receiver<Result<UnixStream, MaeveError>>,
    warned: bool,
    master_switch: Arc<AtomicBool>,
}

#[cfg(feature = "threaded")]
impl Constructable for ServerPrompt {
    fn new() -> ServerPrompt {
        let output = Output::new();
        let listener = Path::new("sockets/socket.maeve");
        if listener.exists() {
            fs::remove_file(&listener).unwrap();
        }
        let listener =
            UnixListener::bind(listener).expect("Error binding sockets.");
        let (tx, rx) = channel();

        let _ = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        /* connection succeeded */
                        tx.send(Ok(stream)).expect("Channel error.");
                    }
                    Err(err) => {
                        /* connection failed */
                        tx.send(Err(MaeveError::Io(err)))
                            .expect("Channel error.");
                    }
                }
            }
        });

        // Also listen for Ctrl-C being pressed to end the server.
        let master_switch = Arc::new(AtomicBool::new(false));
        let switch = master_switch.clone();
        ctrlc::set_handler(move || {
            switch.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        return ServerPrompt {
            output,
            rx: rx,
            warned: false,
            master_switch,
        };
    }
}

#[cfg(feature = "threaded")]
impl Interfaceable for ServerPrompt {
    fn print(&mut self, string: &str) {
        self.output.print(&string)
    }

    fn prompt(&mut self) -> Result<String, MaeveError> {
        self.output.prompt()
    }
}
#[cfg(feature = "threaded")]
impl Spawnable for ServerPrompt {
    fn get_endpoint(&mut self) -> Result<Box<Spawn>, MaeveError> {
        if !self.warned {
            self.warned = true;
            self.output
                .print("Press Ctrl-C to stop listening for clients.");
        }
        // This is a simple loop, but you may want to use more complex
        // logic here, for example to handle I/O.
        while !self.master_switch.load(Ordering::SeqCst) {
            if let Ok(result) = self.rx.recv_timeout(Duration::from_millis(100))
            {
                match result {
                    Ok(stream) => {
                        // connection succeeded
                        stream
                            .set_read_timeout(Some(Duration::new(0, 250000)))
                            .expect("Couldn't set read timeout");

                        let client = ClientPrompt {
                            socket: stream,
                            switch: self.master_switch.clone(),
                        };
                        // Do logging somehow
                        // self.output.print("New File");
                        return Ok(Box::new(client));
                    }
                    Err(err) => {
                        /* connection failed */
                        return Err(err);
                    }
                }
            }
        }
        return Err(MaeveError::Exit);
    }
}

#[cfg(not(feature = "threaded"))]
pub type Screen = Output;
#[cfg(feature = "threaded")]
pub type Screen = ServerPrompt;
