use macroquad::prelude::*;
use macroquad::rand::gen_range;

const ENEMY_SPEED: f32 = 180.0;
const PLAYER_RADIUS: f32 = 4.0;
const PLAYER_COLOR: Color = BLUE;

struct Enemy {
    pos: Vec2,
    vel: Vec2, // pixels per second
    size: f32, // side length of the square
    health: i32,
    color: Color,
}

impl Enemy {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            size: 12.0,
            health: 3,
            color: WHITE,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    fn draw(&self) {
        // draw_rectangle takes the top-left corner, so offset by half the size to center it
        draw_rectangle(
            self.pos.x - self.size / 2.0,
            self.pos.y - self.size / 2.0,
            self.size,
            self.size,
            self.color,
        );
    }

    fn is_dead(&self) -> bool {
        self.health <= 0
    }
    fn overlaps_circle(&self, center: Vec2, radius: f32) -> bool {
        let half = self.size / 2.0;
        let closest = center.clamp(self.pos - vec2(half, half), self.pos + vec2(half, half));
        closest.distance_squared(center) <= radius * radius
    }
}

struct Player {
    index: (i32, i32),
    pos: Vec2,
    radius: f32,
    color: Color,
}

impl Player {
    fn new() -> Self {
        Self {
            index: (1, 1),
            // pos is literally just the screen center.
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            radius: PLAYER_RADIUS,
            color: PLAYER_COLOR,
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}

#[macroquad::main("griddy")]
async fn main() {
    let screen_center: Vec2 = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let grid_spacing = 50.0;
    let height = 3;
    let width = 3;
    let grid = make_grid(height, width, screen_center, grid_spacing);
    let mut player: Player = Player::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut spawn_timer: f32 = gen_range(0.5, 1.0);

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();
        if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::Right) && (0..width).contains(&(player.index.0 + 1)) {
            player.index.0 += 1;
        }

        if is_key_pressed(KeyCode::Left) && (0..width).contains(&(player.index.0 - 1)) {
            player.index.0 -= 1;
        }
        if is_key_pressed(KeyCode::Down) && (0..height).contains(&(player.index.1 + 1)) {
            player.index.1 += 1;
        }
        if is_key_pressed(KeyCode::Up) && (0..height).contains(&(player.index.1 - 1)) {
            player.index.1 -= 1;
        }

        player.pos = grid[player.index.1 as usize][player.index.0 as usize];

        // managing enemies

        spawn_timer -= dt;
        if spawn_timer <= 0.0 {
            let mut e = Enemy::new(random_edge_position());
            e.vel = (player.pos - e.pos).normalize_or_zero() * ENEMY_SPEED;
            enemies.push(e);
            spawn_timer = gen_range(0.5, 1.0);
        }

        // DRAWING

        draw_player(&player.pos);
        draw_grid(&grid);
        for enemy in enemies.iter_mut() {
            enemy.update(dt);
            enemy.draw();
            if enemy.overlaps_circle(player.pos, PLAYER_RADIUS) {
                std::process::exit(0);
            }
        }

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

fn draw_player(pos: &Vec2) {
    draw_circle(pos.x, pos.y, PLAYER_RADIUS, BLUE);
}

fn random_edge_position() -> Vec2 {
    let w = screen_width();
    let h = screen_height();
    let margin = 20.0;

    match gen_range(0, 4) {
        0 => vec2(gen_range(0.0, w), -margin),    // top
        1 => vec2(gen_range(0.0, w), h + margin), // bottom
        2 => vec2(-margin, gen_range(0.0, h)),    // left
        _ => vec2(w + margin, gen_range(0.0, h)), // right
    }
}
