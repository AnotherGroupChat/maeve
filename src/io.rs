//! Operations that interact with the file system.

use load::save;
use std::fs::File;
use std::path::Path;
use protobuf::core::MessageStatic;
use protobuf::CodedOutputStream;
use protobuf::Message; 
use protos::master::Game;
use screen::Interfaceable;

pub fn extract_protobuf<F, M: MessageStatic, I: Interfaceable>(
    src: &mut I,
    path: &str,
    callback: F,
) -> Result<M, String>
where
    F: Fn(&mut I, M) -> Result<M, String>,
{
    return match File::open(&Path::new(path)) {
        Ok(mut is) => match ::protobuf::parse_from_reader::<M>(&mut is) {
            Ok(t) => callback(src, t),
            Err(_) => Err(String::from("Failed to load file.")),
        },
        Err(_) => Err(String::from("Failed to open file")),
    };
}

pub fn write_protobuf<I: Interfaceable>(
    src: &mut I,
    game: Game,
) -> Result<Game, String> {
    let mut file = save(src, &game);
    let mut cos = CodedOutputStream::new(&mut file);
    game.write_to(&mut cos).unwrap();
    cos.flush().unwrap();
    return Ok(game);
}

pub fn prompt_path<F, M: MessageStatic, I: Interfaceable>(
    src: &mut I,
    callback: F,
) -> Result<M, String>
where
    F: Fn(&mut I, M) -> Result<M, String>,
{
    src.print("Please provide the name of save file you'd like to load:");
    let choice = src.prompt();

    return extract_protobuf(src, &choice, callback);
}
