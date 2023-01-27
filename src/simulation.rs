use ggez::{Context, GameResult};
use crate::{construct::*, SimulationConfig, WindowConfig, Param, Storage, ComputeProgram, RenderProgram};


pub struct Simulation {
    pub config: SimulationConfig,
    pub window_config: WindowConfig,

    pub compute_map_program: ComputeProgram,
    pub compute_agent_program: ComputeProgram,
    pub render_map_program: RenderProgram,

    pub constants_storage: Storage,
    pub agent_storage: Storage,
    pub param_storage: Storage,
    pub map_storages: Vec<Storage>,

    pub frame: usize,
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let device = &ctx.gfx.wgpu().device;

        let constants_storage  = construct_constants_storage(device, &window_config, &config)?;
        let param_storage      = construct_param_storage(device)?;
        let agent_storage      = construct_agent_storage(device, &config)?;
        let map_storages       = construct_map_storages(device, &window_config)?;
        let species_storage    = construct_species_storage(device)?;

        let compute_agent_program = construct_compute_agent_program(ctx, &config, &constants_storage, &agent_storage, &param_storage, &map_storages, &species_storage)?;
        let compute_map_program   = construct_compute_map_program(ctx, &window_config, &map_storages, &constants_storage, &param_storage)?; 
        let render_map_program    = construct_render_map_program(ctx, &species_storage)?;

        let simulation = Simulation { 
            config               , 
            window_config        ,
            compute_map_program  , 
            compute_agent_program, 
            render_map_program   , 
            constants_storage    , 
            agent_storage        , 
            param_storage        , 
            map_storages         , 
            frame: 0             ,
        };

        return Ok(simulation);
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult {
        let agent_data = construct_agents(&self.config)?;
        let map_data   = construct_trail_map(&self.window_config)?;

        ctx.gfx.wgpu().queue.write_buffer(&self.agent_storage.buffer  , 0, bytemuck::cast_slice(&agent_data));
        ctx.gfx.wgpu().queue.write_buffer(&self.map_storages[0].buffer, 0, bytemuck::cast_slice(&map_data));
        ctx.gfx.wgpu().queue.write_buffer(&self.map_storages[1].buffer, 0, bytemuck::cast_slice(&map_data));

        return Ok(());
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let frame = ctx.gfx.frame().clone();

        let delta_time = ctx.time.delta().as_secs_f32();
        let param = [Param { delta_time, frame: self.frame as u32 }];

        ctx.gfx.wgpu().queue.write_buffer(&self.param_storage.buffer, 0, bytemuck::cast_slice(&param));
        
        let command_encoder = ctx.gfx.commands().unwrap();

        self.compute_map_program.process(command_encoder, self.frame)?;
        self.compute_agent_program.process(command_encoder, self.frame)?;

        command_encoder.push_debug_group("Render Map");
        {
            let window_area = (self.window_config.width * self.window_config.height) as u32;
            let color_attachments = &[wgpu::RenderPassColorAttachment {
                view: frame.wgpu().1,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }];

            self.render_map_program.process(command_encoder, color_attachments, vec![&self.map_storages[(self.frame + 1) % 2]], 0..1, 0..window_area, self.frame)?;
        }
        command_encoder.pop_debug_group();
        
        self.frame += 1;

        return Ok(());
    }
}
