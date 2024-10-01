use macroquad::prelude::*;
use crate::fluent::Interpolatable;

pub trait Drawable {
    fn draw(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct LineFromOrigin {
    rotation: f32, // in radius
    thickness: f32,
    color: Color,
}

impl LineFromOrigin {
    pub fn new(rotation: f32, thickness: f32, color: Color) -> Self {
        Self {
            rotation,
            thickness,
            color,
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}

impl Drawable for LineFromOrigin {
    fn draw(&self) {
        let screen_center = vec2(screen_width() / 2., screen_height() / 2.);
        let length = screen_center.length() * 2.;
        let end = screen_center + vec2(self.rotation.cos(), self.rotation.sin()) * length;

        draw_line(screen_center.x, screen_center.y, end.x, end.y, self.thickness, self.color);
    }
}

impl Interpolatable for LineFromOrigin {
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        Self {
            rotation: self.rotation.interpolate(&other.rotation, progress),
            thickness: self.thickness.interpolate(&other.thickness, progress),
            color: Color::from_vec(self.color.to_vec().interpolate(&other.color.to_vec(), progress)),
        }
    }
}