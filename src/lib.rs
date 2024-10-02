use menu::MenuItem;
use shapes::Drawable;

mod fluent;
mod math;
mod menu;
mod shapes;

pub trait Updateable {
    fn update(&mut self, delta: f32);
}

pub struct BoundlessDemo {
    menu: menu::Menu,
}

impl BoundlessDemo {
    pub fn new() -> Self {
        let options = vec![
            "File".into(),
            MenuItem::new(
                "Edit".into(),
                vec![
                    "Undo".into(),
                    "Redo".into(),
                    MenuItem::new(
                        "Advanced".into(),
                        vec![
                            "Cut".into(),
                            "Copy".into(),
                            "Paste".into(),
                            MenuItem::new(
                                "Special".into(),
                                vec![
                                    "Format".into(),
                                    "Spell Check".into(),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            MenuItem::new(
                "View".into(),
                vec![
                    "Zoom In".into(),
                    "Zoom Out".into(),
                    "FullScreen".into(),
                    MenuItem::new(
                        "Themes".into(),
                        vec![
                            "Light".into(),
                            "Dark".into(),
                            "Sepia".into(),
                        ],
                    ),
                ],
            ),
            "Help".into(),
            MenuItem::new(
                "Settings".into(),
                vec![
                    "General".into(),
                    "Appearance".into(),
                    "Shortcuts".into(),
                    MenuItem::new(
                        "Advanced".into(),
                        vec![
                            "Network".into(),
                            "Security".into(),
                            "Privacy".into(),
                            MenuItem::new(
                                "Developer".into(),
                                vec![
                                    "Console".into(),
                                    "Debug".into(),
                                    "Logs".into(),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ];
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
