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

const NEW_GAME_PATH: &str = "games/game_design.pb";
const GAME_FILE_PATH: &str = "games/";

#[allow(unused_mut)]
fn load(mut game: Game) -> Result<Game, String> {
    println!("I see you've been a guest with us before.");
    println!("Welcome back {}.", game.get_name().trim());
    return Ok(game);
}

fn new(mut game: Game) -> Result<Game, String> {
    println!("Welcome to Maeve, the hosts are here to serve you.");
    println!("What is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .unwrap();

    game.set_name(name);

    let mut path = String::new();
    println!("Hello {}, please enter the name of the new save file:", game.get_name().trim());
    io::stdin()
        .read_line(&mut path)
        .unwrap();

    let path = format!("{}{}", GAME_FILE_PATH, path.trim());
    File::create(&Path::new(&path)).unwrap();

    return Ok(game);
}

fn extract_protobuf<F, M: MessageStatic>(path: &str, callback: F) -> Result<M, String>
where
    F: Fn(M) -> Result<M, String>,
{
    return match File::open(&Path::new(path)) {
        Ok(mut is) => match protobuf::parse_from_reader::<M>(&mut is) {
            Ok(t) => callback(t),
            Err(_) => Err(String::from("Failed to load file.")),
        },
        Err(_) => Err(String::from("Failed to open file")),
    };
}

fn prompt_path<F, M: MessageStatic>(callback: F) -> Result<M, String>
where
    F: Fn(M) -> Result<M, String>,
{
    println!("Please provide the name of your save file:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");
    let choice = format!("{}{}", GAME_FILE_PATH, choice.trim());

    return extract_protobuf(&choice, callback);
}

fn prompt() -> Result<Game, String> {
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
            1 => return extract_protobuf(NEW_GAME_PATH, new),
            2 => return prompt_path(load),
            3 => return Err(String::from("We look forward to your next visit.")),
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

    #[allow(unused_variables)]
    match result {
        Ok(game) => {
            println!("And the games begin!"); // Do something with the games here.
            //Call the interpreter
            //derpreter(game);
        },
        Err(error) => println!("Exit: {}", &error),
    }
}
