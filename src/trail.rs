use ggez::{GameResult};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Trail {
    pub position: [f32; 2],
    pub value: f32,
    pub t: f32,
}

impl Trail {
    pub fn new(position: [f32; 2], value: f32) -> GameResult<Trail> {
        let trail = Trail { position, value, t: 0.0 };

        return Ok(trail);
    }
}

