#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct SimulationConstants {
    evaporation_rate: f32,
    diffusion_rate: f32,
    diffusion_strength: f32,
}
