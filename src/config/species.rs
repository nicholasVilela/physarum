use serde::{Serialize, Deserialize};
use ggez::{graphics::Color};


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpeciesConfig {
    pub move_speed: f32,
    pub sensor_angle: f32,
    pub sensor_distance: f32,
    pub sensor_size: i32,
    pub turn_speed: f32,
    pub forward_random_strength: f32,
    pub color: Color,
}
