use ggez::{ContextBuilder, event, conf::WindowMode, GameResult};

mod agent;
pub use agent::*;

mod config;
pub use config::*;

mod engine;
pub use engine::*;

mod enums;
pub use enums::*;

mod trail;
pub use trail::*;

mod vec2;
pub use vec2::*;

mod fvec2;
pub use fvec2::*;

#[cfg(test)]
mod tests;


fn main() -> GameResult {
    let window_config = load_config::<WindowConfig>("window")?;
    let window_mode = WindowMode::default().dimensions(window_config.width as f32, window_config.height as f32);

    let (mut ctx, event_loop) = ContextBuilder::new("Physarum", "nicholasVilela")
        .window_mode(window_mode)
        .build()
        .expect("Context could not be created.");

    let engine = Engine::new(&mut ctx)?;

    event::run(ctx, event_loop, engine);
}
