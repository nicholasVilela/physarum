use std::{borrow::Cow, mem};
use ggez::{Context, GameResult, graphics::{Canvas, Color, LinearColor}};
use crate::{util, Agent, Trail, SimulationConfig, WindowConfig, SpeciesConfig, Species, config, SimulationParams};
use wgpu::util::DeviceExt;


pub struct Simulation {
    pub trail: Trail,
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_bind_group: wgpu::BindGroup,
    pub agent_buffer: wgpu::Buffer,
    pub simulation_params_buffer: wgpu::Buffer,

    pub frame: usize,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let agents = Simulation::construct_agents(&config, &window_config)?;
        let trail = Trail::new(ctx, &window_config)?;

        let (compute_pipeline, compute_bind_group, agent_buffer, simulation_params_buffer) = Simulation::construct_compute_shader(ctx, &window_config,  &vec![config], agents)?;
        let (render_pipeline) = Simulation::construct_render_shader(ctx)?; 

        let simulation = Simulation { trail, config, window_config, compute_pipeline, compute_bind_group, render_pipeline, agent_buffer, simulation_params_buffer ,frame: 0 };

        return Ok(simulation);
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult {
        let trail = Trail::new(ctx, &self.window_config)?;

        self.trail = trail;

        return Ok(());
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let frame = ctx.gfx.frame().clone();

        let delta_time = ctx.time.delta().as_secs_f32();
        let simulation_params = [SimulationParams { delta_time, frame: self.frame as u32 }];

        ctx.gfx.wgpu().queue.write_buffer(&self.simulation_params_buffer, 0, bytemuck::cast_slice(&simulation_params));
        let command_encoder = ctx.gfx.commands().unwrap();

        {
            let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            pass.set_pipeline(&self.compute_pipeline);
            pass.set_bind_group(0, &self.compute_bind_group, &[]);
            let work_group_count = (self.config.agent_count as f32 / 32.0).ceil() as u32;
            pass.dispatch(work_group_count, 1, 1);
        }

        {
            let mut pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: frame.wgpu().1,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            }); 

            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.agent_buffer.slice(..));
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

    fn construct_compute_shader(ctx: &mut Context, window_config: &WindowConfig, simulation_config: &Vec<SimulationConfig>, agents: Vec<Agent>) -> GameResult<(wgpu::ComputePipeline, wgpu::BindGroup, wgpu::Buffer, wgpu::Buffer)> {
        let device = &ctx.gfx.wgpu().device;

        let compute_shader = util::construct_shader_module(device, "Compute Shader", include_str!("shaders/update_agents.wgsl"))?;

        let agent_bufsize = mem::size_of::<Agent>() * simulation_config[0].agent_count as usize;
        let agent_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Agent Buffer"),
            contents: bytemuck::cast_slice(&agents),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });

        let map: Vec<f32> = vec![0.0; (window_config.width * window_config.height) as usize];
        let map_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Buffer"),
            contents: bytemuck::cast_slice(&map),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let simulation_params = [
            SimulationParams {
                delta_time: 0.004,
                frame: 0,
            }
        ];
        let simulation_params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simulation Params Buffer"),
            contents: bytemuck::cast_slice(&simulation_params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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
                        min_binding_size: wgpu::BufferSize::new(mem::size_of::<SimulationParams>() as _),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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

        let compute_pipeline_layout = util::construct_pipeline_layout(device, "Compute Pipeline Layout", &vec![&compute_bind_group_layout], &vec![])?;

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
                    resource: simulation_params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: agent_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: map_buffer.as_entire_binding(),
                },
            ],
        });

        return Ok((compute_pipeline, bind_group, agent_buffer, simulation_params_buffer));
    }

    fn construct_render_shader(ctx: &mut Context) -> GameResult<(wgpu::RenderPipeline)> {
        let device = &ctx.gfx.wgpu().device;

        let render_shader = util::construct_shader_module(device, "Render Shader", include_str!("shaders/render_agents.wgsl"))?;
        let pipeline_layout = util::construct_pipeline_layout(device, "Render Pipeline Layout", &vec![], &vec![])?;
        let render_pipeline = util::construct_render_pipeline(
            device, 
            "Render Pipeline", 
            Some(&pipeline_layout), 
            wgpu::VertexState {
                module: &render_shader,
                entry_point: "main_vs",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 16,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x4],
                    },
                ],
            },
            Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "main_fs",
                targets: &[wgpu::ColorTargetState {
                        format: ctx.gfx.surface_format(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            None,
            None,
            wgpu::MultisampleState::default(),
            wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
        )?;

        return Ok((render_pipeline));
    }
}
