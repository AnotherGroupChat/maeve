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

        // Attempt verb search first
        for fuzzed_verb in fuzz(tokens) {

        }

        // Attempt item search
        for fuzzed_item in fuzz(tokens) {
            // Check for associated verbs
            for fuzzed_verb in fuzz(remainder) {
                // If match attempt to run
            }
        }
    }
}
