extern crate prost;

// Include modules, generated from *.proto.
pub mod master {
    include!(concat!(env!("OUT_DIR"), "/maeve.rs"));
}
