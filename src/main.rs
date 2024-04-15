pub mod draw;
pub mod parser;
pub mod robot;

use raylib::prelude::*;
use std::fs;

use draw::*;
use robot::*;

const T_WIDTH: i32 = 200;
const T_HEIGHT: i32 = T_WIDTH / 2;
const T_BORDER: f32 = 2.;

struct Field {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn coord_to_pos(j: i32, i: i32, field: &Field) -> Vector2 {
    let x0 = field.x - i * T_WIDTH / 2;
    let y0 = field.y + i * T_HEIGHT / 2;
    Vector2::new(
        (x0 + j * T_WIDTH / 2) as f32,
        (y0 + j * T_HEIGHT / 2) as f32,
    )
}

// Converts screen position to isometric grid coordinates
// The tiles have their origin on the left corner
fn pos_to_coord(pos: Vector2, field: &Field) -> (i32, i32) {
    let x = pos.x - field.x as f32;
    let y = pos.y - field.y as f32;

    let j = (x / T_WIDTH as f32) + (y / T_HEIGHT as f32);
    let i = (y / T_HEIGHT as f32) - (x / T_WIDTH as f32) + 1.;

    (j as i32, i as i32)
}

enum GameState {
    Normal,
    Simulating,
}

fn main() {
    // Setup raylib
    let (mut rl, thread) = raylib::init()
        .size(2000, 1500)
        .title("HBot")
        .resizable()
        .build();

    rl.set_target_fps(60);

    let sprites = Image::load_image("sprites.png").expect("Error loading sprites");
    let sprites = rl
        .load_texture_from_image(&thread, &sprites)
        .expect("Failed to convert image to texture");

    // Setup robots
    let player_color = Color::new(251, 97, 7, 255);
    let enemy_color = Color::new(0, 127, 255, 255);

    let mut robots: Vec<Robot> = Vec::new();

    robots.push(Robot {
        x: 0,
        y: 9,
        direction: (0, -1),
        team: -1,
        color: player_color,
        name: String::from("Robot"),
        ..Default::default()
    });
    robots[0].core.source =
        fs::read_to_string("space_invader.hasm").expect("Error loading program");
    robots[0].core.compile();

    robots.push(Robot {
        x: 8,
        y: 3,
        direction: (0, 1),
        team: 1,
        color: enemy_color,
        name: String::from("Robot 3"),
        ..Default::default()
    });
    robots.push(Robot {
        x: 8,
        y: 0,
        direction: (0, 1),
        team: 1,
        color: enemy_color,
        name: String::from("Robot 2"),
        ..Default::default()
    });

    /*robots[1].core.source = fs::read_to_string("turret.hasm").expect("Error loading program");
    robots[1].core.compile();*/

    let mut game_state = GameState::Normal;
    let mut sim_time = rl.get_time();
    while !rl.window_should_close() {
        let center = Vector2::new(
            rl.get_screen_width() as f32 / 2.0,
            rl.get_screen_height() as f32 / 2.0,
        );
        let grid_pos = center + Vector2::new((-T_WIDTH / 2) as f32, (-T_HEIGHT / 2 * 10) as f32);
        let field = Field {
            x: grid_pos.x as i32,
            y: grid_pos.y as i32,
            width: 10,
            height: 10,
        };

        // Handle input and update
        match game_state {
            GameState::Normal => {
                if rl.is_key_released(KeyboardKey::KEY_ENTER) {
                    for i in 0..robots.len() {
                        step(i, &mut robots, &field);
                    }
                }

                if rl.is_key_released(KeyboardKey::KEY_SPACE) {
                    game_state = GameState::Simulating;
                }
            }

            GameState::Simulating => {
                if rl.get_time() - sim_time >= 0.1 {
                    for i in 0..robots.len() {
                        step(i, &mut robots, &field);
                    }
                    sim_time = rl.get_time();
                }

                if rl.is_key_released(KeyboardKey::KEY_SPACE) {
                    game_state = GameState::Normal;
                }
            }
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(8, 189, 189, 255));

        draw_plane(&mut d, &field);

        let pos = coord_to_pos(3, 2, &field);
        // draw_block(&mut d, pos.x as i32, pos.y as i32, player_color);

        /*let mouse = d.get_mouse_position();
        let (j, i) = pos_to_coord(mouse, &field);
        let selection_pos = coord_to_pos(j, i, &field);*/

        // Draw robots
        for robot in robots.iter() {
            draw_robot(&mut d, &robot, &field, &sprites);
        }
        robots[0].draw_core(&mut d);
    }
}
