use macroquad::prelude::*;

#[macroquad::main("griddy")]

async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
