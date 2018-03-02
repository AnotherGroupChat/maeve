//! A file that holds logical operations in order to run the game.

use error::MaeveError;
use interpreter::tokenize::tokenize;
use interpreter::machine;
use interpreter::machine::Machine;
use interpreter::parser;
use protos::master::Game;
use screen::Interfaceable;

pub fn evaluate<I: Interfaceable>(
    mut src: &mut I,
    game: &mut Game,
) -> Result<(), MaeveError> {

    let parsers: [&Fn(&Machine<I>, &Vec<String>) -> Result<Option<machine::Action>, MaeveError>;
                     4] = [&parser::builtin, &parser::item, &parser::level, &parser::undefined];

    let mut game = &mut game.clone();
    loop {
        let tokens = src.prompt()?;
        let tokens = &tokenize(&tokens);

        // TODO(45): Put a mutex on threads and pull the newest game from a
        // channel. For example: mut game = src.sync();
        let machine : &mut Machine<I> = &mut Machine::new(&mut src, &mut game);

        for parse in parsers.iter() {
            if let Some(action) = parse(&machine, tokens)? {
                machine.process_action(action)?;
                break;
            };
        }

        // Poor man's debug. It hurts me more than you know.
        // println!("{:?}", machine.game);
        // TODO(45): Broadcast game to the rest of the threads. For example:
        // src.broadcast(game)
    }
}
