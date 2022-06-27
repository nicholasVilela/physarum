use ggez::{GameResult, Context, event::EventHandler, graphics::{self, TextFragment, Color, Text, InstanceArray, DrawParam}, mint::Point2};
use crate::{load, SimulationConfig, Agent, WindowConfig, Trail, Species, SpeciesConfig};


pub struct Engine {
    agents: Vec<Agent>,
    agent_meshbatch: InstanceArray,
    trail: Trail,
    window_config: WindowConfig,
    simulation_config: SimulationConfig,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> GameResult<Engine> {
        let window_config = load::<WindowConfig>("window")?;
        let simulation_config = load::<SimulationConfig>("simulation")?;
        let agents = Engine::construct_agents(&window_config, &simulation_config)?;
        let agent_meshbatch = Engine::construct_agent_meshbatch(ctx, &simulation_config)?;
        let trail = Engine::construct_trail(ctx, &window_config)?;

        ctx.gfx.add_font(
            "Main",
            graphics::FontData::from_path(ctx, "/fonts/BN6FontBold.ttf")?,
        );

        let engine = Engine { agents, agent_meshbatch, trail, window_config , simulation_config};

        return Ok(engine);
    }

    fn construct_agents(window_config: &WindowConfig, simulation_config: &SimulationConfig) -> GameResult<Vec<Agent>> {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        let species_config = load::<SpeciesConfig>(&Species::A.to_string())?;

        for _ in 0..simulation_config.agent_count {
            let agent = Agent::new(Species::A, species_config, window_config, &simulation_config, &mut rng)?;
            agents.push(agent);
        }

        return Ok(agents);
    }
    
    fn construct_agent_meshbatch(ctx: &mut Context, simulation_config: &SimulationConfig) -> GameResult<InstanceArray> {
        let meshbatch = InstanceArray::new(ctx, None, simulation_config.agent_count as u32, false);

        return Ok(meshbatch);
    }

    fn construct_trail(ctx: &mut Context, window_config: &WindowConfig) -> GameResult<Trail> {
        let trail = Trail::new(ctx, window_config)?;

        return Ok(trail);
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ctx.time.delta();

        for agent in &mut self.agents {
            agent.update(delta, &self.window_config, &mut self.trail)?;

            if self.simulation_config.render_agents { 
                let draw_param = DrawParam::new()
                    .dest(Point2 { x: agent.position.x, y: agent.position.y });
                self.agent_meshbatch.push(draw_param);
            }
        }
        
        self.trail.update(ctx, &self.window_config, &self.simulation_config)?;

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = self.window_config.background;
        let mut canvas = graphics::Canvas::from_frame(ctx, background_color);

        canvas.draw(&self.trail.map, DrawParam::default());

        if self.simulation_config.render_agents {
            canvas.draw(&self.agent_meshbatch, DrawParam::default());
            self.agent_meshbatch.clear();
        }

        let fps = ctx.time.fps();
        let fps_fragment = TextFragment::new(format!("{:?}", fps as i32)).font("Main").color(Color::new(0.0, 1.0, 0.0, 1.0));
        let fps_text = Text::new(fps_fragment);
        let fps_draw_param = DrawParam::new().dest(Point2 { x: 0.0, y: 0.0 });

        canvas.draw(&fps_text, fps_draw_param);

        return canvas.finish(ctx);
    }
}
