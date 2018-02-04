//! A file that holds logical operations in order to run the game.

use interpreter::token::tokenize;
use load::save;
use protos::master::Game;
use screen::Interfaceable;

pub fn evaluate<I: Interfaceable>(
    src: &mut I,
    game: &mut Game,
) -> Result<(), String> {
    loop {
        let token_string = src.prompt();
        let tokens = tokenize(&token_string);
        if tokens.len() == 1 {
            match &tokens.first().unwrap_or(&String::from(""))[..] {
                "exit" | "quit" => return Ok(()),
                "save" => maybe_bail!(save(src, game)),
                parsed => src.print(parsed),
            };
        }
    }
}
