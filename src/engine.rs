use std::{collections::HashMap, str::FromStr};
use ggez::{GameResult, Context, event::EventHandler, graphics::{self, Color, MeshBatch, MeshBuilder, DrawParam}, mint::Point2, timer};
use crate::{load, SimulationConfig, Agent, WindowConfig, Vec2, Trail, Species, SpeciesConfig};
use rayon::prelude::*;


pub struct Engine {
    agents: Vec<Agent>,
    agent_meshbatch: MeshBatch,
    trail: Trail,
    window_config: WindowConfig,
    simulation_config: SimulationConfig,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> GameResult<Engine> {
        let window_config = load::<WindowConfig>("window")?;
        let simulation_config = load::<SimulationConfig>("simulation")?;
        let agents = Engine::construct_agents(&window_config, &simulation_config)?;
        let agent_meshbatch = Engine::construct_agent_meshbatch(ctx)?;
        let trail = Engine::construct_trail(ctx, &window_config)?;

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
    
    fn construct_agent_meshbatch(ctx: &mut Context) -> GameResult<MeshBatch> {
        let color = load::<SpeciesConfig>("species_A")?.color;
        let mesh = MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                Point2 { x: 1.0, y: 1.0 },
                1.0,
                1.0,
                color,
            )
            .unwrap()
            .build(ctx)
            .unwrap();

        let meshbatch = MeshBatch::new(mesh).unwrap();

        return Ok(meshbatch);
    }

    fn construct_trail(ctx: &mut Context, window_config: &WindowConfig) -> GameResult<Trail> {
        let trail = Trail::new(ctx, window_config)?;

        return Ok(trail);
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = timer::delta(ctx);

        for agent in &mut self.agents {
            agent.update(delta, &self.window_config, &mut self.trail)?;

            if self.simulation_config.render_agents { 
                let draw_param = DrawParam::new()
                    .dest(Point2 { x: agent.position.x, y: agent.position.y });
                self.agent_meshbatch.add(draw_param);
            }
        }
        
        self.trail.update(ctx, &self.window_config, &self.simulation_config)?;

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        graphics::draw(ctx, &self.trail.map, DrawParam::default())?;

        if self.simulation_config.render_agents {
            self.agent_meshbatch.draw(ctx, DrawParam::default()).unwrap();
            self.agent_meshbatch.clear();
        }

        let fps = timer::fps(ctx);
        let fps_text = graphics::Text::new(format!("{:?}", fps as i32));
        let fps_text_draw_param = graphics::DrawParam::new().dest(Point2 { x: 0.0, y: 0.0 }).color(Color::GREEN);

        graphics::draw(ctx, &fps_text, fps_text_draw_param)?;

        return graphics::present(ctx);
    }
}
