extern crate glob;

use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use glob::glob;

fn main() {
    println!("cargo:rerun-if-changed=\"src/protos/*.proto\"");

    generate_pbs();
    generate_protos();
}

fn generate_pbs() {
    for entry in glob("./games/*.pbtxt").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => generate_pb(path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn generate_pb(path: std::path::PathBuf) {
    let file_name = &path.file_stem()
        .expect("Error parsing file stem in build.rs")
        .to_str()
        .expect("Error converting path name to string in build.rs");

    let output_file = File::create(format!("games/{}.pb", &file_name))
        .expect("Error creating output file in build.rs");

    let mut input_file = File::open(&path).expect("Failed to open file");
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .expect("Failed to read from file.");

    let mut protos_cmd = Command::new("protoc")
        .arg("--encode=Maeve.Game")
        .arg("protos/master.proto")
        .stdin(Stdio::piped())
        .stdout(Stdio::from(output_file))
        .spawn()
        .expect("Failed to compile pb file.");

    /* Assign _ because it must be used */
    protos_cmd
        .stdin
        .as_mut()
        .expect("Error borrowing mutably in build.rs")
        .write_all(contents.as_bytes())
        .expect("Couldn't write to pb file.");
}

fn generate_protos() {
    for entry in glob("./protos/*.proto").expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => generate_proto(path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn generate_proto(path: std::path::PathBuf) {
    Command::new("protoc")
        .arg("--rust_out")
        .arg("src/protos")
        .arg(&path)
        .spawn()
        .expect("Failed to generate proto");
}
