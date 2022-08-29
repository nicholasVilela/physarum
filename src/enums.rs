use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Pattern {
    Random,
    Center,
    Spherical,
}
