use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub enum Species {
    A,
    B,
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "species_{:?}", self);
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Pattern {
    Random,
    Center,
    Spherical,
}
