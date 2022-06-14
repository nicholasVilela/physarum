use std::iter::repeat;
use ggez::{graphics::{Image, MeshBatch, MeshBuilder, DrawMode, Color}, Context, GameResult};
use crate::{WindowConfig, Vec2, FVec2};


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

    pub fn update(&mut self, ctx: &mut Context, window_config: &WindowConfig) -> GameResult {
        let width = window_config.width as u16;
        let height = window_config.height as u16;

        self.map = Image::from_rgba8(ctx, width, height, &self.buffer)?;

        return Ok(());
    }

    pub fn update_pixel(&mut self, position: FVec2, window_config: &WindowConfig) -> GameResult {
        let pixel_index = (position.y * (window_config.width - 1.0)) + position.x;
        let pixel_index = pixel_index as usize;
        let pixel_index = pixel_index * 4;
        
        println!("{:?}", pixel_index);

        self.buffer[pixel_index] = 255;
        // self.buffer[pixel_index + 1] = 255;
        // self.buffer[pixel_index + 2] = 255;

        return Ok(());
    }

    fn construct_buffer( width: usize, height: usize) -> GameResult<Vec<u8>> {
        let count = width * height * 4;
        let color = vec![0,0,0,255];
        let buffer: Vec<u8> = repeat(color).flat_map(|x| x).take(count).collect();

        return Ok(buffer);
    }
}