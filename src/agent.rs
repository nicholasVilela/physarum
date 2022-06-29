use std::{f32::consts::{TAU, PI}, time::Duration};
use rand::{Rng};
use ggez::{GameResult};
use crate::{WindowConfig, Trail, SpeciesConfig, Species, Pattern, SimulationConfig};
use glam::{Vec2};


pub struct Agent {
    pub species: Species,
    pub config: SpeciesConfig,
    // pub mask: UVec3,
    pub position: Vec2,
    pub angle: f32,
}

impl Agent {
    pub fn new<R: Rng + ?Sized>(species: Species, species_config: SpeciesConfig, window_config: &WindowConfig, simulation_config: &SimulationConfig,  rng: &mut R) -> GameResult<Agent> {
        let angle = rng.gen::<f32>();
        let position = Agent::calculate_position(&simulation_config.pattern, window_config, rng)?;
        
        // let species_id = rng.gen_range(1..simulation_config.species_count + 1);
        // let index = species_id - 1;
        // let mask = UVec3::new(
        //     if species_id == 1 { 1 } else { 0 },
        //     if species_id == 2 { 1 } else { 0 },
        //     if species_id == 3 { 1 } else { 0 },
        // );

        let agent = Agent {
            species,
            config: species_config,
            position,
            angle: angle * TAU,
            // mask,
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
        // let sense_weight = self.mask * 2 - 1;

        for offset_x in -self.config.sensor_size..self.config.sensor_size + 1 {
            for offset_y in -self.config.sensor_size..self.config.sensor_size + 1 {
                let pos_x = (width - 1).min((sensor_position.x as i32 + offset_x).max(0));
                let pos_y = (height - 1).min((sensor_position.y as i32 + offset_y).max(0));

                let sample = trail.get_pixel(Vec2::new(pos_x as f32, pos_y as f32), window_config)?;

                // sum += sense_weight.dot(sample) as f32;
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
            self.angle += (random_steer_strength + self.config.forward_random_strength) * 2.0  * turn_speed * delta.as_secs_f32();
        }
        else if weight_right > weight_left {
            self.angle -= (random_steer_strength + self.config.right_random_strength) * turn_speed * delta.as_secs_f32();
        }
        else if weight_left > weight_right {
            self.angle += (random_steer_strength + self.config.left_random_strength) * turn_speed * delta.as_secs_f32();
        }

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());
        let mut next_position = self.position + direction * delta.as_secs_f32() * self.config.move_speed;
        
        if next_position.x < 0.0 || next_position.x >= width || next_position.y < 0.0 || next_position.y >= height {
            next_position.x = (width - 1.0).min(next_position.x.max(0.0));
            next_position.y = (height - 1.0).min(next_position.y.max(0.0));

            self.angle = rng.gen::<f32>() * TAU;
        }
        else {
            // let old_trail = trail.get_pixel(next_position, window_config)?;
            // let value = (UVec3::new(1, 1, 1)).min(old_trail + self.mask);
            // trail.apply(self.position, value, &self.config,  window_config)?;

            trail.update_pixel(self.position, &self.config, window_config)?;
        }

        self.position = next_position;

        return Ok(());
    }

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
