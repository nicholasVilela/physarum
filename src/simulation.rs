use std::{borrow::Cow, mem};
use ggez::{Context, GameResult, graphics::{DrawParam, Canvas, Color, LinearColor}};
use crate::{Agent, Trail, SimulationConfig, WindowConfig, SpeciesConfig, Species, config};


pub struct Simulation {
    pub agents: Vec<Agent>,
    pub trail: Trail,
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_bind_group: wgpu::BindGroup,
    pub render_bind_group: wgpu::BindGroup,
    // pub agent_buffer: wgpu::Buffer,
    // pub trail_buffer: wgpu::Buffer,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let agents = Simulation::construct_agents(&config, &window_config)?;
        let trail = Trail::new(ctx, &window_config)?;

        let (compute_pipeline, compute_bind_group, agent_buffer, simulation_config_buffer) = Simulation::construct_compute_shader(ctx, &config)?;
        let (render_pipeline, render_bind_group) = Simulation::construct_render_shader(ctx, &config)?; 

        let simulation = Simulation { agents, trail, config, window_config, compute_pipeline, compute_bind_group, render_pipeline, render_bind_group };

        return Ok(simulation);
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult {
        let agents = Simulation::construct_agents(&self.config, &self.window_config)?;
        let trail = Trail::new(ctx, &self.window_config)?;

        self.agents = agents;
        self.trail = trail;

        return Ok(());
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let device = &ctx.gfx.wgpu().device;
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            pass.set_pipeline(&self.compute_pipeline);
            pass.set_bind_group(0, &self.compute_bind_group, &[]);
            pass.dispatch(32, 1, 1);
        }

        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let device = &ctx.gfx.wgpu().device;
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let frame = ctx.gfx.frame().clone();

        {
            let mut pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: frame.wgpu().1,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(
                            LinearColor::from(Color::new(0.1, 0.1, 0.1, 1.0))
                                .into(),
                        ),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.render_pipeline);
            pass.set_bind_group(0, &self.render_bind_group, &[]);
            pass.draw(0..6, 0..500);
        }

        return Ok(());
    }

    // pub fn update(&mut self, ctx: &mut Context) -> GameResult {
    //     let delta = ctx.time.delta();

    //     for agent in &mut self.agents {
    //         agent.update(delta, &self.window_config, &mut self.trail)?;

    //         // if self.config.render_agents { 
    //         //     let draw_param = DrawParam::new()
    //         //         .dest(Point2 { x: agent.position.x, y: agent.position.y });
    //         //     self.agent_meshbatch.push(draw_param);
    //         // }
    //     }
    //     self.trail.update(ctx, &self.window_config, &self.config)?;

    //     return Ok(());
    // }

    // pub fn render(&mut self, canvas: &mut Canvas) -> GameResult {
    //     canvas.draw(&self.trail.map, DrawParam::default());

    //     return Ok(());
    // }

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

    fn construct_compute_shader(ctx: &mut Context, simulation_config: &SimulationConfig) -> GameResult<(wgpu::ComputePipeline, wgpu::BindGroup, wgpu::Buffer, wgpu::Buffer)> {
        let device = &ctx.gfx.wgpu().device;

        let compute_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/simulation.wgsl")))
        });

        let agent_bufsize = mem::size_of::<Agent>() * simulation_config.agent_count as usize;
        let agent_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Agent Buffer"),
            usage: wgpu::BufferUsages::STORAGE,
            size: agent_bufsize as _,
            mapped_at_creation: false,
        });

        let simulation_config_buffer_size = mem::size_of::<SimulationConfig>() as u64;
        let simulation_config_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Simulation Config Buffer"),
            usage: wgpu::BufferUsages::UNIFORM,
            size: simulation_config_buffer_size,
            mapped_at_creation: false,
        });

        let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(simulation_config_buffer_size),
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
                // wgpu::BindGroupLayoutEntry {
                //     binding: 2,
                //     visibility: wgpu::ShaderStages::COMPUTE,
                //     ty: wgpu::BindingType::StorageTexture {
                //         access: wgpu::StorageTextureAccess::WriteOnly,
                //         format: wgpu::TextureFormat::Rgba8UnormSrgb,
                //         view_dimension: wgpu::TextureViewDimension::D2,
                //     },
                //     count: None,
                // }
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: simulation_config_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: agent_buffer.as_entire_binding(),
                }
            ],
        });

        return Ok((compute_pipeline, bind_group, agent_buffer, simulation_config_buffer));
    }

    fn construct_render_shader(ctx: &mut Context, simulation_config: &SimulationConfig) -> GameResult<(wgpu::RenderPipeline, wgpu::BindGroup)> {
        let device = &ctx.gfx.wgpu().device;

        let render_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/render.wgsl")))
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Group Layout"),
            entries: &[
                // wgpu::BindGroupLayoutEntry {
                //     binding: 0,
                //     visibility: wgpu::ShaderStages::VERTEX,
                //     ty: wgpu::BindingType::Buffer {
                //         ty: wgpu::BufferBindingType::Uniform,
                //         has_dynamic_offset: false,
                //         min_binding_size: wgpu::BufferSize::new(agent_bufsize as _),
                //     },
                //     count: None,
                // },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 6 * 4,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                    }
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                        format: ctx.gfx.surface_format(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                }]
            }),
            multiview: None,
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: None,
            primitive: wgpu::PrimitiveState::default(),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &bind_group_layout,
            entries: &[],
        });

        return Ok((render_pipeline, bind_group));
    }
}
