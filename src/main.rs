use macroquad::prelude::*;

#[macroquad::main("Fly Away")]
async fn main() {

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let speed = 6.0;

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += speed;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= speed;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y += speed;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y -= speed;
        }

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await
    }
}
