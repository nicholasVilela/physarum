#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Param {
    pub delta_time: f32,
    pub frame: u32,
}
