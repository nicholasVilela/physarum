use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub agent_count: u32,
    pub diffusion_rate: f32,
    pub diffusion_strength: f32,
}
