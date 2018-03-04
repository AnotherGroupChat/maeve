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
use maeve::screen::Interfaceable;
use maeve::screen::Screen;

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
            match evaluate(&mut src, &mut game.clone()) {
                Ok(()) | Err(MaeveError::Exit) => src.print("Goodbye!"),
                Err(err) => src.print(&format!("Runtime error: {}", &err)),
            }
        }
        Err(err) => src.print(&format!("Exit: {}", &err)),
    }
}
