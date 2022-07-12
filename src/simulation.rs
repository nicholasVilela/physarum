use crate::Constants;
use std::{mem};
use ggez::{Context, GameResult};
use crate::{util, Agent, Trail, SimulationConfig, WindowConfig, Param, Storage, ComputeProgram, RenderProgram};


pub struct Simulation {
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_map_program: ComputeProgram,
    pub compute_agent_program: ComputeProgram,
    pub render_map_program: RenderProgram,
    pub render_agent_program: RenderProgram,

    pub constants_storage: Storage,
    pub agent_storage: Storage,
    pub param_storage: Storage,
    pub map_storages: Vec<Storage>,

    pub frame: usize,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let device = &ctx.gfx.wgpu().device;

        let constants_storage = Simulation::construct_constants_storage(device, &window_config, &config)?;
        let param_storage = Simulation::construct_param_storage(device)?;
        let agent_storage = Simulation::construct_agent_storage(device, &config)?;
        let map_storages = Simulation::construct_map_storages(device, &window_config)?;

        let compute_agent_program = Simulation::construct_compute_agent_program(ctx, &config, &constants_storage, &agent_storage, &param_storage, &map_storages)?;
        let compute_map_program = Simulation::construct_compute_map_program(ctx, &window_config, &map_storages, &constants_storage, &param_storage)?;
        let render_agent_program = Simulation::construct_render_agent_program(ctx)?; 
        let render_map_program = Simulation::construct_render_map_program(ctx)?;

        let simulation = Simulation { 
            config, 
            window_config,
            compute_map_program, 
            compute_agent_program, 
            render_map_program, 
            render_agent_program, 
            constants_storage, 
            agent_storage, 
            param_storage, 
            map_storages, 
            frame: 0 
        };

