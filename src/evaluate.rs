//! A file that holds logical operations in order to run the game.

use interpreter::token::tokenize;
use protos::master::Game;
use screen::Interfaceable;

pub fn evaluate<I: Interfaceable>(src: &mut I, mut _game: Game) -> () {
    let token_string = src.prompt();
    let _tokens = tokenize(&token_string);
}
