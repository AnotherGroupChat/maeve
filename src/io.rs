use std;
use std::fs::File;
use std::path::Path;

use protobuf::core::MessageStatic;


pub fn extract_protobuf<F, M: MessageStatic>(path: &str, callback: F) -> Result<M, String>
where
    F: Fn(M) -> Result<M, String>,
{
    return match File::open(&Path::new(path)) {
        Ok(mut is) => match ::protobuf::parse_from_reader::<M>(&mut is) {
            Ok(t) => callback(t),
            Err(_) => Err(String::from("Failed to load file.")),
        },
        Err(_) => Err(String::from("Failed to open file")),
    };
}

pub fn prompt_path<F, M: MessageStatic>(callback: F) -> Result<M, String>
where
    F: Fn(M) -> Result<M, String>,
{
    println!("Please provide the name of your save file:");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");
    let choice = choice.trim();

    return extract_protobuf(&choice, callback);
}