        return Ok(simulation);
    }

    pub fn reset(&mut self, _ctx: &mut Context) -> GameResult {
        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let frame = ctx.gfx.frame().clone();

        let delta_time = ctx.time.delta().as_secs_f32();
        let param = [Param { delta_time, frame: self.frame as u32 }];

        ctx.gfx.wgpu().queue.write_buffer(&self.param_storage.buffer, 0, bytemuck::cast_slice(&param));
        
        let command_encoder = ctx.gfx.commands().unwrap();

        command_encoder.push_debug_group("Update Map");
        {
            self.compute_map_program.process(command_encoder, self.frame)?;
        }
        command_encoder.pop_debug_group();

        command_encoder.push_debug_group("Update Agents");
        {
            self.compute_agent_program.process(command_encoder, self.frame)?;
        }
        command_encoder.pop_debug_group();

        command_encoder.push_debug_group("Render Map");
        {
            let window_area = (self.window_config.width * self.window_config.height) as u32 / 4;
            let color_attachments = &[wgpu::RenderPassColorAttachment {
                view: frame.wgpu().1,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }];

            self.render_map_program.process(command_encoder, color_attachments, vec![&self.map_storages[(self.frame + 1) % 2]], 0..1, 0..window_area)?;
        }
        command_encoder.pop_debug_group();

        command_encoder.push_debug_group("Render Agents");
        {
            let color_attachments = &[wgpu::RenderPassColorAttachment {
                view: frame.wgpu().1,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }];

            self.render_agent_program.process(command_encoder, color_attachments, vec![&self.agent_storage], 0..1, 0..self.config.agent_count as u32)?;
        }
        command_encoder.pop_debug_group();
        
        self.frame += 1;

        return Ok(());
    }

    fn construct_agents(simulation_config: &SimulationConfig) -> GameResult<Vec<Agent>> {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..simulation_config.agent_count {
            let agent = Agent::new(&mut rng)?;
            agents.push(agent);
        }

        return Ok(agents);
    }

    fn construct_trail_map(window_config: &WindowConfig) -> GameResult<Vec<Trail>> {
        let mut trail_map = Vec::new();

        for y in 0..window_config.height / 2 {
            for x in 0..window_config.width / 2 {
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

    fn construct_constants_storage(device: &wgpu::Device, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult<Storage> {
        let size = mem::size_of::<Constants>();
        let data = vec![Constants::new(window_config, simulation_config)?];
        let buffer = util::construct_buffer_init(device, "Constants Buffer", &data, wgpu::BufferUsages::UNIFORM)?;

        let storage = Storage { size, buffer };

        return Ok(storage);
    }

    fn construct_agent_storage(device: &wgpu::Device, simulation_config: &SimulationConfig) -> GameResult<Storage> {
        let size = mem::size_of::<Agent>() * simulation_config.agent_count as usize;
        let data = Simulation::construct_agents(simulation_config)?;
        let buffer = util::construct_buffer_init(device, "Agent Buffer", &data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE)?;

        let storage = Storage { size, buffer };

        return Ok(storage);
    }

    fn construct_param_storage(device: &wgpu::Device) -> GameResult<Storage> {
        let size = mem::size_of::<Param>();
        let data = vec![Param::default()];
        let buffer = util::construct_buffer_init(device, "Param Buffer", &data, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)?;

        let storage = Storage { size, buffer };

        return Ok(storage);
    }

    fn construct_map_storages(device: &wgpu::Device, window_config: &WindowConfig) -> GameResult<Vec<Storage>> {
        let size = (mem::size_of::<Trail>() * (window_config.width as usize / 2) * (window_config.height as usize / 2));
        let data = Simulation::construct_trail_map(window_config)?;

        let mut storages = Vec::new();

        for i in 0..2 {
            let buffer = util::construct_buffer_init(device, &format!("Map Buffer {}", i), &data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST)?;
            let storage = Storage { size, buffer };

            storages.push(storage);
        }

        return Ok(storages);
    }

    fn construct_compute_agent_program(ctx: &mut Context, simulation_config: &SimulationConfig, constants_storage: &Storage, agent_storage: &Storage, param_storage: &Storage, map_storages: &Vec<Storage>) -> GameResult<ComputeProgram> {
        let device = &ctx.gfx.wgpu().device;

        let compute_shader = util::construct_shader_module(device, "Compute Shader", include_str!("shaders/update_agents.wgsl"))?;

        let compute_bind_group_entries = &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(constants_storage.size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(param_storage.size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(agent_storage.size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
                },
                count: None,
            },
        ];
        let compute_bind_group_layout = util::construct_bind_group_layout(device, "Compute Agent Bind Group Layout", compute_bind_group_entries)?;

        let compute_pipeline_layout = util::construct_pipeline_layout(device, "Compute Agent Pipeline Layout", &vec![&compute_bind_group_layout], &vec![])?;
        let compute_pipeline = util::construct_compute_pipeline(device, "Compute Agent Pipeline", Some(&compute_pipeline_layout), &compute_shader, "main")?;


        let mut compute_bind_groups = Vec::new();

        for i in 0..2 {
            let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&format!("Compute Agent Bind Group {}", i)),
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: constants_storage.buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: param_storage.buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: agent_storage.buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: map_storages[i].buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: map_storages[(i + 1) % 2].buffer.as_entire_binding(),
                    },
                ],
            });

            compute_bind_groups.push(compute_bind_group);
        }

        let work_group_count = (simulation_config.agent_count as f32 / 32.0).ceil() as u32;
        let compute_agent_program = ComputeProgram::new(compute_pipeline, compute_bind_groups, (work_group_count, 1, 1))?;

        return Ok(compute_agent_program);
    }

    fn construct_compute_map_program(ctx: &mut Context, window_config: &WindowConfig, map_storages: &Vec<Storage>, constants_storage: &Storage, param_storage: &Storage) -> GameResult<ComputeProgram> {
        let device = &ctx.gfx.wgpu().device;

        let compute_map_shader = util::construct_shader_module(device, "Compute Map Shader", include_str!("shaders/update_map.wgsl"))?;

        let compute_map_bind_group_entries = &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(constants_storage.size as _),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(param_storage.size as _),
                },
                count: None,
            },
        ];

        let compute_map_bind_group_layout = util::construct_bind_group_layout(device, "Compute Map Bind Group Layout", compute_map_bind_group_entries)?;

        let compute_map_pipeline_layout = util::construct_pipeline_layout(device, "Compute Map Pipeline Layout", &vec![&compute_map_bind_group_layout], &vec![])?;
        let compute_map_pipeline = util::construct_compute_pipeline(device, "Compute Map Pipeline", Some(&compute_map_pipeline_layout), &compute_map_shader, "main")?;

        let mut compute_map_bind_groups = Vec::new();

        for i in 0..2 {
            let compute_map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Compute Map Bind Group"),
                layout: &compute_map_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: map_storages[i].buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: map_storages[(i + 1) % 2].buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: constants_storage.buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: param_storage.buffer.as_entire_binding(),
                    },
                ],
            });

            compute_map_bind_groups.push(compute_map_bind_group);
        }

        let work_group_count = ((window_config.width * window_config.height) as f32 / 32.0) as u32;
        let compute_map_program = ComputeProgram::new(compute_map_pipeline, compute_map_bind_groups, (work_group_count, 1, 1))?;

        return Ok(compute_map_program);
    }

    fn construct_render_agent_program(ctx: &mut Context) -> GameResult<RenderProgram> {
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

        let render_agent_program = RenderProgram::new(render_pipeline, vec![])?;

        return Ok(render_agent_program);
    }

    fn construct_render_map_program(ctx: &mut Context) -> GameResult<RenderProgram> {
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

        let render_map_program = RenderProgram::new(render_map_pipeline, vec![])?;

        return Ok(render_map_program);
    } 
}
