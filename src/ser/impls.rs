use super::*;
use crate::{Result};

impl Serialize for i32 {
    fn begin(&self) -> Result<Fragment> {
        Ok(Fragment::I32(*self))
    }
}

impl Serialize for bool {
    fn begin(&self) -> Result<Fragment> {
        Ok(Fragment::Boolean(*self))
    }
}

impl Serialize for String {
    fn begin(&self) -> Result<Fragment> {
        Ok(Fragment::String(self.clone()))
    }
}
