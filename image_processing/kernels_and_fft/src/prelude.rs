pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper tuple struct for new type pattern
pub struct W<T>(pub T);