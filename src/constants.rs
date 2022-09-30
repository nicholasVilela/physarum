use crate::{WindowConfig, SimulationConfig};
use ggez::{GameResult};


#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Constants {
    window_height: f32,
    window_width: f32,
    diffusion_rate: f32,
    diffusion_strength: f32,
}

impl Constants {
    pub fn new(window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult<Constants> {
        let constants = Constants {
            window_height: window_config.height as f32,
            window_width: window_config.width as f32,
            diffusion_rate: simulation_config.diffusion_rate,
            diffusion_strength: simulation_config.diffusion_strength,
        };

        return Ok(constants);
    }
}
