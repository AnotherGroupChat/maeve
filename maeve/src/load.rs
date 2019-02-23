//! Operations to manage loaded protos.

use error::MaeveError;
use io::write_protobuf;
use protos::master::Game;
use screen::Interfaceable;
use std::path::Path;

#[allow(unused_mut)]
pub fn load<I: Interfaceable>(
    src: &mut I,
    mut game: Game,
) -> Result<Game, MaeveError> {
    src.print("I see you've been a guest with us before.");
    src.print(&format!("Welcome back {}.", game.name));
    return Ok(game);
}

pub fn new<I: Interfaceable>(
    src: &mut I,
    mut game: Game,
) -> Result<Game, MaeveError> {
    src.print("Welcome to Maeve, the hosts are here to serve you.");
    src.print("What is your name?");

    let name = src.prompt()?;
    game.name = name;
    save(src, &mut game)?;
    return Ok(game);
}

pub fn save<I: Interfaceable>(
    src: &mut I,
    game: &mut Game,
) -> Result<(), MaeveError> {
    src.print(&format!(
        "Please enter the name of the new save file, \
         or hit enter to use the default location ({}):",
        game.save_path
    ));

    let mut path = src.prompt()?;
    if path == "" {
        path = String::from(game.save_path.to_string());
    }
    game.save_path = path;

    if !Path::new(&game.save_path).exists()
        || src.confirm("Do you want to save over this file?")?
    {
        src.print("Saving...");
        return write_protobuf(&game.save_path, game);
    }
    src.print("File not saved.");
    return Ok(());
}
