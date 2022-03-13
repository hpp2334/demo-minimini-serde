#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "_")
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Boolean(bool),
    I32(i32),
    String(String),
    LB,    // {
    RB,    // }
    Colon, // :
    Comma, // ,
}
