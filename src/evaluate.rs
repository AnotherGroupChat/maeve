//! A file that holds logical operations in order to run the game.

use error::MaeveError;
use interpreter::tokenize::tokenize;
use interpreter::machine;
use interpreter::machine::Machine;
use interpreter::parser;
use protos::master::Game;
use protos::master::game;
use screen::Interfaceable;
use std::collections::HashMap;

// Clones content on load. Should by rights be actual references (Will take a
// look at shared pointers, and the RC struct). Currenty this implementation
// causes the ugly hacks for getting and settings states cause can' refer to
// these. Ideally refactor and make 101% better.
fn extract_information<'g>(
    game: &'g Game,
    items: &mut HashMap<String, game::Item>,
) -> Result<&'g game::Level, MaeveError> {
    if let Some(ref person) = game.person {
        items.extend(person.inventory.clone());
        if let Some(level) = game.levels.get(&person.level) {
            items.extend(level.items.clone());
            return Ok(level);
        }
        return Err(MaeveError::from("Level for character not found..."));
    }
    return Err(MaeveError::from(
        "A Character was not specifying in the game...",
    ));
}

pub fn evaluate<I: Interfaceable>(
    src: &mut I,
    game: &mut Game,
) -> Result<(), MaeveError> {

    let parsers: [&Fn(&Machine<I>) -> Result<Option<machine::Action>, MaeveError>;
                     4] = [&parser::builtin, &parser::item, &parser::level, &parser::undefined];

    let mut items: HashMap<String, game::Item> = HashMap::new();
    let level = extract_information(&game, &mut items)?;
    let mut game = &mut game.clone();
    loop {
        let token_string = src.prompt()?;

        // TODO(45): Put a mutex on threads and pull the newest game from a
        // channel. For example: mut game = src.sync();
        let mut machine = Machine {
            src: src,
            game: &mut game,
            level: level,
            items: &items,
            tokens: &tokenize(&token_string),
        };

        for parse in parsers.iter() {
            if let Some(action) = parse(&mut machine)? {
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
