use std::{fs::File};
use serde::{de::DeserializeOwned, Serialize};
use ggez::{GameResult};
use ron::{de::from_reader};

mod simulation;
mod species;
mod window;

pub use simulation::*;
pub use species::*;
pub use  window::*;


pub fn load<T: Serialize + DeserializeOwned>(name: &str) -> GameResult<T> {
    let path = format!("{}/config/{}.ron", env!("CARGO_MANIFEST_DIR"), name);
    let file = File::open(&path).expect(&format!("Failed to open file at path: {}", path));
    let config = match from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load {} config: {}", name, e);
            std::process::exit(1);
        }
    };

    return Ok(config);
}
