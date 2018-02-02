//! Operations to manage loaded protos.

use std::fs::File;
use std::path::Path;
use protos::master::Game;
use screen::Interfaceable;
use protobuf::CodedOutputStream;
use protobuf::Message;
use io::write_protobuf;

#[allow(unused_mut)]
pub fn load<I: Interfaceable>(
    src: &mut I,
    mut game: Game,
) -> Result<Game, String> {
    src.print("I see you've been a guest with us before.");
    src.print(&format!("Welcome back {}.", game.get_name()));
    return Ok(game);
}

pub fn new<I: Interfaceable>(
    src: &mut I,
    mut game: Game,
) -> Result<Game, String> {
    src.print("Welcome to Maeve, the hosts are here to serve you.");
    src.print("What is your name?");

    let name = src.prompt();
    game.set_name(name.clone());
    return write_protobuf(src, game);
}
