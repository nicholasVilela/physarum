use ggez::{GameResult};


pub struct ComputeProgram {
    pipeline: wgpu::ComputePipeline,
    bind_groups: Vec<wgpu::BindGroup>,
    work_group_count: u32,
}

impl ComputeProgram {
    pub fn new(pipeline: wgpu::ComputePipeline, bind_groups: Vec<wgpu::BindGroup>, work_group_count: u32) -> GameResult<ComputeProgram> {
        let compute_program = ComputeProgram { pipeline, bind_groups, work_group_count };

        return Ok(compute_program);
    }

    pub fn process(&mut self, command_encoder: &mut wgpu::CommandEncoder) -> GameResult {
        let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        pass.set_pipeline(&self.pipeline);
        self.bind_groups.iter().enumerate().for_each(|(i, group)| pass.set_bind_group(i as u32, group, &[]));
        pass.dispatch(self.work_group_count, 1, 1);

        return Ok(());
    }
}
