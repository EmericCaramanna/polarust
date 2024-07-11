mod bird;
use bird::Bird;

use ggez::{
    conf, event, glam::*, graphics::{self, Color}, input::keyboard::KeyCode, mint::Point2, Context, GameResult
};
use core::time;
use std::f32::consts::PI;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;


struct MainState {
    bird: Bird,
    points: [Point2<f32>; 2000],
    spiral: graphics::Mesh,
    zoom: f32,
    zoom_factor: f32,
    zoom_count: u16,
    zooming_out: bool
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            10.0,
            2.0,
            Color::WHITE,
        )?;
        let bird = Bird::new(circle, 100.0, 10.0, Color::WHITE);
        let mut points: [Point2<f32>; 2000] = [Point2 { x: 0.0, y: 0.0 }; 2000];
        for i in 0..2000 {
            let (x, y) = MainState::poltocart(i as f32 * 5.0, i as f32 * 0.1);
            points[i].x = x * 1.0;
            points[i].y = y;
        }
        let spiral = graphics::Mesh::new_line(ctx, &points, 10.0, Color::RED)?;
        Ok(MainState { bird, spiral, points, zoom: 1.0, zoom_factor: 0., zoom_count: 0, zooming_out: false})
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
            vec2(0., 0.),
            10.0 * self.zoom,
            2.0,
            self.bird.color,
        )?;
        let mut points: [Point2<f32>; 2000] = [Point2 { x: 0.0, y: 0.0 }; 2000];
        for i in 0..2000 {
            let (x, y) = MainState::poltocart(i as f32 * 5.0, i as f32 * 0.1);
            points[i].x = x * self.zoom;
            points[i].y = y * self.zoom;
        }
        let spiral = graphics::Mesh::new_line(ctx, &points, 10.0 * self.zoom, Color::RED)?;
        self.spiral = spiral;
        self.bird.circle = circle;
        self.points = points;
        canvas.draw(&self.spiral, Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0));
        canvas.draw(&self.bird.circle, Vec2::new(x + SCREEN_WIDTH / 2.0, y + SCREEN_HEIGHT / 2.0));
        canvas.finish(ctx)?;
        Ok(())
    }

    fn zoom_out(&mut self, dt: time::Duration) {
        self.zoom = if self.zoom - (self.zoom_factor * dt.as_secs_f32()) < 0.1 { 0.1 } else { self.zoom - (self.zoom_factor * dt.as_secs_f32()) };
    }

    fn update_elements(&mut self, dt: time::Duration) {
        if self.bird.angle % PI >= 0.0 && self.bird.angle % PI < 0.1 {
            self.zoom_count = self.zoom_count + 1;
            self.zoom_factor = if self.zoom_count % 2 == 0 { self.zooming_out = true; 0.05 } else { self.zooming_out = false; 0.0 };
            println!("{}", self.zoom_count );
        }
        self.zoom_out(dt);
    }

    fn point_circle_collision(point: Point2<f32>, circle_center: Point2<f32>, circle_radius: f32) -> bool {
        let dist_x = point.x - circle_center.x;
        let dist_y = point.y - circle_center.y;
        let distance = (dist_x.powf(2.0) + dist_y.powf(2.0)).sqrt();
          
        distance <= circle_radius
    }

    fn spiral_intersects_circle(&mut self) -> bool {
        for i in 0..(self.points.len() - 1) {
            let (xc, yc) = MainState::poltocart(self.bird.radius, self.bird.angle);
            let center: Point2<f32> = Point2 { x: xc, y: yc };
            if MainState::point_circle_collision(self.points[i], center, 10.0 * self.zoom) == true {
                return true;
            }
        }
        false
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
        self.bird.update(ctx.time.delta(), ctx);
        self.update_elements(ctx.time.delta());
        let col = self.spiral_intersects_circle();
        if col {
            self.bird.update_color(Color::RED);
        } else {
            self.bird.update_color(Color::WHITE);
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_poltocart() {
        assert_eq!(MainState::poltocart(10., 50.), (9.64966, -2.6237485));
    }

    #[test]
    fn test_point_circle_collision() {
        assert_eq!(MainState::point_circle_collision(Point2 {x: 0., y: 0.}, Point2 {x: 0., y: 0.}, 15.), true);
        assert_eq!(MainState::point_circle_collision(Point2 {x: 10., y: 0.}, Point2 {x: 0., y: 0.}, 15.), true);
        assert_eq!(MainState::point_circle_collision(Point2 {x: 0., y: 0.}, Point2 {x: 0., y: -10.}, 15.), true);
        assert_eq!(MainState::point_circle_collision(Point2 {x: 0., y: 0.}, Point2 {x: 0., y: -100.}, 15.), false);
        assert_eq!(MainState::point_circle_collision(Point2 {x: -50., y: 50.}, Point2 {x: 0., y: 0.}, 15.), false);
    }
}