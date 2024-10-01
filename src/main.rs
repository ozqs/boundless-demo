use boundless_demo::Updateable;
use macroquad::prelude::*;

#[macroquad::main("Boundless demo")]
async fn main() {
    let mut boundless_demo = boundless_demo::BoundlessDemo::new();
    loop {
        clear_background(WHITE);
        boundless_demo.update(get_frame_time());
        next_frame().await;
    }
}