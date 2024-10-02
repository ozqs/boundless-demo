use std::{
    f32::consts::PI,
    ops::Deref,
    sync::{Arc, LazyLock, Mutex},
    usize,
};

use macroquad::prelude::*;

use crate::{
    fluent::Fluent,
    math,
    shapes::{Drawable, LineFromOrigin},
    Updateable,
};

static STACK: LazyLock<Arc<Mutex<Vec<Vec<MenuItem>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

#[derive(Clone)]
pub struct MenuItem {
    text: String,
    sub: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(text: String, sub: Vec<MenuItem>) -> Self {
        Self { text, sub }
    }
}

impl From<&str> for MenuItem {
    fn from(s: &str) -> Self {
        Self::new(s.to_string(), Vec::new())
    }
}

impl Deref for MenuItem {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

#[derive(Clone)]
pub struct Menu {
    options: Vec<MenuItem>,
    lines: Vec<Fluent<LineFromOrigin>>,
}

impl Menu {
    pub fn new(options: Vec<MenuItem>) -> Self {
        let lines = Vec::new();

        let mut s = Self { options, lines };
        s.initialize(0.);

        s
    }

    fn initialize(&mut self, rotation_delta: f32) {
        let len = self.options.len();
        for i in 0..len {
            let mut obj = Fluent::new(LineFromOrigin::new(rotation_delta, 2., BLACK), 1.);

            let mut rotation = 2. * PI / len as f32 * i as f32 + rotation_delta;

            while rotation < 0. {
                rotation += 2. * PI;
            }

            while rotation > 2. * PI {
                rotation -= 2. * PI;
            }

            obj.set_target(LineFromOrigin::new(rotation, 2., BLACK));

            self.lines.push(obj);
        }
    }

    fn calculate_click(&mut self, mouse_pos: (f32, f32)) -> Option<usize> {
        let screen_center = vec2(screen_width() / 2., screen_height() / 2.);
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);

        if (mouse_pos - screen_center).length() <= 50. {
            return Some(usize::MAX);
        }

        let mut rotation = (mouse_pos - screen_center).to_angle();
        if rotation < 0. {
            rotation += PI * 2.;
        }

        for i in 0..self.lines.len() {
            let line = &self.lines[i];

            let begin = line.value.rotation();
            let mut end = self.lines[(i + 1) % self.lines.len()].value.rotation();

            if end <= 0. {
                end += PI * 2.;
            }

            if begin <= rotation && rotation <= end {
                return Some(i);
            }
        }

        None
    }
}

impl Updateable for Menu {
    fn update(&mut self, delta: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if let Some(x) = self.calculate_click(mouse_pos) {

                let rotation;

                if x == usize::MAX {
                    self.options = match STACK.lock().unwrap().pop() {
                        Some(x) => x,
                        None => return,
                    };
                    rotation = 0.;
                } else if self.options[x].sub.is_empty() {
                    return;
                } else {
                    STACK.lock().unwrap().push(self.options.clone());
                    self.options = self.options[x].sub.clone();
                    rotation = self.lines[x].value.rotation()
                }

                self.lines.clear();


                self.initialize(rotation);

                return;
            }
        }

        for line in self.lines.iter_mut() {
            line.update(delta);
        }
    }
}

impl Drawable for Menu {
    fn draw(&self) {
        // Draw Back Button in the center with radius 50.
        let center = vec2(screen_width() / 2., screen_height() / 2.);
        draw_circle(center.x, center.y, 50., BLACK);

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
