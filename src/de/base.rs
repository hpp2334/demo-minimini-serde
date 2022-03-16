use std::rc::Rc;

use crate::common::{Error, Result};

pub struct PlaceStore<T> {
    out: Rc<Option<T>>,
}

impl<T> PlaceStore<T> {
    pub fn set(&mut self, value: T) {
        let inner = unsafe { Rc::get_mut_unchecked(&mut self.out) };
        *inner = Some(value);
    }
    pub fn try_unwrap(mut self) -> Result<T> {
        let out = unsafe { Rc::get_mut_unchecked(&mut self.out) };
        out.take().ok_or(Error)
    }
    pub fn clone(&self) -> Self {
        Self {
            out: Rc::clone(&self.out),
        }
    }
}

impl<T> std::default::Default for PlaceStore<T> {
    fn default() -> Self {
        Self { out: Rc::new(None) }
    }
}

#[macro_export]
macro_rules! make_place {
    ($name:ident) => {
        pub struct $name<T> {
            pub out: PlaceStore<T>,
        }
    };
}

make_place!(Place);

// Default implement for Visitor
pub trait Visitor {
    fn boolean(&mut self, _x: bool) -> Result<()> {
        unimplemented!()
    }
    fn i32(&mut self, _x: i32) -> Result<()> {
        unimplemented!()
    }
    fn string(&mut self, _x: &str) -> Result<()> {
        unimplemented!()
    }
    fn map(&mut self) -> Result<Box<dyn Map>> {
        unimplemented!()
    }
}

pub trait Map {
    fn key(&mut self, key: &str) -> Result<Box<dyn Visitor>>;
    fn finish(&mut self) -> Result<()>;
}

pub trait Deserialize: Sized {
    fn begin(out: PlaceStore<Self>) -> Box<dyn Visitor>;
}
