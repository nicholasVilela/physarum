use ggez::{ContextBuilder, event, conf::WindowMode, GameResult};

#[cfg(test)]
mod tests;

mod agent;
mod config;
mod engine;
mod enums;
mod params;
mod simulation;
mod trail;
mod util;

pub use agent::*;
pub use config::*;
pub use engine::*;
pub use enums::*;
pub use params::*;
pub use simulation::*;
pub use trail::*;
pub use util::*;



fn main() -> GameResult {
    let window_config = load::<WindowConfig>("window")?;
    let window_mode = WindowMode::default().dimensions(window_config.width as f32, window_config.height as f32);

    let (mut ctx, event_loop) = ContextBuilder::new("Physarum", "nicholasVilela")
        .add_resource_path("resources")
        .window_mode(window_mode)
        .build()
        .expect("Context could not be created.");

    let engine = Engine::new(&mut ctx)?;

    event::run(ctx, event_loop, engine);
}
