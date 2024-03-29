use std::ops::Range;
use ggez::{GameResult};
use crate::{Storage};


pub struct RenderProgram {
    pipeline: wgpu::RenderPipeline,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl RenderProgram {
    pub fn new(pipeline: wgpu::RenderPipeline, bind_groups: Vec<wgpu::BindGroup>) -> GameResult<RenderProgram> {
        let render_program = RenderProgram { pipeline, bind_groups };

        return Ok(render_program);
    }

    pub fn process(&self, command_encoder: &mut wgpu::CommandEncoder, color_attachments: &[wgpu::RenderPassColorAttachment], vertex_storages: Vec<&Storage>, vertex_range: Range<u32>, instance_range: Range<u32>, frame: usize) -> GameResult {
        let mut pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments,
            depth_stencil_attachment: None,
        }); 

        pass.set_pipeline(&self.pipeline);
        // pass.set_vertex_buffer(0, vertex_storages[0].buffer.slice(..));
        vertex_storages.iter().enumerate().for_each(|(i, storage)| pass.set_vertex_buffer(i as u32, storage.buffer.slice(..)));
        // pass.set_bind_group(0, &self.bind_groups[0], &[]);
        self.bind_groups.iter().enumerate().for_each(|(i, group)| pass.set_bind_group(i as u32, group, &[]));
        pass.draw(vertex_range, instance_range);
        
        return Ok(());
    }
}
