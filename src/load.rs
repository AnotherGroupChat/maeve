use ::std;
use ::std::fs::File;
use ::std::path::Path;

use protos::master::Game;


#[allow(unused_mut)]
pub fn load(mut game: Game) -> Result<Game, String> {
    println!("I see you've been a guest with us before.");
    println!("Welcome back {}.", game.get_name().trim());
    return Ok(game);
}

pub fn new(mut game: Game) -> Result<Game, String> {
    println!("Welcome to Maeve, the hosts are here to serve you.");
    println!("What is your name?");

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .unwrap();

    game.set_name(name);

    let mut path = String::new();
    println!("Hello {}, please enter the name of the new save file:", game.get_name().trim());
    std::io::stdin()
        .read_line(&mut path)
        .unwrap();

    let path = path.trim();
    File::create(&Path::new(&path)).unwrap();

    return Ok(game);
}
