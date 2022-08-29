use serde::{Serialize, Deserialize};
use ggez::{graphics::Color};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub background: Color,
    pub show_fps: bool,
    pub fullscreen: bool,
    pub auto_run: bool,
}
