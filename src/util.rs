//! A collection of macros and helpers to increase code reuse.

// TODO(31): This macro should become redundant when error propagation is
// properly implemented. Should be replaced by `possibly_failing_function()?`
macro_rules! maybe_bail {
    // This macro takes an expression of type `expr` and attempts
    // to unwrap the value, or exit the calling function on failure.
    ($result:expr) => (
        match $result {
            Ok(unwrapped) => unwrapped,
            Err(error) => return Err(error),
        }
    );
    ($result:expr, $error:expr) => (
        match $result {
            Ok(unwrapped) => unwrapped,
            Err(_) => return Err(String::from($error)),
        }
    );
}
