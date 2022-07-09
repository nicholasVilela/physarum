use serde::{Serialize, Deserialize};
use crate::{Pattern};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub agent_count: u32,
    pub evaporation_rate: f32,
    pub diffusion_rate: f32,
    pub diffusion_strength: f32,
}
