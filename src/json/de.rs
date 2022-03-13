use crate::common::Token;
use crate::de::Visitor;
use crate::PlaceStore;

use crate::common::{Error, Result};
use crate::de::Deserialize;

enum Event {
    Primitive(Token),
    MapStart,
    End,
}

struct Deserializer<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Deserializer<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    pub fn event(&mut self) -> Result<Event> {
        if self.pos >= self.tokens.len() {
            return Ok(Event::End);
        }

        let token = self.tokens[self.pos].clone();
        let evt = match &token {
            Token::Boolean(_) => Event::Primitive(token),
            Token::I32(_) => Event::Primitive(token),
            Token::String(_) => Event::Primitive(token),
            Token::LB => Event::MapStart,
            _ => return Err(Error),
        };
        self.bump();
        Ok(evt)
    }
    pub fn bump(&mut self) {
        self.pos += 1;
    }
    pub fn peek(&self) -> Token {
        self.tokens[self.pos].clone()
    }
    pub fn parse(&mut self) -> Token {
        let token = self.peek();
        self.bump();
        token
    }
}

pub fn from_tokens<'a, T: Deserialize>(tokens: &'a Vec<Token>) -> Result<T> {
    let out: PlaceStore<T> = Default::default();
    let mut de = Deserializer::new(tokens);

    let visitor = T::begin(out.clone());

    from_tokens_impl(&mut de, visitor)?;

    let out = out.try_unwrap()?;
    Ok(out)
}

fn from_tokens_impl<'a>(de: &mut Deserializer, mut visitor: Box<dyn Visitor>) -> Result<()> {
    match de.event()? {
        Event::Primitive(token) => match token {
            Token::I32(x) => {
                visitor.i32(x)?;
            }
            Token::Boolean(x) => {
                visitor.boolean(x)?;
            }
            Token::String(x) => {
                visitor.string(x.as_str())?;
            }
            _ => {
                return Err(Error);
            }
        },
        Event::MapStart => {
            let mut map = visitor.map()?;

            'map_start_loop: loop {
                let key = match de.parse() {
                    Token::String(x) => Ok(x),
                    Token::RB => break 'map_start_loop,
                    _ => Err(Error),
                }?;
                de.parse(); // :

                let visitor = map.key(key.as_str())?;
                from_tokens_impl(de, visitor)?;
                if let Token::Comma = de.peek() {
                    de.parse(); // ,
                }
                if let Token::RB = de.peek() {
                    map.finish()?;
                }
            }
        }
        Event::End => return Ok(()),
    };
    Ok(())
}
