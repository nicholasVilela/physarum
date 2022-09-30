use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SpeciesConfig {
    pub sensor_size: f32,
    pub sensor_angle: f32,
    pub sensor_distance: f32,
    pub turn_speed: f32,
    pub move_speed: f32,
    pub random_forward_strength: f32,
    pub random_left_strength: f32,
    pub random_right_strength: f32,
    pub weight: f32,
    pub color: [f32; 3],
}
