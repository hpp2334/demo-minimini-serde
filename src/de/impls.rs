use super::{
    base::{Deserialize, Place, Visitor},
    PlaceStore,
};
use crate::common::Result;

impl Deserialize for bool {
    fn begin(out: PlaceStore<Self>) -> Box<dyn Visitor> {
        impl Visitor for Place<bool> {
            fn boolean(&mut self, x: bool) -> Result<()> {
                self.out.set(x);
                Ok(())
            }
        }

        let place = Place { out };
        Box::new(place)
    }
}

impl Deserialize for i32 {
    fn begin(out: PlaceStore<Self>) -> Box<dyn Visitor> {
        impl Visitor for Place<i32> {
            fn i32(&mut self, x: i32) -> Result<()> {
                self.out.set(x);
                Ok(())
            }
        }

        let place = Place { out };
        Box::new(place)
    }
}

impl Deserialize for String {
    fn begin(out: PlaceStore<Self>) -> Box<dyn Visitor> {
        impl Visitor for Place<String> {
            fn string(&mut self, x: &str) -> Result<()> {
                self.out.set(x.to_owned());
                Ok(())
            }
        }

        let place = Place { out };
        Box::new(place)
    }
}
