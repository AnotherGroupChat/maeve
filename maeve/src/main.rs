//! Entry point for commandline interaction with maeve interpreter.

#[macro_use]
extern crate clap;
extern crate maeve;
extern crate prost;

use clap::App;
use maeve::error::MaeveError;
use maeve::evaluate::evaluate;
use maeve::io::extract_protobuf;
use maeve::io::prompt_path;
use maeve::load::load;
use maeve::load::new;
use maeve::protos::master::Game;
use maeve::screen::Constructable;
use maeve::screen::Interfaceable;
use maeve::screen::Screen;
#[cfg(feature = "threaded")]
use maeve::screen::Spawnable;
use std::thread;

fn menu<I: Interfaceable>(src: &mut I) -> Result<Game, MaeveError> {
    loop {
        src.print(
            "Please select an option:\
             \n\t1 - New Game\
             \n\t2 - Load Game\
             \n\t3 - Exit Game",
        );

        match src.prompt()?.parse() {
            Ok(1) => return prompt_path(src, new),
            Ok(2) => return prompt_path(src, load),
            Ok(3) => return Err(MaeveError::Exit),
            _ => println!("That is not how this works, choose again."),
        }
    }
}

fn main() {
    let mut src = Screen::new();
    src.print("Welcome to Maeve!");

    let yaml = load_yaml!("app.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let (load_game, new_game) =
        (matches.value_of("load"), matches.value_of("new"));
    let result = match (load_game, new_game) {
        (Some(path), _) => extract_protobuf(&mut src, path, load),
        (_, Some(path)) => extract_protobuf(&mut src, path, new),
        (_, _) => menu(&mut src),
    };

    match result {
        Ok(game) => {
            src.print("And the games begin!");
            let mut threads = vec![];
            // Make this backwards compat
            loop {
                match src.get_endpoint() {
                    Ok(mut endpoint) => {
                        let mut endpoint = *endpoint;
                        // Next would be associating a client with a character
                        // last would be syncing all clients' games.

                        // clone arc here instead
                        let game_ref = game.clone();
                        threads.push(thread::spawn(move || {
                            match evaluate(&mut endpoint, game_ref) {
                                Ok(()) | Err(MaeveError::Exit) => {
                                    endpoint.print("Goodbye!")
                                }
                                Err(err) => endpoint
                                    .print(&format!("Runtime error: {}", &err)),
                            }
                        }))
                    }
                    Err(err) => match err {
                        MaeveError::Exit => {
                            src.print(&format!("Exit: {}", &err));
                            break;
                        }
                        _ => {
                            src.print(&format!(
                                "Client failed to start: {}",
                                &err
                            ));
                        }
                    },
                }
            }
            for thread in threads {
                // Wait for the thread to finish. Returns a result.
                let _ = thread.join();
            }
        }
        Err(err) => src.print(&format!("Exit: {}", &err)),
    }
}
