use std::{mem, fs, fs::DirEntry, num::{NonZeroU32, NonZeroU64}};
use ggez::{Context, GameResult};
use crate::{util, Agent, Trail, Constants, SimulationConfig, WindowConfig, Param, Storage, ComputeProgram, RenderProgram, config, SpeciesConfig, Species};
use walkdir::WalkDir;


pub fn construct_species(species_count: &mut usize) -> GameResult<Vec<Species>> {
  let mut data = Vec::new();
  let path = format!("{}/config/species/", env!("CARGO_MANIFEST_DIR"));
  let mut dir: Vec<DirEntry> = fs::read_dir(path)?.map(|r| r.unwrap()).collect();

  dir.sort_by_key(|d| d.path());

  for file in dir {            
      let mut name = file.file_name().into_string().unwrap();
      
      let offset = name.find('.').unwrap_or(name.len());
      name.drain(offset..);
      
      let config = config::load::<SpeciesConfig>(&format!("species/{}", &name))?;
      let species = Species::new(config)?;
      
      *species_count += 1;
      
      data.push(species);
  }

  return Ok(data);
}

pub fn construct_agents(simulation_config: &SimulationConfig) -> GameResult<Vec<Agent>> {
  let mut agents = Vec::new();
  let mut rng = rand::thread_rng();

  let path = "./config/species";
  let species_count = WalkDir::new(path).into_iter().count() as u32 - 1;

  for _ in 0..simulation_config.agent_count {
      let agent = Agent::default()?
          .random_angle(&mut rng)?
          .random_position(&mut rng)?
          // .with_species(0)?;
          .random_species(&mut rng, species_count)?;
      agents.push(agent);
  }

  return Ok(agents);
}

pub fn construct_trail_map(window_config: &WindowConfig) -> GameResult<Vec<Trail>> {
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

pub fn construct_constants_storage(device: &wgpu::Device, window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult<Storage> {
  let size = mem::size_of::<Constants>();
  let data = vec![Constants::new(window_config, simulation_config)?];
  let buffer = util::construct_buffer_init(device, "Constants Buffer", &data, wgpu::BufferUsages::UNIFORM)?;

  let storage = Storage { size, buffer };

  return Ok(storage);
}

pub fn construct_agent_storage(device: &wgpu::Device, simulation_config: &SimulationConfig) -> GameResult<Storage> {
  let size = mem::size_of::<Agent>() * simulation_config.agent_count as usize;
  let data = construct_agents(simulation_config)?;
  let buffer = util::construct_buffer_init(device, "Agent Buffer", &data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST)?;

  let storage = Storage { size, buffer };

  return Ok(storage);
}

pub fn construct_param_storage(device: &wgpu::Device) -> GameResult<Storage> {
  let size = mem::size_of::<Param>();
  let data = vec![Param::default()];
  let buffer = util::construct_buffer_init(device, "Param Buffer", &data, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)?;

  let storage = Storage { size, buffer };

  return Ok(storage);
}

pub fn construct_map_storages(device: &wgpu::Device, window_config: &WindowConfig) -> GameResult<Vec<Storage>> {
  let size = mem::size_of::<Trail>() * window_config.width as usize * window_config.height as usize;
  let data = construct_trail_map(window_config)?;

  let mut storages = Vec::new();

  for i in 0..2 {
      let buffer = util::construct_buffer_init(device, &format!("Map Buffer {}", i), &data, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST)?;
      let storage = Storage { size, buffer };

      storages.push(storage);
  }

  return Ok(storages);
}

pub fn construct_species_storage(device: &wgpu::Device) -> GameResult<Storage> {
  let mut species_count = 0;
  let data: Vec<Species> = construct_species(&mut species_count)?;

  let size = mem::size_of::<Species>() * species_count as usize;
  let buffer = util::construct_buffer_init(device, &format!("Species Buffer"), &data, wgpu::BufferUsages::STORAGE)?;
  
  let storage = Storage{ size, buffer };
  
  return Ok(storage);
}

pub fn construct_compute_agent_program(ctx: &mut Context, simulation_config: &SimulationConfig, constants_storage: &Storage, agent_storage: &Storage, param_storage: &Storage, map_storages: &Vec<Storage>, species_storage: &Storage) -> GameResult<ComputeProgram> {
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
              ty: wgpu::BufferBindingType::Storage { read_only: true },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(species_storage.size as _),
          },
          count: None,
      },
      wgpu::BindGroupLayoutEntry {
          binding: 3,
          visibility: wgpu::ShaderStages::COMPUTE,
          ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Storage { read_only: false },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(agent_storage.size as _),
          },
          count: None,
      },
      wgpu::BindGroupLayoutEntry {
          binding: 4,
          visibility: wgpu::ShaderStages::COMPUTE,
          ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Storage { read_only: true },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
          },
          count: None,
      },
      wgpu::BindGroupLayoutEntry {
          binding: 5,
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
                  resource: species_storage.buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 3,
                  resource: agent_storage.buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 4,
                  resource: map_storages[i].buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 5,
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

pub fn construct_compute_map_program(ctx: &mut Context, window_config: &WindowConfig, map_storages: &Vec<Storage>, constants_storage: &Storage, param_storage: &Storage) -> GameResult<ComputeProgram> {
  let device = &ctx.gfx.wgpu().device;

  let compute_map_shader = util::construct_shader_module(device, "Compute Map Shader", include_str!("shaders/update_map.wgsl"))?;

  let compute_map_bind_group_entries = &[
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
              ty: wgpu::BufferBindingType::Storage { read_only: true },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
          },
          count: None,
      },
      wgpu::BindGroupLayoutEntry {
          binding: 3,
          visibility: wgpu::ShaderStages::COMPUTE,
          ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Storage { read_only: false },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(map_storages[0].size as _),
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
                  resource: constants_storage.buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 1,
                  resource: param_storage.buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 2,
                  resource: map_storages[i].buffer.as_entire_binding(),
              },
              wgpu::BindGroupEntry {
                  binding: 3,
                  resource: map_storages[(i + 1) % 2].buffer.as_entire_binding(),
              },
          ],
      });

      compute_map_bind_groups.push(compute_map_bind_group);
  }

  let work_group_count = ((window_config.width * window_config.height) as f32 / 32.0) as u32;
  let compute_map_program = ComputeProgram::new(compute_map_pipeline, compute_map_bind_groups, (work_group_count, 1, 1))?;

  return Ok(compute_map_program);
}

