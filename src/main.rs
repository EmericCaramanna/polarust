
use ggez::{
    conf::{self, WindowSetup}, event, glam::*, graphics::{self, Color}, input::keyboard::KeyCode, Context, GameResult
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

    fn carttopol(x: f32, y: f32) -> (f32,f32) {
        ((x.powf(2.0) + y.powf(2.0)).sqrt(), y.atan2(x))
    }

    fn poltocart(radius: f32, angle: f32) -> (f32, f32) {
        (radius * angle.cos(), radius * angle.sin())
    }
}

struct Bird {
    radius: f32,
    angle: f32,
    gravity: f32,
    lift: f32,
    velocity: f32,
    circle: graphics::Mesh
}

impl Bird {
    fn new(circle: graphics::Mesh) -> Bird {
        Bird {
            radius: 100.0,
            angle: 0.0,
            gravity: -0.6,
            lift: 15.0,
            velocity: 0.0,
            circle
        }
    }

    fn jump(&mut self) -> () {
        self.velocity = if self.velocity + self.lift > 8.0 { 8.0 } else { self.velocity + self.lift };
    }

    fn update(&mut self) -> () {
        self.radius = self.radius + self.velocity;
        
        self.velocity = if self.velocity + self.gravity < -8.0 { -8.0 } else { self.velocity + self.gravity };
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
        let (x, y) = MainState::poltocart(self.bird.radius, self.bird.angle);

        canvas.draw(&self.bird.circle, Vec2::new(x + 800.0 / 2.0, y + 800.0 / 2.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("polarust", "emeric")
        .window_setup(conf::WindowSetup::default().title("Polarust"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 800.0));
    ;
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}