use ggez::{GameResult};


pub struct ComputeProgram {
    pipeline: wgpu::ComputePipeline,
    bind_groups: Vec<wgpu::BindGroup>,
    dispatch_group: (u32, u32, u32),
}

impl ComputeProgram {
    pub fn new(pipeline: wgpu::ComputePipeline, bind_groups: Vec<wgpu::BindGroup>, dispatch_group: (u32, u32, u32)) -> GameResult<ComputeProgram> {
        let compute_program = ComputeProgram { pipeline, bind_groups, dispatch_group, };

        return Ok(compute_program);
    }

    pub fn process(&mut self, command_encoder: &mut wgpu::CommandEncoder) -> GameResult {
        let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

        pass.set_pipeline(&self.pipeline);
        self.bind_groups.iter().enumerate().for_each(|(i, group)| pass.set_bind_group(i as u32, group, &[]));
        pass.dispatch(self.dispatch_group.0, self.dispatch_group.1, self.dispatch_group.2);

        return Ok(());
    }
}
