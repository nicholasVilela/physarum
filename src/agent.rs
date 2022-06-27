use std::{f32::consts::{TAU, PI}, time::Duration};
use rand::{Rng, distributions::Uniform};
use ggez::{GameResult};
use crate::{WindowConfig, Trail, FVec2, load_config, SpeciesConfig, Species, Pattern};
use glam::{Vec2, IVec2, UVec3};


pub struct Agent {
    pub species: Species,
    pub config: SpeciesConfig,
    pub position: Vec2,
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

    fn sense(&mut self, sensor_angle_offset: f32, trail: &Trail,  window_config: &WindowConfig) -> GameResult<f32> {
        let width = window_config.width;
        let height = window_config.height;

        let sensor_angle = self.angle + sensor_angle_offset;
        let sensor_direction = Vec2::new(sensor_angle.cos(), sensor_angle.sin());
        let sensor_position = self.position + sensor_direction * self.config.sensor_distance;
        
        let mut sum = 0.0;

        for offset_x in -self.config.sensor_size..self.config.sensor_size + 1 {
            for offset_y in -self.config.sensor_size..self.config.sensor_size + 1 {
                let pos_x = (width - 1).min((sensor_position.x as i32 + offset_x).max(0));
                let pos_y = (height - 1).min((sensor_position.y as i32 + offset_y).max(0));

                let sample = trail.get_pixel(Vec2::new(pos_x as f32, pos_y as f32), window_config)?;

                sum += (sample.x + sample.y + sample.z) as f32;
            }
        }

        return Ok(sum);
    }

    pub fn update(&mut self, delta: Duration, window_config: &WindowConfig, trail: &mut Trail) -> GameResult {
        let mut rng = rand::thread_rng();

        let width = window_config.width as f32;
        let height = window_config.height as f32;

        let sensor_angle_rad = self.config.sensor_angle * (PI / 180.0);
        let weight_forward = self.sense(0.0, trail, window_config)?;
        let weight_left = self.sense(sensor_angle_rad, trail, window_config)?;
        let weight_right = self.sense(-sensor_angle_rad, trail, window_config)?;

        let turn_speed = self.config.turn_speed * TAU;
        let random_steer_strength = rng.gen::<f32>();

        if weight_forward > weight_left && weight_forward > weight_right {
            self.angle += 0.0;
        }
        else if weight_forward < weight_left && weight_forward < weight_right {
            self.angle += (random_steer_strength - 0.5) * 2.0  * turn_speed * delta.as_secs_f32();
        }
        else if weight_right > weight_left {
            self.angle -= random_steer_strength * turn_speed * delta.as_secs_f32();
        }
        else if weight_left > weight_right {
            self.angle += random_steer_strength * turn_speed * delta.as_secs_f32();
        }

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());
        let mut next_position = self.position + direction * delta.as_secs_f32() * self.config.move_speed;
        
        if next_position.x < 0.0 || next_position.x >= width || next_position.y < 0.0 || next_position.y >= height {
            next_position.x = (width - 1.0).min(next_position.x.max(0.0));
            next_position.y = (height - 1.0).min(next_position.y.max(0.0));

            self.angle = rng.gen::<f32>() * TAU;
        }
        else {
            trail.update_pixel(self.position, &self.config, window_config)?;
        }

        self.position = next_position;

        return Ok(());
    }

    // pub fn update(&mut self, delta: Duration, window_config: &WindowConfig, trail: &mut Trail) -> GameResult {
    //     let mut rng = rand::thread_rng();
        
    //     let width = window_config.width as f32;
    //     let height = window_config.height as f32;

    //     let move_speed = self.config.move_speed;
    //     let look_ahead= self.config.look_ahead;

    //     let direction = FVec2::new(self.angle.cos(), self.angle.sin());
    //     let mut velocity = direction * move_speed * delta.as_secs_f32();
        
    //     // let forward_weight = FVec2::new(velocity.x * look_ahead, velocity.y * look_ahead);
    //     // let left_weight = FVec2::new(velocity.y * look_ahead, -velocity.x * look_ahead);
    //     // let right_weight = FVec2::new(-velocity.y * look_ahead, velocity.x * look_ahead);

    //     let forward_weight = FVec2::new(velocity.x, velocity.y * look_ahead);
    //     let left_weight = FVec2::new(-velocity.x * look_ahead, velocity.y * look_ahead);
    //     let right_weight = FVec2::new(velocity.x * look_ahead, -velocity.y * look_ahead);
        
    //     let forward_pixel = trail.get_pixel(self.position + forward_weight, window_config)?;
    //     let left_pixel = trail.get_pixel(self.position + left_weight, window_config)?;
    //     let right_pixel = trail.get_pixel(self.position + right_weight, window_config)?;
        
    //     let forward_strength = (forward_pixel[0] + forward_pixel[1] + forward_pixel[2]) as usize;
    //     let left_strength = (left_pixel[0] + left_pixel[1] + left_pixel[2]) as usize;
    //     let right_strength = (right_pixel[0] + right_pixel[1] + right_pixel[2]) as usize;

    //     let total_weight = (forward_strength + left_strength + right_strength).min(self.config.max_weight);

    //     if total_weight > 0 {
    //         let nudge_x = ((forward_strength as f32 * forward_weight.x + left_strength as f32 * left_weight.x + right_strength as f32 * right_weight.x) / total_weight as f32) * self.config.strength;
    //         let nudge_y = ((forward_strength as f32 * forward_weight.y + left_strength as f32 * left_weight.y + right_strength as f32 * right_weight.y) / total_weight as f32) * self.config.strength;
    //         velocity = FVec2::new(velocity.x + nudge_x, velocity.y + nudge_y);
    //     }

    //     let mut position = self.position + velocity;

        // if position.x < 0.0 || position.x >= width || position.y < 0.0 || position.y >= height {
        //     position.x = (width - 0.01).min(position.x.max(0.0));
        //     position.y = (height - 0.01).min(position.y.max(0.0));

        //     self.angle = rng.gen::<f32>() * TAU;
        // }

    //     self.position = position;

    //     trail.update_pixel(self.position, &self.config, window_config)?;

    //     return Ok(());
    // }

    pub fn calculate_position<R: Rng + ?Sized>(pattern: &Pattern, window_config: &WindowConfig, rng: &mut R) -> GameResult<Vec2> {
        let mut position = Vec2::new(0.0, 0.0);

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
