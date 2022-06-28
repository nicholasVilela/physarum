use ggez::{GameResult, Context, event::EventHandler, graphics::{self, TextFragment, Color, Text, Canvas, DrawParam}, mint::Point2, input::keyboard::{KeyCode}};
use crate::{load, SimulationConfig, WindowConfig, Simulation};


pub struct Engine {
    simulation: Simulation,
    window_config: WindowConfig,
    running: bool,
    paused: bool,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> GameResult<Engine> {
        let simulation_config = load::<SimulationConfig>("simulation")?;
        let window_config = load::<WindowConfig>("window")?;
        let simulation = Simulation::new(ctx, simulation_config, window_config)?;
        let running = false;
        let paused = false;

        ctx.gfx.add_font(
            "Main",
            graphics::FontData::from_path(ctx, "/fonts/BN6FontBold.ttf")?,
        );

        let engine = Engine { simulation, window_config , running, paused };

        return Ok(engine);
    }

    fn render_intro_text(&mut self, canvas: &mut Canvas) -> GameResult {
        let text_fragment = TextFragment::new("Press SPACE to Start\rPress P to Pause\rPress R to Restart\rPress ESC to close")
            .font("Main")
            .color(Color::new(1.0, 1.0, 1.0, 1.0));
        let text = Text::new(text_fragment);
        let text_draw_param = DrawParam::new()
            .dest(Point2 { x: ((self.window_config.width / 2) - self.window_config.width / 8) as f32, y: (self.window_config.height / 2) as f32 });
        canvas.draw(&text, text_draw_param);

        return Ok(());
    }

    fn render_fps(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let fps = ctx.time.fps();
        let fps_fragment = TextFragment::new(format!("{:?}", fps as i32)).font("Main").color(Color::new(0.0, 1.0, 0.0, 1.0));
        let fps_text = Text::new(fps_fragment);
        let fps_draw_param = DrawParam::new().dest(Point2 { x: 0.0, y: 0.0 });
        canvas.draw(&fps_text, fps_draw_param);

        return Ok(());
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) { self.running = true; }
        if self.running && ctx.keyboard.is_key_just_pressed(KeyCode::P) { self.paused = !self.paused; }
        
        if !self.running || self.paused { return Ok(()); }

        if ctx.keyboard.is_key_just_pressed(KeyCode::R) { 
            self.simulation.reset(ctx)?;
            self.paused = false;
         }
        
        self.simulation.update(ctx)?;

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = self.window_config.background;
        let mut canvas = graphics::Canvas::from_frame(ctx, background_color);

        if self.running {
            self.simulation.render(&mut canvas)?;

            // if self.simulation_config.render_agents {
            //     canvas.draw(&self.agent_meshbatch, DrawParam::default());
            //     self.agent_meshbatch.clear();
            // }
        }
        else {
            self.render_intro_text(&mut canvas)?;
        }

        if self.window_config.show_fps {
            self.render_fps(ctx, &mut canvas)?;
        }

        return canvas.finish(ctx);
    }
}
