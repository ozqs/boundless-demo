use std::{f32::consts::PI, option};

use macroquad::prelude::*;

use crate::{
    fluent::Fluent,
    math,
    shapes::{Drawable, LineFromOrigin},
    Updateable,
};

struct MenuItem {
    text: String,
    sub: Vec<MenuItem>,
}

pub struct Menu {
    options: Vec<String>,
    lines: Vec<Fluent<LineFromOrigin>>,
}

impl Menu {
    pub fn new(options: Vec<String>) -> Self {
        let mut lines = Vec::new();
        let len = options.len();
        for i in 0..len {
            let mut obj = Fluent::new(LineFromOrigin::new(0., 2., BLACK), 1.);

            obj.set_target(LineFromOrigin::new(
                2. * PI / len as f32 * i as f32,
                2.,
                BLACK,
            ));

            lines.push(obj);
        }

        Self { options, lines }
    }
}

impl Updateable for Menu {
    fn update(&mut self, delta: f32) {
        for line in self.lines.iter_mut() {
            line.update(delta);
        }
    }
}

impl Drawable for Menu {
    fn draw(&self) {
        for i in 0..self.options.len() {
            self.lines[i].value.draw();
            // Draw text

            let screen_center = vec2(screen_width() / 2., screen_height() / 2.);

            // let begin = 2. * PI / self.lines.len() as f32 * i as f32;
            let begin = self.lines[i % self.lines.len()].value.rotation();
            // let end = begin + 2. * PI / self.lines.len() as f32;
            let end = self.lines[(i + 1) % self.lines.len()].value.rotation();

            let begin = math::calculate_intersect(begin, screen_width(), screen_height());
            let end = math::calculate_intersect(end, screen_width(), screen_height());

            let triangle = (begin, end, screen_center);
            let measurement = measure_text(&self.options[i], None, 20, 1.);
            let position = vec2(
                (triangle.0.x + triangle.1.x + triangle.2.x) / 3.,
                (triangle.0.y + triangle.1.y + triangle.2.y) / 3.,
            ) - vec2(measurement.width, measurement.height) / 2.;

            draw_text(&self.options[i], position.x, position.y, 20., BLACK);
        }
    }
}
