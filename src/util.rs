use std::num::NonZeroU32;
use std::{borrow::Cow};
use ggez::{GameResult};


pub fn construct_shader_module(
    device: &wgpu::Device, 
    label: &str, 
    path: &str, 
) -> GameResult<wgpu::ShaderModule> {
    let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(path)),
    });

    return Ok(shader_module);
}

pub fn construct_pipeline_layout(
    device: &wgpu::Device, 
    label: &str, 
    bind_group_layouts: &Vec<&wgpu::BindGroupLayout>, 
    push_constant_ranges: &Vec<wgpu::PushConstantRange>,
) -> GameResult<wgpu::PipelineLayout> {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts,
        push_constant_ranges,
    });

    return Ok(pipeline_layout);
}

pub fn construct_render_pipeline(
    device: &wgpu::Device, 
    label: &str, 
    layout: Option<&wgpu::PipelineLayout>, 
    vertex: wgpu::VertexState, 
    fragment: Option<wgpu::FragmentState>, 
    multiview: Option<NonZeroU32>, 
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    primitive: wgpu::PrimitiveState,
) -> GameResult<wgpu::RenderPipeline> {
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout,
        vertex,
        fragment,
        multiview,
        depth_stencil,
        multisample,
        primitive,
    });

    return Ok(render_pipeline);
}

pub fn construct_vertex_state<'a>(module: &'a wgpu::ShaderModule, entry_point: &'a str, buffers: &'static Vec<wgpu::VertexBufferLayout>) -> GameResult<wgpu::VertexState<'a>> {
    let vertex_state = wgpu::VertexState {
        module,
        entry_point,
        buffers,
    };

    return Ok(vertex_state);
}
