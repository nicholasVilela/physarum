use std::{borrow::Cow, mem};
use ggez::{Context, GameResult, graphics::{Canvas, Color, LinearColor}};
use crate::{Agent, Trail, SimulationConfig, WindowConfig, SpeciesConfig, Species, config};
use wgpu::util::DeviceExt;


pub struct Simulation {
    pub trail: Trail,
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_bind_groups: Vec<wgpu::BindGroup>,
    pub agent_buffers: Vec<wgpu::Buffer>,

    pub frame: usize,
    // pub render_bind_group: wgpu::BindGroup,
    // pub trail_buffer: wgpu::Buffer,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let agents = Simulation::construct_agents(&config, &window_config)?;
        let trail = Trail::new(ctx, &window_config)?;

        let (compute_pipeline, compute_bind_groups, agent_buffers) = Simulation::construct_compute_shader(ctx, &vec![config], agents)?;
        let (render_pipeline) = Simulation::construct_render_shader(ctx)?; 

        let simulation = Simulation { trail, config, window_config, compute_pipeline, compute_bind_groups, render_pipeline, agent_buffers ,frame: 0 };

        return Ok(simulation);
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult {
        let agents = Simulation::construct_agents(&self.config, &self.window_config)?;
        let trail = Trail::new(ctx, &self.window_config)?;

        self.trail = trail;

        return Ok(());
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let device = &ctx.gfx.wgpu().device;
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            pass.set_pipeline(&self.compute_pipeline);
            pass.set_bind_group(0, &self.compute_bind_groups[self.frame % 2], &[]);
            pass.dispatch(32, 1, 1);
        }

        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let device = &ctx.gfx.wgpu().device;
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let frame = ctx.gfx.frame().clone();
        // let command_encoder = ctx.gfx.commands().unwrap();

        {
            let mut pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: frame.wgpu().1,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(
                            LinearColor::from(Color::new(1.0, 0.1, 0.1, 1.0))
                                .into(),
                        ),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.render_pipeline);
            // pass.set_bind_group(0, &self.render_bind_group, &[]);
            pass.set_vertex_buffer(0, self.agent_buffers[(self.frame + 1) % 2].slice(..));
            pass.draw(0..1, 0..self.config.agent_count as u32);
        }

        self.frame += 1;

        return Ok(());
    }

    fn construct_agents(simulation_config: &SimulationConfig, window_config: &WindowConfig) -> GameResult<Vec<Agent>> {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        let species_config = config::load::<SpeciesConfig>(&Species::A.to_string())?;

        for _ in 0..simulation_config.agent_count {
            let agent = Agent::new(Species::A, species_config, window_config, &simulation_config, &mut rng)?;
            agents.push(agent);
        }

        return Ok(agents);
    }

    fn construct_compute_shader(ctx: &mut Context, simulation_config: &Vec<SimulationConfig>, agents: Vec<Agent>) -> GameResult<(wgpu::ComputePipeline, Vec<wgpu::BindGroup>, Vec<wgpu::Buffer>)> {
        let device = &ctx.gfx.wgpu().device;

        let compute_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/simulation.wgsl")))
        });

        let agent_bufsize = mem::size_of::<Agent>() * simulation_config[0].agent_count as usize;
        let mut agent_buffers = Vec::<wgpu::Buffer>::new();
        for i in 0..2 {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Agent Buffer {}", i)),
                contents: bytemuck::cast_slice(&agents),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

            agent_buffers.push(buffer);
        }

        let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(agent_bufsize as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(agent_bufsize as _),
                    },
                    count: None,
                },
            ],
        });

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&compute_bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        let mut bind_groups = Vec::<wgpu::BindGroup>::new();
        for i in 0..2 {
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Compute Bind Group"),
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: agent_buffers[i].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: agent_buffers[(i + 1) % 2].as_entire_binding(),
                    }
                ],
            });

            bind_groups.push(bind_group);
        }

        return Ok((compute_pipeline, bind_groups, agent_buffers));
    }

    fn construct_render_shader(ctx: &mut Context) -> GameResult<(wgpu::RenderPipeline)> {
        let device = &ctx.gfx.wgpu().device;

        let render_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/render.wgsl")))
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "main_vs",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32],
                    }
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "main_fs",
                targets: &[wgpu::ColorTargetState {
                        format: ctx.gfx.surface_format(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            multiview: None,
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
            primitive: wgpu::PrimitiveState::default(),
        });

        return Ok((render_pipeline));
    }
}
