use std::{f32::consts::TAU, collections::HashMap, time::Duration};
use rand::{Rng};
use ggez::{GameResult, timer};
use crate::{Vec2, WindowConfig};


pub struct Agent {
    pub position: Vec2,
    pub angle: f32,
    pub move_speed: f32,
}

impl Agent {
    pub fn new<R: Rng + ?Sized>(width: f32, height: f32, move_speed: f32, rng: &mut R) -> GameResult<Agent> {
        let (x, y, angle) = rng.gen::<(f32, f32, f32)>();
        let agent = Agent {
            position: Vec2::new(x * width, y * height),
            angle: angle * TAU,
            move_speed,
        };

        return Ok(agent);
    }

    pub fn update(&mut self, delta: Duration, window_config: &WindowConfig, trail_map: &mut HashMap<String, i32>) -> GameResult {
        let mut rng = rand::thread_rng();

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());
        let mut position = self.position + direction * self.move_speed * delta.as_secs_f32();

        let width = window_config.width;
        let height = window_config.height;

        if position.x < 0.0 || position.x >= width || position.y < 0.0 || position.y >= height {
            position.x = (width - 0.01).min(position.x.max(0.0));
            position.y = (height - 0.01).min(position.y.max(0.0));

            self.angle = rng.gen::<f32>() * TAU;
        }

        // trail_map.remove(&self.position.string());

        self.position = position ;
        
        if trail_map.contains_key(&self.position.string()) {
            let value = trail_map.get_mut(&self.position.string()).unwrap();
            *value = 1;
        }
        else {
            trail_map.insert(self.position.string(), 1);
        }

        return Ok(());
    }
}