use std::fmt::Display;
use std::process::exit;

pub trait ExpectExit<T> {
    fn expect_exit(self, msg: &str) -> T;
}

impl<T> ExpectExit<T> for Option<T> {
    fn expect_exit(self, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                eprintln!("Error: {}", msg);
                exit(1)
            }
        }
    }
}

impl<T, E: Display> ExpectExit<T> for Result<T, E> {
    fn expect_exit(self, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error: {} - {}", msg, err);
                exit(1)
            }
        }
    }
}
