use std::rc::Rc;

use crate::{ser::Fragment, Result, Serialize, Token};

struct UnsafeRcVec<T> {
    vec: Rc<Vec<T>>,
}

impl<T> UnsafeRcVec<T> {
    pub fn new() -> Self {
        Self {
            vec: Default::default(),
        }
    }
    pub fn push(&mut self, v: T) {
        let vec = unsafe { Rc::get_mut_unchecked(&mut self.vec) };
        vec.push(v);
    }
    pub fn clone(&self) -> Self {
        Self {
            vec: Rc::clone(&self.vec),
        }
    }
    pub fn unwrap(mut self) -> Vec<T> {
        let vec = unsafe { Rc::get_mut_unchecked(&mut self.vec) };
        let vec = std::mem::take(vec);
        vec
    }
}

pub fn to_tokens<T: Serialize>(v: &T) -> Result<Vec<Token>> {
    let vec = UnsafeRcVec::new();
    to_token_impl(v, vec.clone())?;

    let vec = vec.unwrap();
    Ok(vec)
}

fn to_token_impl(v: &dyn Serialize, mut vec: UnsafeRcVec<Token>) -> Result<()> {
    let fragment = Serialize::begin(v)?;

    match fragment {
        Fragment::I32(x) => {
            vec.push(Token::I32(x));
        }
        Fragment::Boolean(x) => {
            vec.push(Token::Boolean(x));
        }
        Fragment::String(x) => {
            vec.push(Token::String(x));
        }
        Fragment::Map(mut map) => {
            let mut should_push_comma = false;

            vec.push(Token::LB);
            while let Some((key, v)) = map.next() {
                if should_push_comma {
                    vec.push(Token::Comma);
                }
                should_push_comma = true;

                vec.push(Token::String(key));
                vec.push(Token::Colon);
                to_token_impl(v, vec.clone())?;
            }
            vec.push(Token::RB);
        }
    }

    Ok(())
}
