//! Operations that interact with the file system.

use error::MaeveError;
use prost::Message;
use screen::Interfaceable;
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub fn extract_protobuf<F, M, I>(
    src: &mut I,
    path: &str,
    callback: F,
) -> Result<M, MaeveError>
where
    F: Fn(&mut I, M) -> Result<M, MaeveError>,
    M: Message,
    I: Interfaceable,
{
    let mut buf = Vec::new();
    File::open(&Path::new(path))?.read_to_end(&mut buf);

    let message = M::decode(&buf)?;
    return Ok(callback(src, message)?);
}

pub fn write_protobuf<M: Message>(
    path: &str,
    message: &M,
) -> Result<(), MaeveError> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf)?;
    File::create(&Path::new(&path))?.write_all(&buf)?;
    return Ok(());
}

pub fn prompt_path<F, M: Message, I: Interfaceable>(
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
