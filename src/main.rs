use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,

}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[derive(Default)]
struct Swipe {
    start_position: Option<Vec2>,
    end_position: Option<Vec2>,
}

impl Swipe {
    fn detect(&self) -> Option<&'static str> {
        if let (Some(start), Some(end)) = (self.start_position, self.end_position) {
            let dx = end.x - start.x;
            let dy = end.y - start.y;

            let min_distance = 50.0;
            let max_vertical_deviation = 30.0;

            if dx.abs() > min_distance && dy.abs() < max_vertical_deviation {
               return if dx > 0.0 {
                Some("Swipe Right")
               } else {
                Some("Swipe Left")
               };
            }
        }
        None
    }
}

const MOVEMENT_SPEED: f32 = 200.0;

#[macroquad::main("Fly Away")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];

    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let mut gameover = false;

    let mut swipe = Swipe::default();

    loop {
        if !gameover {
            clear_background(DARKPURPLE);

            let delta_time = get_frame_time();

            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }

            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }

            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }

            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }

            //touches
            for touch in touches() {
                match touch.phase {
                    TouchPhase::Started => {
                        swipe.start_position = Some(touch.position);
                    }
                    TouchPhase::Ended => {
                        swipe.end_position = Some(touch.position);


                        if let Some(direction) = swipe.detect() {
                            draw_text(direction, screen_width() / 2.0 - 100.0, screen_height() / 2.0, 40.0, RED);
                        }
                        
                        
                        circle.x += MOVEMENT_SPEED * delta_time;
                        //circle.y += MOVEMENT_SPEED * delta_time;

                        swipe.start_position = None;
                        swipe.end_position = None;
                        break;

                    }
                    _ => {}

                }
            }
            //end touches

            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            squares.retain(|square| square.y < screen_height() + square.size);

            if squares.iter().any(|square| circle.collides_with(square)) {
                gameover = true;
            }


            draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);

            for square in &squares {
                draw_rectangle(
                    square.x - square.size / 2.0,
                    square.y - square.size / 2.0,
                    square.size,
                    square.size,
                    GREEN,
                );
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            println!("pressing space");
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );

            let pressspace = "Press Space Bar to Play Again";
            let press_dimensions = measure_text(pressspace, None, 50, 1.0);

            draw_text(
                pressspace,
                screen_width() / 2.0 - press_dimensions.width / 4.0,
                screen_height() / 2.0 + 40.0,
                25.0,
                GREEN,
            );

        }

        next_frame().await
    }
}
