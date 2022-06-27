use serde::{Serialize, Deserialize};
use ggez::{graphics::Color};


#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub background: Color,
}
