use std::num::{NonZeroU8, NonZeroUsize};
use std::iter::repeat;
use ggez::{graphics::{Image, ImageFormat}, Context, GameResult};
use crate::{WindowConfig, SpeciesConfig, SimulationConfig};
use stackblur::blur;
use glam::{UVec3, Vec2};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Trail {
    pub position: [f32; 2],
    pub value: f32,
}

impl Trail {
    pub fn new(position: [f32; 2], value: f32) -> GameResult<Trail> {
        let trail = Trail { position, value };

        return Ok(trail);
    }
}

// pub struct Trail {
//     pub map: Image,
//     pub buffer: Vec<u8>,
// }

// impl Trail {
//     pub fn new(ctx: &mut Context, window_config: &WindowConfig) -> GameResult<Trail> {
//         let width = window_config.width as u32;
//         let height = window_config.height as u32;

//         let buffer: Vec<u8>= Trail::construct_buffer(width as usize, height as usize)?;
//         let map = Image::from_pixels(ctx, &buffer, ImageFormat::Rgba8UnormSrgb, width, height);

//         let trail = Trail { map, buffer };

//         return Ok(trail);
//     }

//     pub fn update(&mut self, ctx: &mut Context, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult {
//         let width = window_config.width as u32;
//         let height = window_config.height as u32;

//         if simulation_config.blur_strength > 0 {
//             let mut pixels = unsafe { self.buffer.align_to_mut::<u32>().1 };
//             blur(&mut pixels, NonZeroUsize::new(width as usize).unwrap(), NonZeroUsize::new(height as usize).unwrap(), NonZeroU8::new(simulation_config.blur_strength).unwrap());
//         }

//         if simulation_config.evaporation_speed > 0 {
//             for y in 0..window_config.height {
//                 for x in 0..window_config.width {
//                     let position = Vec2::new(x as f32, y as f32);
    
//                     self.evaporate_pixel(position, window_config, simulation_config)?;
//                 }
//             }
//         }

//         self.map = Image::from_pixels(ctx, &self.buffer, ImageFormat::Rgba8UnormSrgb, width, height);

//         return Ok(());
//     }

//     pub fn update_pixel(&mut self, position: Vec2, species_config: &SpeciesConfig, window_config: &WindowConfig) -> GameResult {
//         let pixel_index = self.get_pixel_index(position, window_config)?;

//         self.buffer[pixel_index] = (species_config.color.r * 255.0) as u8;
//         self.buffer[pixel_index + 1] = (species_config.color.g * 255.0) as u8;
//         self.buffer[pixel_index + 2] = (species_config.color.b * 255.0) as u8;

//         return Ok(());
//     }

//     fn evaporate_color(&mut self, pixel_index: usize, evaporation_speed: u8) -> GameResult {
//         if self.buffer[pixel_index] == 0 { return Ok(()); }

//         if self.buffer[pixel_index] < evaporation_speed { 
//             self.buffer[pixel_index] = 0;
//             return Ok(()); 
//         } 

//         self.buffer[pixel_index] -= evaporation_speed;
        
//         return Ok(());
//     }

//     pub fn evaporate_pixel(&mut self, position: Vec2, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult {
//         let pixel_index = self.get_pixel_index(position, window_config)?;
//         let evaporation_speed = simulation_config.evaporation_speed;

//         for i in 0..3 {
//             self.evaporate_color(pixel_index + i, evaporation_speed)?;
//         }

//         return Ok(());
//     }

//     pub fn get_pixel_index(&self, position: Vec2, window_config: &WindowConfig) -> GameResult<usize> {
//         let pixel_index = position.y as usize;
//         let pixel_index = pixel_index * window_config.width as usize + position.x as usize;
//         let mut pixel_index = pixel_index * 4;

//         let max_pixel_index = ((window_config.width * window_config.height * 4)) as usize - 4;

//         if pixel_index > max_pixel_index {
//             pixel_index = max_pixel_index;
//         }

//         return Ok(pixel_index);
//     }

//     pub fn get_pixel(&self, position: Vec2, window_config: &WindowConfig) -> GameResult<UVec3> {
//         let pixel_index = self.get_pixel_index(position, window_config)?;

//         let pixel_r = self.buffer[pixel_index];
//         let pixel_g = self.buffer[pixel_index + 1];
//         let pixel_b = self.buffer[pixel_index + 2];
        
//         let pixel = UVec3::new(pixel_r as u32, pixel_g as u32, pixel_b as u32);

//         return Ok(pixel);
//     }

//     fn construct_buffer( width: usize, height: usize) -> GameResult<Vec<u8>> {
//         let count = width * height * 4;
//         let color = vec![0,0,0,1];
//         let buffer: Vec<u8> = repeat(color).flat_map(|x| x).take(count).collect();

//         return Ok(buffer);
//     }
// }
