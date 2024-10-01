use shapes::Drawable;

mod fluent;
mod shapes;
mod menu;
mod math;

pub trait Updateable {
    fn update(&mut self, delta: f32);
}

pub struct BoundlessDemo {
    menu: menu::Menu,
}

impl BoundlessDemo {
    pub fn new() -> Self {
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
        Self {
            menu: menu::Menu::new(options),
        }
    }
}

impl Updateable for BoundlessDemo {
    fn update(&mut self, delta: f32) {
        self.menu.update(delta);
        self.menu.draw();
    }
}