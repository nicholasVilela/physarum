use serde::{Serialize, Deserialize};
use crate::{Pattern};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub agent_count: u32,
    pub evaporation_speed: u8,
    pub blur_strength: u8,
    // pub pattern: Pattern,
    // pub render_agents: bool,
}
