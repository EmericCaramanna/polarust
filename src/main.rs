use ggez::{
    conf, event, glam::*, graphics::{self, Color}, input::keyboard::KeyCode, mint::Point2, Context, GameResult
};
use core::time;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;


struct MainState {
    bird: Bird,
    spiral: graphics::Mesh
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
        let point = Point2 { x: 0.0, y: 0.0 };
        let mut points: [Point2<f32>; 200] = [point; 200];
        for i in 0..200 {
            let (x, y) = MainState::poltocart(i as f32, i as f32);
            points[i].x = x;
            points[i].y = y;
        }
        let spiral = graphics::Mesh::new_line(ctx, &points, 1.0, Color::RED)?;

        Ok(MainState { bird, spiral})
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
            lift: 500.0,
            velocity: 0.0,
            circle
        }
    }

    fn jump(&mut self) -> () {
        self.velocity = if self.velocity + self.lift > 400.0 { 400.0 } else { self.velocity + self.lift };
    }

    fn update(&mut self, dt: time::Duration) -> () {
        self.radius = self.radius + (self.velocity * dt.as_secs_f32());
        
        self.velocity = if self.velocity + self.gravity < -500.0 { -500.0 } else { self.velocity + self.gravity };
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

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.bird.update(ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let (x, y) = MainState::poltocart(self.bird.radius, self.bird.angle);

        canvas.draw(&self.bird.circle, Vec2::new(x + SCREEN_WIDTH / 2.0, y + SCREEN_HEIGHT / 2.0));

        canvas.draw(&self.spiral, Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0));
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("polarust", "emeric")
        .window_setup(conf::WindowSetup::default().title("Polarust"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}