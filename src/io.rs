//! Operations that interact with the file system.

use protobuf::CodedOutputStream;
use protobuf::Message;
use protobuf::core::MessageStatic;
use screen::Interfaceable;
use std::fs::File;
use std::path::Path;

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

pub fn write_protobuf<M: Message>(path: &str, msg: &M) -> Result<(), String> {
    let mut file =
        maybe_bail!(File::create(&Path::new(&path)), "Error creating file.");
    let mut cos = CodedOutputStream::new(&mut file);
    maybe_bail!(
        msg.write_to(&mut cos),
        "Error attempting to write save file."
    );
    maybe_bail!(cos.flush(), "Error flushing write buffer.");
    return Ok(());
}

pub fn prompt_path<F, M: MessageStatic, I: Interfaceable>(
    src: &mut I,
    callback: F,
) -> Result<M, String>
where
    F: Fn(&mut I, M) -> Result<M, String>,
{
    src.print("Please provide the name of save file you'd like to load:");
    let choice = src.prompt().unwrap();

    return extract_protobuf(src, &choice, callback);
}
