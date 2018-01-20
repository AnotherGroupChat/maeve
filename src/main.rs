#[macro_use]
extern crate clap;
extern crate protobuf;
use clap::App;

mod protos;
use protos::master::Game;
mod io;
use io::*;
mod load;
use load::*;

fn prompt() -> Result<Game, String> {
    loop {
        println!("Please select an option:");
        println!("1 - New Game");
        println!("2 - Load Game");
        println!("3 - Exit Game");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => return prompt_path(new),
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
