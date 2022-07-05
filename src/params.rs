use ggez::{GameResult};


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimulationParams {
    pub delta_time: f32,
    pub frame: u32,
}
