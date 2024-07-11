use core::time;

use ggez::{graphics::{self, Color}, Context};



pub(crate) struct Bird {
    pub radius: f32,
    pub angle: f32,
    pub gravity: f32,
    pub lift: f32,
    pub velocity: f32,
    pub circle: graphics::Mesh,
    pub color: Color
}

impl Bird {
    pub(crate) fn new(circle: graphics::Mesh, radius:f32, angle: f32, color: Color) -> Bird {
        Bird {
            radius,
            angle,
            gravity: -0.1,
            lift: 50.0,
            velocity: 0.0,
            circle,
            color
        }
    }

    pub(crate) fn jump(&mut self) {
        self.velocity = if self.velocity + self.lift > 400.0 { 400.0 } else { self.velocity + self.lift };
    }

    pub(crate) fn update(&mut self, dt: time::Duration, ctx: &mut Context) {
        self.radius = self.radius + (self.velocity * dt.as_secs_f32());
        self.angle = self.angle + (0.5 * dt.as_secs_f32());
        self.velocity = if self.velocity + self.gravity < -500.0 { -500.0 } else { self.velocity + self.gravity };
    }

    pub(crate) fn update_color(&mut self, color: Color) {
        self.color = color;
    }
}