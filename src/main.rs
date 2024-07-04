
use ggez::{
    event, glam::*, graphics::{self, Color}, input::keyboard::KeyCode, Context, GameResult
};

struct MainState {
    bird: Bird,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(100., 0.),
            10.0,
            2.0,
            Color::WHITE,
        )?;
        let bird = Bird::new(circle);

        Ok(MainState { bird })
    }
}

struct Bird {
    x: f32,
    y: f32,
    gravity: f32,
    lift: f32,
    velocity: f32,
    circle: graphics::Mesh
}

impl Bird {
    fn new(circle: graphics::Mesh) -> Bird {
        Bird {
            x: 100.0,
            y: 250.0,
            gravity: 0.6,
            lift: -15.0,
            velocity: 0.0,
            circle
        }
    }

    fn jump(&mut self) -> () {
        self.velocity = if self.velocity + self.lift > -5.0 { -5.0 } else { self.velocity + self.lift };
    }

    fn update(&mut self) -> () {
        self.y += self.velocity;
        self.velocity = if self.velocity + self.gravity > 8.0 { 8.0 } else { self.velocity + self.gravity };


    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
            Ok(if input.keycode == Some(KeyCode::Space) {
                self.bird.jump();
            })
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.bird.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.bird.circle, Vec2::new(self.bird.x, self.bird.y));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}