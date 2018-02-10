//! Operations that interact with the file system.

use error::MaeveError;
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
) -> Result<M, MaeveError>
where
    F: Fn(&mut I, M) -> Result<M, MaeveError>,
{
    return match File::open(&Path::new(path)) {
        Ok(mut is) => match ::protobuf::parse_from_reader::<M>(&mut is) {
            Ok(t) => callback(src, t),
            Err(err) => Err(MaeveError::Proto(err)),
        },
        Err(_) => Err(MaeveError::Load),
    };
}

pub fn write_protobuf<M: Message>(
    path: &str,
    msg: &M,
) -> Result<(), MaeveError> {
    let mut file = File::create(&Path::new(&path))?;
    let mut cos = CodedOutputStream::new(&mut file);
    msg.write_to(&mut cos)?;
    cos.flush()?;
    return Ok(());
}

pub fn prompt_path<F, M: MessageStatic, I: Interfaceable>(
    src: &mut I,
    callback: F,
) -> Result<M, MaeveError>
where
    F: Fn(&mut I, M) -> Result<M, MaeveError>,
{
    src.print("Please provide the name of save file you'd like to load:");
    let choice = src.prompt()?;

    return extract_protobuf(src, &choice, callback);
}
