extern crate prost;

// Include the `items` module, which is generated from items.proto.
pub mod master {
    include!(concat!(env!("OUT_DIR"), "/maeve.rs"));
}
