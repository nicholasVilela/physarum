use std::num::{NonZeroU8, NonZeroUsize};
use std::iter::repeat;
use ggez::{graphics::{Image, MeshBatch, MeshBuilder, DrawMode, Color}, Context, GameResult};
use crate::{WindowConfig, Vec2, SpeciesConfig, FVec2, SimulationConfig};
use stackblur::blur;


pub struct Trail {
    pub map: Image,
    pub buffer: Vec<u8>,
}

impl Trail {
    pub fn new(ctx: &mut Context, window_config: &WindowConfig) -> GameResult<Trail> {
        let width = window_config.width as u16;
        let height = window_config.height as u16;

        let buffer: Vec<u8>= Trail::construct_buffer(width as usize, height as usize)?;
        let map = Image::from_rgba8(ctx, width, height, &buffer)?;

        let trail = Trail { map, buffer };

        return Ok(trail);
    }

    pub fn update(&mut self, ctx: &mut Context, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult {
        let width = window_config.width as u16;
        let height = window_config.height as u16;

        let mut pixels = unsafe { self.buffer.align_to_mut::<u32>().1 };
        blur(&mut pixels, NonZeroUsize::new(width as usize).unwrap(), NonZeroUsize::new(height as usize).unwrap(), NonZeroU8::new(simulation_config.blur_strength).unwrap());

        for y in 0..window_config.height {
            for x in 0..window_config.width {
                let position = FVec2::new(x as f32, y as f32);

                self.evaporate_pixel(position, window_config, simulation_config)?;
            }
        }

        self.map = Image::from_rgba8(ctx, width, height, &self.buffer)?;

        return Ok(());
    }

    pub fn update_pixel(&mut self, position: FVec2, species_config: &SpeciesConfig, window_config: &WindowConfig) -> GameResult {
        let pixel_index = self.get_pixel_index(position, window_config)?;

        // let position_map = 

        self.buffer[pixel_index] = (species_config.color.r * 255.0) as u8;
        self.buffer[pixel_index + 1] = (species_config.color.g * 255.0) as u8;
        self.buffer[pixel_index + 2] = (species_config.color.b * 255.0) as u8;

        return Ok(());
    }

    pub fn evaporate_pixel(&mut self, position: FVec2, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult {
        let pixel_index = self.get_pixel_index(position, window_config)?;
        let evaporation_speed = simulation_config.evaporation_speed;

        if self.buffer[pixel_index] > evaporation_speed {
            self.buffer[pixel_index] -= evaporation_speed;
        }
        if self.buffer[pixel_index + 1] > evaporation_speed {
            self.buffer[pixel_index + 1] -= evaporation_speed;
        }
        if self.buffer[pixel_index + 2] > evaporation_speed {
            self.buffer[pixel_index + 2] -= evaporation_speed;
        }

        return Ok(());
    }

    pub fn get_pixel_index(&mut self, position: FVec2, window_config: &WindowConfig) -> GameResult<usize> {
        let pixel_index = position.y as usize;
        let pixel_index = pixel_index * window_config.width as usize + position.x as usize;
        let mut pixel_index = pixel_index * 4;

        let max_pixel_index = ((window_config.width * window_config.height * 4)) as usize - 4;

        if pixel_index > max_pixel_index {
            pixel_index = max_pixel_index;
        }

        return Ok(pixel_index);
    }

    pub fn get_pixel(&mut self, position: FVec2, window_config: &WindowConfig) -> GameResult<Vec<u8>> {
        let pixel_index = self.get_pixel_index(position, window_config)?;

        let pixel_r = self.buffer[pixel_index];
        let pixel_g = self.buffer[pixel_index + 1];
        let pixel_b = self.buffer[pixel_index + 2];
        
        let pixel = vec!{pixel_r, pixel_g, pixel_b};

        return Ok(pixel);
    }

    fn construct_buffer( width: usize, height: usize) -> GameResult<Vec<u8>> {
        let count = width * height * 4;
        let color = vec![0,0,0,255];
        let buffer: Vec<u8> = repeat(color).flat_map(|x| x).take(count).collect();

        return Ok(buffer);
    }
}