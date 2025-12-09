mod error;
pub use error::{HttpStatus, Error};

pub type Result<T> = std::result::Result<T, Error>;
