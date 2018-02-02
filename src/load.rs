//! Operations to manage loaded protos.

use io::write_protobuf;
use protos::master::Game;
use screen::Interfaceable;
use std::fs::File;
use std::path::Path;

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

pub fn save<I: Interfaceable>(src: &mut I, game: &Game) -> File {
    let mut path;
    let file;
    loop {
        src.print(&format!(
            "Hello {}, please enter the name of the new save file:",
            game.get_name()
        ));

        path = src.prompt();

        if Path::new(&path).exists() {
            src.confirm();
            match src.prompt().parse() {
                Ok(1) => {
                    file = File::create(&Path::new(&path)).unwrap();
                    break;
                }
                _ => {
                    src.print("Ok let's try again then...");
                    continue;
                }
            };
        } else {
            file = File::create(&Path::new(&path)).unwrap();
            break;
        }
    }
    return file;
}
