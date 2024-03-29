use ggez::{ContextBuilder, event, conf::{WindowMode, WindowSetup, FullscreenType}, GameResult};

#[cfg(test)]
mod tests;

mod agent;
mod config;
mod constants;
mod construct;
mod engine;
mod enums;
mod param;
mod programs;
mod simulation;
mod species;
mod storage;
mod trail;
mod util;

pub use agent::*;
pub use config::*;
pub use constants::*;
pub use construct::*;
pub use engine::*;
pub use enums::*;
pub use param::*;
pub use programs::*;
pub use simulation::*;
pub use species::*;
pub use storage::*;
pub use trail::*;
pub use util::*;



fn main() -> GameResult {
    let window_config = load::<WindowConfig>("window")?;
    let window_mode = WindowMode::default()
        .fullscreen_type(if window_config.fullscreen { FullscreenType::True } else { FullscreenType::Windowed })
        .dimensions(window_config.width as f32, window_config.height as f32);
    let window_setup = WindowSetup::default()
        .title(&window_config.title);

    let (mut ctx, event_loop) = ContextBuilder::new("Physarum", "nicholasVilela")
        .add_resource_path("resources")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Context could not be created.");

    let engine = Engine::new(&mut ctx)?;

    event::run(ctx, event_loop, engine);
}