pub fn construct_render_map_program(ctx: &mut Context, species_storage: &Storage) -> GameResult<RenderProgram> {
  let device = &ctx.gfx.wgpu().device;

  let render_shader = util::construct_shader_module(device, "Render Map Shader", include_str!("shaders/render_map.wgsl"))?;
  let render_map_bind_group_entries = &[
      wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Storage { read_only: true },
              has_dynamic_offset: false,
              min_binding_size: wgpu::BufferSize::new(species_storage.size as _),
          },
          count: None,
      },
  ];
  let render_map_bind_group_layout = util::construct_bind_group_layout(device, "Render Map Bind Group Layout", render_map_bind_group_entries)?;
  let pipeline_layout = util::construct_pipeline_layout(device, "Render Map Pipeline Layout", &vec![&render_map_bind_group_layout], &vec![])?;
  let render_map_pipeline = util::construct_render_pipeline(
      device, 
      "Render Map Pipeline", 
      Some(&pipeline_layout), 
      wgpu::VertexState {
          module: &render_shader,
          entry_point: "main_vs",
          buffers: &[
              wgpu::VertexBufferLayout {
                  array_stride: mem::size_of::<Agent>() as u64,
                  step_mode: wgpu::VertexStepMode::Instance,
                  attributes: &wgpu::vertex_attr_array![0 => Float32, 1 => Float32, 2 => Float32, 3 => Uint32],
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
                      alpha: wgpu::BlendComponent::OVER,
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

  let render_map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Compute Map Bind Group"),
      layout: &render_map_bind_group_layout,
      entries: &[
          wgpu::BindGroupEntry {
              binding: 0,
              resource: species_storage.buffer.as_entire_binding(),
          },
      ],
  });

  let render_map_program = RenderProgram::new(render_map_pipeline, vec![render_map_bind_group])?;

  return Ok(render_map_program);
} 
