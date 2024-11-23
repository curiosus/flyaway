use macroquad::prelude::*;

#[macroquad::main("Fly Away")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
