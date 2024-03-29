use ggez::{GameResult, Context, event::EventHandler, graphics::{self, TextFragment, Color, Text, Canvas, DrawParam}, mint::Point2, input::keyboard::{KeyCode}};
use ggez_egui::{EguiBackend, egui};
use crate::{load, SimulationConfig, WindowConfig, Simulation};


pub struct Engine {
    simulation: Simulation,
    window_config: WindowConfig,
    egui_backend: EguiBackend,
    running: bool,
    paused: bool,
}

impl Engine {
    pub fn new(ctx: &mut Context) -> GameResult<Engine> {
        let simulation_config = load::<SimulationConfig>("simulation")?;
        let window_config = load::<WindowConfig>("window")?;
        let egui_backend = EguiBackend::default();
        let simulation = Simulation::new(ctx, simulation_config, window_config.clone())?;
        let running = window_config.auto_run;
        let paused = false;

        ctx.gfx.add_font(
            "Main",
            graphics::FontData::from_path(ctx, "/fonts/BN6FontBold.ttf")?,
        );

        let engine = Engine { simulation, window_config, egui_backend, running, paused };

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

        // let egui_ctx = self.egui_backend.ctx();
        // egui::Window::new("config").show(&egui_ctx, |ui| {
        //     ui.label("testing");
        // });
        // self.egui_backend.update(ctx);

        if ctx.keyboard.is_key_just_pressed(KeyCode::R) {
            self.simulation.reset(ctx)?;
            self.paused = false;
        }

        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.running {
            if self.paused { return Ok(()); }

            self.simulation.render(ctx)?;

            if !self.window_config.show_fps { return Ok(()); }
            
            let mut canvas = Canvas::from_frame(ctx, None);
            canvas.draw(&self.egui_backend, DrawParam::default());
            self.render_fps(ctx, &mut canvas)?;
            canvas.finish(ctx)?;

            return Ok(());
        }

        let mut canvas = Canvas::from_frame(ctx, self.window_config.background); 
        self.render_intro_text(&mut canvas)?;
        canvas.finish(ctx)?;

        return Ok(());
    }
}
