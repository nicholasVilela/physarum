use std::{mem};
use ggez::{Context, GameResult};
use crate::{util, Agent, Trail, SimulationConfig, WindowConfig, SpeciesConfig, Species, config, SimulationParams};


pub struct Simulation {
    // pub trail: Trail,
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_map_pipeline: wgpu::RenderPipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_bind_group: wgpu::BindGroup,
    pub agent_buffer: wgpu::Buffer,
    pub map_buffer: wgpu::Buffer,
    pub simulation_params_buffer: wgpu::Buffer,

    pub frame: usize,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let agents = Simulation::construct_agents(&config, &window_config)?;
        // let trail = Trail::new(ctx, &window_config)?;

        let (compute_pipeline, compute_bind_group, agent_buffer, simulation_params_buffer, map_buffer) = Simulation::construct_compute_shader(ctx, &window_config,  &vec![config], agents)?;
        let render_pipeline = Simulation::construct_render_shader(ctx)?; 
        let render_map_pipeline = Simulation::construct_render_map_shader(ctx)?;

        let simulation = Simulation { config, window_config, compute_pipeline, compute_bind_group, render_pipeline, render_map_pipeline, agent_buffer, simulation_params_buffer, map_buffer, frame: 0 };

        return Ok(simulation);
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let frame = ctx.gfx.frame().clone();

        let window_area = self.window_config.width * self.window_config.height;

        let delta_time = ctx.time.delta().as_secs_f32();
        let simulation_params = [SimulationParams { delta_time, frame: self.frame as u32 }];

        ctx.gfx.wgpu().queue.write_buffer(&self.simulation_params_buffer, 0, bytemuck::cast_slice(&simulation_params));
        let command_encoder = ctx.gfx.commands().unwrap();

        // // Update Agents
        {
            let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            pass.set_pipeline(&self.compute_pipeline);
            pass.set_bind_group(0, &self.compute_bind_group, &[]);
            let work_group_count = (self.config.agent_count as f32 / 32.0).ceil() as u32;
            pass.dispatch(work_group_count, 1, 1);
        }

        // Render Map
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

            pass.set_pipeline(&self.render_map_pipeline);
            pass.set_vertex_buffer(0, self.map_buffer.slice(..));
            pass.draw(0..1, 0..window_area as u32);
        }

        // // Render Agents
        {
            let mut pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: frame.wgpu().1,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        // load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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

    fn construct_trail_map(window_config: &WindowConfig) -> GameResult<Vec<Trail>> {
        let mut trail_map = Vec::new();

        for y in 0..window_config.height {
            for x in 0..window_config.width {
                let xb = (window_config.width / 2) as f32;
                let yb = (window_config.height / 2) as f32;
                
                let x_pos = (x as f32 - xb) / xb;
                let y_pos = (y as f32 - yb) / yb;

                let trail = Trail::new([x_pos, y_pos], 0.0)?;

                trail_map.push(trail);
            }
        }

        return Ok(trail_map);
    }

    fn construct_compute_shader(ctx: &mut Context, window_config: &WindowConfig, simulation_config: &Vec<SimulationConfig>, agents: Vec<Agent>) -> GameResult<(wgpu::ComputePipeline, wgpu::BindGroup, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer)> {
        let device = &ctx.gfx.wgpu().device;

        let compute_shader = util::construct_shader_module(device, "Compute Shader", include_str!("shaders/update_agents.wgsl"))?;

        let agent_size = mem::size_of::<Agent>() * simulation_config[0].agent_count as usize;
        let agent_buffer = util::construct_buffer_init(device, "Agent Buffer", &agents, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE)?;

        let map = Simulation::construct_trail_map(window_config)?;
        let map_size = mem::size_of::<Trail>() * window_config.width as usize * window_config.height as usize;
        // let map_size = 12 * window_config.width as usize * window_config.height as usize;
        let map_buffer = util::construct_buffer_init(device, "Map Buffer", &map, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE)?;

        let simulation_params = vec![SimulationParams::default()];
        let simulation_params_buffer = util::construct_buffer_init(device, "Simulation Params Buffer", &simulation_params, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)?;

        let compute_bind_group_entries = &[
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
                    min_binding_size: wgpu::BufferSize::new(agent_size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(map_size as _),
                },
                count: None,
            },
        ];
        let compute_bind_group_layout = util::construct_bind_group_layout(device, "Compute Bind Group Layout", compute_bind_group_entries)?;

        let compute_pipeline_layout = util::construct_pipeline_layout(device, "Compute Pipeline Layout", &vec![&compute_bind_group_layout], &vec![])?;
        let compute_pipeline = util::construct_compute_pipeline(device, "Compute Pipeline", Some(&compute_pipeline_layout), &compute_shader, "main")?;

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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

        return Ok((compute_pipeline, compute_bind_group, agent_buffer, simulation_params_buffer, map_buffer));
    }

    fn construct_render_shader(ctx: &mut Context) -> GameResult<wgpu::RenderPipeline> {
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
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                    },
                ],
            },
            Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "main_fs",
                targets: &[wgpu::ColorTargetState {
                        format: ctx.gfx.surface_format(),
                        blend: Some(wgpu::BlendState{
                            color: wgpu::BlendComponent{
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,},
                            alpha: wgpu::BlendComponent::OVER
                        }),
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

        return Ok(render_pipeline);
    }

    fn construct_render_map_shader(ctx: &mut Context) -> GameResult<wgpu::RenderPipeline> {
        let device = &ctx.gfx.wgpu().device;

        let render_shader = util::construct_shader_module(device, "Render Map Shader", include_str!("shaders/render_map.wgsl"))?;
        let pipeline_layout = util::construct_pipeline_layout(device, "Render Map Pipeline Layout", &vec![], &vec![])?;
        let render_map_pipeline = util::construct_render_pipeline(
            device, 
            "Render Map Pipeline", 
            Some(&pipeline_layout), 
            wgpu::VertexState {
                module: &render_shader,
                entry_point: "main_vs",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 16,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32],
                    },
                ],
            },
            Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "main_fs",
                targets: &[wgpu::ColorTargetState {
                        format: ctx.gfx.surface_format(),
                        blend: Some(wgpu::BlendState{
                            color: wgpu::BlendComponent{
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,},
                            alpha: wgpu::BlendComponent::OVER
                        }),
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

        return Ok(render_map_pipeline);
    } 
}
