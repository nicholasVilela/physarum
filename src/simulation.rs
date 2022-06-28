use ggez::{Context, GameResult, graphics::{DrawParam, Canvas}};
use crate::{Agent, Trail, SimulationConfig, WindowConfig, SpeciesConfig, Species, config};


pub struct Simulation {
    pub agents: Vec<Agent>,
    pub trail: Trail,
    pub config: SimulationConfig,
    pub window_config: WindowConfig
}

impl Simulation {
    pub fn new(ctx: &mut Context, config: SimulationConfig, window_config: WindowConfig) -> GameResult<Simulation> {
        let agents = Simulation::construct_agents(&config, &window_config)?;
        let trail = Trail::new(ctx, &window_config)?;
        let simulation = Simulation { agents, trail, config, window_config };

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
        let delta = ctx.time.delta();

        for agent in &mut self.agents {
            agent.update(delta, &self.window_config, &mut self.trail)?;

            // if self.config.render_agents { 
            //     let draw_param = DrawParam::new()
            //         .dest(Point2 { x: agent.position.x, y: agent.position.y });
            //     self.agent_meshbatch.push(draw_param);
            // }
        }
        self.trail.update(ctx, &self.window_config, &self.config)?;

        return Ok(());
    }

    pub fn render(&mut self, canvas: &mut Canvas) -> GameResult {
        canvas.draw(&self.trail.map, DrawParam::default());

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
}
