//! A file that holds logical operations in order to run the game.

use error::MaeveError;
use interpreter::machine;
use interpreter::machine::create_machine;
use interpreter::machine::Machine;
use interpreter::parser;
use interpreter::tokenize::tokenize;
use protos::master::Game;
use screen::Interfaceable;

pub fn evaluate<I: Interfaceable>(
    src: &mut I,
    mut game: Game,
) -> Result<(), MaeveError> {
    let parsers: [&Fn(
        &Machine<I>,
        &Vec<String>,
    ) -> Result<Option<machine::Action>, MaeveError>; 4] = [
        &parser::builtin,
        &parser::item,
        &parser::level,
        &parser::undefined,
    ];

    let mut description = String::new();
    loop {
        if !description.is_empty() {
            src.print(&description);
        }

        let tokens = src.prompt()?;
        let tokens = &tokenize(&tokens);

        // TODO(45): Put a mutex on threads and pull the newest game from a
        // channel. For example: mut game = src.sync();

        let mut machine = *create_machine(src, &mut game)?;

        for parse in parsers.iter() {
            if let Some(action) = parse(&machine, tokens)? {
                description = machine.process_action(action)?.clone();
                break;
            };
        }
        // Poor man's debug. It hurts me more than you know.
        // println!("{:?}", machine.game);
        // TODO(45): Broadcast game to the rest of the threads. For example:
        // src.broadcast(game)
    }
}
