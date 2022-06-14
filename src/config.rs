use std::{fs::File, collections::HashMap};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use ggez::{GameResult};
use ron::{de::from_reader};


pub fn load_config<T: Serialize + DeserializeOwned>(name: &str) -> GameResult<T> {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub agent_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeciesConfig {
    pub move_speed: f32,
}