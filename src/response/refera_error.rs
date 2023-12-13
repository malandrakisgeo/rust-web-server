use std::{error, fmt, io};
//https://blog.burntsushi.net/rust-error-handling/#standard-library-traits-used-for-error-handling
#[derive(Debug)]
pub enum ReferaError {
    Io(io::Error),
}
impl From<io::Error> for ReferaError {
    fn from(err: io::Error) -> ReferaError {
        println!("Error!");
        ReferaError::Io(err)
    }
}

impl fmt::Display for ReferaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ReferaError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for ReferaError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ReferaError::Io(ref err) => err.description(),
            //ReferaError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            ReferaError::Io(ref err) => Some(err),
            //ReferaError::Parse(ref err) => Some(err),
        }
    }
}