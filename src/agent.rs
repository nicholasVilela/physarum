use std::{f32::consts::TAU, collections::HashMap, time::Duration, fmt};
use rand::{Rng};
use ggez::{GameResult, timer};
use crate::{Vec2, WindowConfig, Trail, FVec2, load_config, SpeciesConfig};


#[derive(Debug)]
pub enum Species {
    A,
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "species_{:?}", self);
    }
}

pub struct Agent {
    pub species: Species,
    pub position: FVec2,
    pub angle: f32,
}

impl Agent {
    pub fn new<R: Rng + ?Sized>(species: Species, window_config: &WindowConfig,  rng: &mut R) -> GameResult<Agent> {
        let (x, y, angle) = rng.gen::<(f32, f32, f32)>();
        let agent = Agent {
            species,
            position: FVec2::new(x * window_config.width as f32 , y * window_config.height as f32),
            angle: angle * TAU,
        };

        return Ok(agent);
    }

    pub fn update(&mut self, delta: Duration, window_config: &WindowConfig, trail: &mut Trail) -> GameResult {
        let mut rng = rand::thread_rng();

        let move_speed = self.move_speed()?;

        let direction = FVec2::new(self.angle.cos(), self.angle.sin());
        let mut position = self.position + direction * move_speed * delta.as_secs_f32();

        let width = window_config.width as f32;
        let height = window_config.height as f32;

        if position.x < 0.0 || position.x >= width || position.y < 0.0 || position.y >= height {
            position.x = (width - 0.01).min(position.x.max(0.0));
            position.y = (height - 0.01).min(position.y.max(0.0));

            self.angle = rng.gen::<f32>() * TAU;
        }

        self.position = position;

        trail.update_pixel(self.position, window_config)?;

        return Ok(());
    }

    fn move_speed(&mut self) -> GameResult<f32> {
        let config = load_config::<SpeciesConfig>(&Species::A.to_string())?;

        return Ok(config.move_speed);

        // match self.species {
        //     Species::A => {
        //         let config = load_config::<SpeciesConfig>("species_a")?;

        //         return Ok(config.move_speed);
        //     },
        // }
    }
}