use crate::{Option, Result};

pub enum Fragment<'a> {
    Boolean(bool),
    I32(i32),
    String(String),
    Map(Box<dyn Map + 'a>),
}

pub trait Map {
    fn next(&mut self) -> Option<(String, &dyn Serialize)>;
}

pub trait Serialize {
    fn begin(&self) -> Result<Fragment>;
}
