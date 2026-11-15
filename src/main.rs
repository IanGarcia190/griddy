use macroquad::{input::KeyCode::W, prelude::*};

#[macroquad::main("griddy")]

async fn main() {
    let screen_center: Vec2 = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let grid_spacing = 50.0;
    let height = 3;
    let width = 3;
    let grid = make_grid(height, width, screen_center, grid_spacing);
    let mut player_index = (1, 1); //(x,y) coordinate
    //let mut player_coord: Vec2 = get_player_coord(&player_index, grid);

    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::D) {
            player_index.0 += 1;
        }

        if is_key_pressed(KeyCode::A) {
            player_index.0 -= 1;
        }
        if is_key_pressed(KeyCode::W) {
            player_index.1 += 1;
        }
        if is_key_pressed(KeyCode::S) {
            player_index.1 -= 1;
        }

        //draw_player(&player_coord);
        draw_grid(&grid);
        next_frame().await
    }
}

fn make_grid(height: i32, width: i32, screen_center: Vec2, grid_spacing: f32) -> Vec<Vec<Vec2>> {
    let mut grid: Vec<Vec<Vec2>> = Vec::with_capacity(height as usize);
    let topleft: Vec2 = screen_center - Vec2::new(grid_spacing, grid_spacing);

    //Currently we have the top left node of the grid
    //We now add nodes horizontally and vertically according to the spacing.

    for i in 0..height {
        let mut row: Vec<Vec2> = Vec::with_capacity(width as usize);
        for j in 0..width {
            let node_coord: Vec2 =
                topleft + Vec2::new(j as f32 * grid_spacing, i as f32 * grid_spacing);
            row.push(node_coord);
        }
        grid.push(row);
    }

    grid
}

fn draw_grid(grid: &Vec<Vec<Vec2>>) {
    for row in grid {
        for pos in row {
            draw_circle_lines(pos.x, pos.y, 10.0, 2.0, YELLOW);
        }
    }
}

//fn get_player_coord(player_index: &(i32, i32), grid: Vec<Vec<Vec2>>) {}
