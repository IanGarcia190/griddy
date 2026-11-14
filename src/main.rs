use macroquad::prelude::*;

#[macroquad::main("griddy")]

async fn main() {
    let center_height: f32 = screen_height();
    let center_width: f32 = screen_width();
    let screen_center: (f32, f32) = (center_width, center_height);
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}

fn make_grid(height: i32, width: i32, screen_center: (f32, f32)) -> Vec<Vec<Vec2>> {
    let mut grid: Vec<Vec<Vec2>> = Vec::new();

    grid
}

fn draw_grid(grid: Vec<Vec<Vec2>>) {}
