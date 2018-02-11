extern crate maeve;

use maeve::error::MaeveError;
use std::error::Error;

#[test]
fn exit_test() {
    let x = MaeveError::Exit;
    assert_eq!(x.description(), "Exiting");
}
