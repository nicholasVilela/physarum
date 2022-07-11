use ggez::{ContextBuilder, event, conf::WindowMode, GameResult};

#[cfg(test)]
mod tests;

mod agent;
mod config;
mod constants;
mod engine;
mod enums;
mod param;
mod programs;
mod simulation;
mod storage;
mod trail;
mod util;

pub use agent::*;
pub use config::*;
pub use constants::*;
pub use engine::*;
pub use enums::*;
pub use param::*;
pub use programs::*;
pub use simulation::*;
pub use storage::*;
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
