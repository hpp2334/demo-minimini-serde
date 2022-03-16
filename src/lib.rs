#![feature(get_mut_unchecked)]

pub(crate) mod common;
pub mod de;
mod json;
pub mod ser;

pub use common::*;
pub use de::{Deserialize, Place, PlaceStore, Visitor};
pub use json::*;
pub use ser::Serialize;

pub use std::boxed::Box;
pub use std::option::Option;
pub use std::rc::Rc;

pub use minimini_serde_derive::*;
