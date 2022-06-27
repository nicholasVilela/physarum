use std::{f32::consts::TAU, time::Duration};
use rand::{Rng, distributions::Uniform};
use ggez::{GameResult};
use crate::{WindowConfig, Trail, FVec2, load_config, SpeciesConfig, Species, Pattern};


pub struct Agent {
    pub species: Species,
    pub config: SpeciesConfig,
    pub position: FVec2,
    pub angle: f32,
}

impl Agent {
    pub fn new<R: Rng + ?Sized>(species: Species, window_config: &WindowConfig, pattern: &Pattern,  rng: &mut R) -> GameResult<Agent> {
        let angle = rng.gen::<f32>();
        let config = load_config::<SpeciesConfig>(&Species::A.to_string())?;
        let position = Agent::calculate_position(pattern, window_config, rng)?;

        let agent = Agent {
            species,
            config,
            position,
            angle: angle * TAU,
        };

        return Ok(agent);
    }

    pub fn update(&mut self, delta: Duration, window_config: &WindowConfig, trail: &mut Trail) -> GameResult {
        let mut rng = rand::thread_rng();
        
        let width = window_config.width as f32;
        let height = window_config.height as f32;

        let move_speed = self.config.move_speed;
        let look_ahead= self.config.look_ahead;

        let direction = FVec2::new(self.angle.cos(), self.angle.sin());
        let mut velocity = direction * move_speed * delta.as_secs_f32();
        
        let forward_weight = FVec2::new(velocity.x * look_ahead, velocity.y * look_ahead);
        let left_weight = FVec2::new(velocity.y * look_ahead, -velocity.x * look_ahead);
        let right_weight = FVec2::new(-velocity.y * look_ahead, velocity.x * look_ahead);
        
        let forward_pixel = trail.get_pixel(self.position + forward_weight, window_config)?;
        let left_pixel = trail.get_pixel(self.position + left_weight, window_config)?;
        let right_pixel = trail.get_pixel(self.position + right_weight, window_config)?;
        
        let forward_strength = (forward_pixel[0] + forward_pixel[1] + forward_pixel[2]) as usize;
        let left_strength = (left_pixel[0] + left_pixel[1] + left_pixel[2]) as usize;
        let right_strength = (right_pixel[0] + right_pixel[1] + right_pixel[2]) as usize;

        let total_weight = (forward_strength + left_strength + right_strength).min(self.config.max_weight);

        if total_weight > 0 {
            let nudge_x = ((forward_strength as f32 * forward_weight.x + left_strength as f32 * left_weight.x + right_strength as f32 * right_weight.x) / total_weight as f32) * self.config.strength;
            let nudge_y = ((forward_strength as f32 * forward_weight.y + left_strength as f32 * left_weight.y + right_strength as f32 * right_weight.y) / total_weight as f32) * self.config.strength;
            velocity = FVec2::new(velocity.x + nudge_x, velocity.y + nudge_y);
        }

        let mut position = self.position + velocity;

        if position.x < 0.0 || position.x >= width || position.y < 0.0 || position.y >= height {
            position.x = (width - 0.01).min(position.x.max(0.0));
            position.y = (height - 0.01).min(position.y.max(0.0));

            self.angle = rng.gen::<f32>() * TAU;
        }

        self.position = position;

        trail.update_pixel(self.position, &self.config, window_config)?;

        return Ok(());
    }

    pub fn calculate_position<R: Rng + ?Sized>(pattern: &Pattern, window_config: &WindowConfig, rng: &mut R) -> GameResult<FVec2> {
        let mut position = FVec2::new(0.0, 0.0);

        match pattern {
            Pattern::Random => {
                let (x, y) = rng.gen::<(f32, f32)>();

                position.x = x * window_config.width as f32;
                position.y = y * window_config.height as f32;
            },
            Pattern::Spherical => {
                todo!();
            },
            Pattern::Center => {
                position.x = window_config.width as f32 / 2.0;
                position.y = window_config.height as f32 / 2.0;
            }
        };

        return Ok(position);
    }
}
