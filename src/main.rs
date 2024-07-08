use ggez::{
    conf, event, glam::*, graphics::{self, Color}, input::keyboard::KeyCode, mint::Point2, Context, GameResult
};
use core::time;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;


struct MainState {
    bird: Bird,
    points: [Point2<f32>; 2000],
    spiral: graphics::Mesh,
    zoom: f32
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
        let bird = Bird::new(circle, 100.0, 50.0);
        let point = Point2 { x: 0.0, y: 0.0 };
        let mut points: [Point2<f32>; 2000] = [point; 2000];
        for i in 0..2000 {
            let (x, y) = MainState::poltocart(i as f32, i as f32 * 0.1);
            points[i].x = x;
            points[i].y = y;
        }
        let spiral = graphics::Mesh::new_line(ctx, &points, 1.0, Color::RED)?;

        Ok(MainState { bird, spiral, points,zoom: 1.0 })
    }

    fn poltocart(radius: f32, angle: f32) -> (f32, f32) {
        (radius * angle.cos(), radius * angle.sin())
    }

    fn draw_elements(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let (x, y) = MainState::poltocart(self.bird.radius, self.bird.angle);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(x * self.zoom, y * self.zoom),
            10.0 * self.zoom,
            2.0,
            Color::WHITE,
        )?;
        let point = Point2 { x: 0.0, y: 0.0 };
        let mut points: [Point2<f32>; 2000] = [point; 2000];
        for i in 0..2000 {
            let (x, y) = MainState::poltocart(i as f32 * 5.0, i as f32 * 0.1);
            points[i].x = x * self.zoom;
            points[i].y = y * self.zoom;
        }
        let spiral = graphics::Mesh::new_line(ctx, &points, 1.0, Color::RED)?;
        self.spiral = spiral;
        self.bird.circle = circle;
        self.points = points;
        canvas.draw(&self.spiral, Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0));
        canvas.draw(&self.bird.circle, Vec2::new(x + SCREEN_WIDTH / 2.0, y + SCREEN_HEIGHT / 2.0));
        canvas.finish(ctx)?;

        Ok(())
    }

    fn zoom_out(&mut self, dt: time::Duration) {
        self.zoom = if self.zoom - (0.05 * dt.as_secs_f32()) < 0.1 { 0.1 } else { self.zoom - (0.05 * dt.as_secs_f32()) };
    }

    fn update_elements(&mut self, dt: time::Duration) {
        self.zoom_out(dt);
    }

    fn line_intersects_circle(x1:f32, y1:f32, x2:f32, y2:f32, xc:f32, yc:f32, r:f32) -> bool {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let fx = x1 - xc;
        let fy = y1 - yc;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (fx * dx + fy * dy);
        let c = (fx * fx + fy * fy) - r * r;

        let mut discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            false
        } else {
            discriminant = discriminant.sqrt();
            let t1 = (-b - discriminant) / (2.0 * a);
            let t2 = (-b + discriminant) / (2.0 * a);

            if (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0) {
                true
            } else {
                false
            }

        }
    }

    fn spiral_intersects_circle(&mut self) -> bool {
        for i in 0..(self.points.len() - 1) {
            let (x1, y1, x2, y2) = (self.points[i].x, self.points[i].y, self.points[i + 1].x, self.points[i + 1].y);
            let (xc, yc) = MainState::poltocart(self.bird.radius, self.bird.angle);
            if MainState::line_intersects_circle(x1, y1, x2, y2, xc, yc, 10.0 * self.zoom) == true {
                return true;
            }
        }
        false
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
    fn new(circle: graphics::Mesh, radius:f32, angle: f32) -> Bird {
        Bird {
            radius,
            angle,
            gravity: -0.1,
            lift: 50.0,
            velocity: 0.0,
            circle
        }
    }

    fn jump(&mut self) -> () {
        self.velocity = if self.velocity + self.lift > 400.0 { 400.0 } else { self.velocity + self.lift };
    }

    fn update(&mut self, dt: time::Duration) -> () {
        self.radius = self.radius + (self.velocity * dt.as_secs_f32());
        self.angle = self.angle + (0.5 * dt.as_secs_f32());
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
        self.update_elements(ctx.time.delta());
        let col = self.spiral_intersects_circle();
        if col {
            println!("collision!");
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let draw_elements = self.draw_elements(ctx)?;
        Ok(draw_elements)
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