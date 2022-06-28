use serde::{Serialize, Deserialize};
use ggez::{graphics::Color};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub background: Color,
    pub show_fps: bool,
}
