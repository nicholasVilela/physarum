use std::num::{NonZeroU8, NonZeroUsize};
use std::iter::repeat;
use ggez::{graphics::{Image, MeshBatch, MeshBuilder, DrawMode, Color}, Context, GameResult};
use crate::{WindowConfig, SpeciesConfig, FVec2, SimulationConfig};
use stackblur::blur;
use glam::{UVec3, Vec2};


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

        if simulation_config.blur_strength > 0 {
            let mut pixels = unsafe { self.buffer.align_to_mut::<u32>().1 };
            blur(&mut pixels, NonZeroUsize::new(width as usize).unwrap(), NonZeroUsize::new(height as usize).unwrap(), NonZeroU8::new(simulation_config.blur_strength).unwrap());
        }

        for y in 0..window_config.height {
            for x in 0..window_config.width {
                let position = Vec2::new(x as f32, y as f32);

                self.evaporate_pixel(position, window_config, simulation_config)?;
            }
        }

        self.map = Image::from_rgba8(ctx, width, height, &self.buffer)?;

        return Ok(());
    }

    pub fn update_pixel(&mut self, position: Vec2, species_config: &SpeciesConfig, window_config: &WindowConfig) -> GameResult {
        let pixel_index = self.get_pixel_index(position, window_config)?;

        self.buffer[pixel_index] = (species_config.color.r * 255.0) as u8;
        self.buffer[pixel_index + 1] = (species_config.color.g * 255.0) as u8;
        self.buffer[pixel_index + 2] = (species_config.color.b * 255.0) as u8;

        return Ok(());
    }

    fn evaporate_color(&mut self, pixel_index: usize, evaporation_speed: u8) -> GameResult {
        if self.buffer[pixel_index] == 0 { return Ok(()); }

        if self.buffer[pixel_index] < evaporation_speed { 
            self.buffer[pixel_index] = 0;
            return Ok(()); 
        } 

        self.buffer[pixel_index] -= evaporation_speed;
        
        return Ok(());
    }

    pub fn evaporate_pixel(&mut self, position: Vec2, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult {
        let pixel_index = self.get_pixel_index(position, window_config)?;
        let evaporation_speed = simulation_config.evaporation_speed;

        for i in 0..3 {
            self.evaporate_color(pixel_index + i, evaporation_speed)?;
        }

        return Ok(());
    }

    pub fn get_pixel_index(&self, position: Vec2, window_config: &WindowConfig) -> GameResult<usize> {
        let pixel_index = position.y as usize;
        let pixel_index = pixel_index * window_config.width as usize + position.x as usize;
        let mut pixel_index = pixel_index * 4;

        let max_pixel_index = ((window_config.width * window_config.height * 4)) as usize - 4;

        if pixel_index > max_pixel_index {
            pixel_index = max_pixel_index;
        }

        return Ok(pixel_index);
    }

    // pub fn get_pixels_in_radius(&mut self, position: Vec2, radius: i32) -> GameResult<Vec<Vec2>> {
    //     let pixel_list = Trail::calculate_pixel_list(radius);
    //     let positions: Vec<Vec2> = pixel_list.iter().map(|pos| FVec2::new(position.x + pos.0 as f32, position.y + pos.1 as f32)).collect();

    //     return Ok(positions);
    // }

    pub fn get_pixel(&self, position: Vec2, window_config: &WindowConfig) -> GameResult<UVec3> {
        let pixel_index = self.get_pixel_index(position, window_config)?;

        let pixel_r = self.buffer[pixel_index];
        let pixel_g = self.buffer[pixel_index + 1];
        let pixel_b = self.buffer[pixel_index + 2];
        
        let pixel = UVec3::new(pixel_r as u32, pixel_g as u32, pixel_b as u32);

        return Ok(pixel);
    }

    pub fn get_pixel_alpha(&mut self, position: Vec2, window_config: &WindowConfig) -> GameResult< u8> {
        let pixel_index = self.get_pixel_index(position, window_config)?;

        let alpha = self.buffer[pixel_index + 3];

        return Ok(alpha);
    }

    fn construct_buffer( width: usize, height: usize) -> GameResult<Vec<u8>> {
        let count = width * height * 4;
        let color = vec![0,0,0,1];
        let buffer: Vec<u8> = repeat(color).flat_map(|x| x).take(count).collect();

        return Ok(buffer);
    }

    fn calculate_pixel_list(radius: i32) -> Vec<(i32, i32)> {
        let mut pixel_list = vec![(0,0)];
    
        if radius > 0 {
            for y in -radius..radius + 1 {
                for x in -radius..radius + 1{
                    let t = (x, y);
                    if t == (0,0) { continue; }
    
                    pixel_list.push(t);
                }
            }
        }
    
        return pixel_list;
    }
}
