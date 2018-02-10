//! A file that holds logical operations in order to run the game.

use error::MaeveError;
use interpreter::token::tokenize;
use load::save;
use protos::master::Game;
use screen::Interfaceable;

pub fn evaluate<I: Interfaceable>(
    src: &mut I,
    game: &mut Game,
) -> Result<(), MaeveError> {
    loop {
        let token_string = src.prompt()?;
        let tokens = tokenize(&token_string);
        if tokens.len() == 1 {
            match &tokens.first().unwrap_or(&String::from(""))[..] {
                "exit" | "quit" => return Ok(()),
                "save" => save(src, game)?,
                parsed => src.print(parsed),
            };
        }
    }
}
