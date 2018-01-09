#[macro_use]
extern crate clap;
extern crate protobuf;
use clap::App;
use std::io;
use std::fs::File;
use std::path::Path;

mod protos;
use protos::master::Game;
use protobuf::core::MessageStatic;

fn load(game: Game) -> Result<Game, &'static str> {
    println!("I see you've been a guest with us before.");
    // Todo some logic for loading/validating games here.
    return Ok(game);
}

fn new(game: Game) -> Result<Game, &'static str> {
    println!("Welcome to Maeve, the hosts are here to serve you.");
    // Todo some logic for starting/validating games here.
    return Ok(game);
}

fn extract_protobuf<F, M: MessageStatic>(path: &str, callback: F) -> Result<M, &'static str>
where
    F: Fn(M) -> Result<M, &'static str>,
{
    return match File::open(&Path::new(path)) {
        Ok(mut is) => match protobuf::parse_from_reader::<M>(&mut is) {
            Ok(t) => callback(t),
            Err(_) => Err("Failed to load file."),
        },
        Err(_) => Err("Failed to open file"),
    };
}

fn prompt_path<F, M: MessageStatic>(callback: F) -> Result<M, &'static str>
where
    F: Fn(M) -> Result<M, &'static str>,
{
    println!("Please provide the path to the game:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");

    return extract_protobuf(&choice.trim(), callback);
}

fn prompt() -> Result<Game, &'static str> {
    loop {
        println!("Please select an option:");
        println!("1 - New Game");
        println!("2 - Load Game");
        println!("3 - Exit Game");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => return prompt_path(new),
            2 => return prompt_path(load),
            3 => return Err("We look forward to your next visit."),
            _ => println!("That is not how this works, choose again."),
        }
    }
}

fn main() {
    println!("Welcome to Maeve!");

    let yaml = load_yaml!("app.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let (load_game, new_game) = (matches.value_of("load"), matches.value_of("new"));
    let result = match (load_game, new_game) {
        (Some(path), _) => extract_protobuf(path, load),
        (_, Some(path)) => extract_protobuf(path, new),
        (_, _) => prompt(),
    };

    match result {
        Ok(game) => println!("And the games begin!"), // Do something with the games here.
        Err(error) => println!("Exit: {}", error),
    }
}
