pub mod draw;
pub mod parser;
pub mod robot;

use raylib::prelude::*;
use rfd::FileDialog;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;

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
fn pos_to_coord(pos: Vector2, field: &Field) -> Option<(i32, i32)> {
    let x = pos.x - field.x as f32;
    let y = pos.y - field.y as f32;

    let j = ((x / T_WIDTH as f32) + (y / T_HEIGHT as f32)) as i32;
    let i = ((y / T_HEIGHT as f32) - (x / T_WIDTH as f32) + 1.) as i32;

    if j >= 0 && j < field.width && i >= 0 && i < field.height {
        return Some((j, i));
    }

    return None;
}

#[derive(PartialEq)]
enum GameState {
    Normal,
    Simulating,
    AddingRobot,
}

enum AnimState {
    Idle,
    Moving,
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
        aoi: vec![(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)],
        ..Default::default()
    });
    robots[0].core.source =
        fs::read_to_string("space_invader.hasm").expect("Error loading program");
    robots[0].core.compile();

    robots.push(Robot {
        x: 5,
        y: 0,
        direction: (0, 1),
        team: 1,
        color: enemy_color,
        name: String::from("Robot 2"),
        ..Default::default()
    });
    robots[1].core.source = fs::read_to_string("turret.hasm").expect("Error loading program");
    robots[1].core.compile();

    let mut game_state = GameState::Normal;
    let mut selected_robot: Option<usize> = None;

    let anim_duration = 0.1;
    let mut sim_time = rl.get_time();
    let mut animation_state = AnimState::Idle;
    let mut rob_pos_snapshot = vec![];
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

        let mouse = rl.get_mouse_position();
        let hover_tile = pos_to_coord(mouse, &field);
        let time = rl.get_time();

        // Handle input and update
        match game_state {
            GameState::Normal => {
                if rl.is_key_released(KeyboardKey::KEY_ENTER) {
                    animation_state = AnimState::Moving;
                    rob_pos_snapshot.clear();
                    for r in robots.iter() {
                        rob_pos_snapshot.push((r.x, r.y));
                    }
                    sim_time = rl.get_time();
                    step_game(&mut robots, &field);
                }

                if rl.is_key_released(KeyboardKey::KEY_SPACE) {
                    game_state = GameState::Simulating;
                }
            }

            GameState::Simulating => {
                if time - sim_time >= anim_duration {
                    animation_state = AnimState::Moving;
                    rob_pos_snapshot.clear();
                    for r in robots.iter() {
                        rob_pos_snapshot.push((r.x, r.y));
                    }
                    sim_time = rl.get_time();
                    step_game(&mut robots, &field);
                }

                if rl.is_key_released(KeyboardKey::KEY_SPACE) {
                    game_state = GameState::Normal;
                }
            }

            GameState::AddingRobot => {}
        }

        // Handle animation state
        if time - sim_time >= anim_duration {
            animation_state = AnimState::Idle;
        }

        // Handle robot selection
        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            match game_state {
                GameState::AddingRobot => {
                    let files = FileDialog::new()
                        .add_filter("text", &["txt", "hasm"])
                        .set_directory("/")
                        .pick_file();

                    if let Some(path) = files {
                        let source = fs::read_to_string(path).expect("Error loading program");
                        if let Some(hover_tile) = hover_tile {
                            robots.push(Robot {
                                x: hover_tile.0,
                                y: hover_tile.1,
                                direction: (0, -1),
                                team: -1,
                                color: player_color,
                                name: String::from("new"),
                                core: Core {
                                    source,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                            let last = robots.len() - 1;
                            robots[last].core.compile();
                        }
                    }
                    game_state = GameState::Normal;
                }

                _ => {
                    if let Some(hover_tile) = hover_tile {
                        selected_robot = None;
                        let mut found_robot = false;
                        for (i, robot) in robots.iter().enumerate() {
                            if robot.x == hover_tile.0 && robot.y == hover_tile.1 {
                                selected_robot = Some(i);
                                found_robot = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(8, 189, 189, 255));

        // Draw UI
        d.gui_set_style(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_SIZE as i32,
            50,
        );

        // Start/stop button
        let sim_btn_str = match game_state {
            GameState::Simulating => CStr::from_bytes_with_nul(b"Stop\0"),
            _ => CStr::from_bytes_with_nul(b"Start\0"),
        }
        .expect("Failed to create CStr");

        if d.gui_button(
            Rectangle {
                x: 1000.,
                y: 0.,
                width: 300.,
                height: 100.,
            },
            Some(sim_btn_str),
        ) {
            match game_state {
                GameState::Normal => game_state = GameState::Simulating,
                GameState::Simulating => game_state = GameState::Normal,
                _ => (),
            };
        }

        // Step button
        if d.gui_button(
            Rectangle {
                x: 1300.,
                y: 0.,
                width: 300.,
                height: 100.,
            },
            Some(CStr::from_bytes_with_nul(b"Step\0").expect("Failed to create CStr")),
        ) {
            if game_state == GameState::Normal {
                step_game(&mut robots, &field);
            }
        }

        // Add button
        if d.gui_button(
            Rectangle {
                x: 1600.,
                y: 0.,
                width: 300.,
                height: 100.,
            },
            Some(CStr::from_bytes_with_nul(b"Add robot\0").expect("Failed to create CStr")),
        ) {
            game_state = GameState::AddingRobot;
        }

        // Draw play field
        // robots[0].draw_core(&mut d);
        draw_plane(&mut d, &field);

        // Draw robots
        match animation_state {
            AnimState::Idle => {
                for (i, robot) in robots.iter().enumerate() {
                    draw_robot(&mut d, &robot, &field, &sprites, (0, 0), 1.);
                }
            }

            AnimState::Moving => {
                for (i, robot) in robots.iter().enumerate() {
                    draw_robot(
                        &mut d,
                        &robot,
                        &field,
                        &sprites,
                        rob_pos_snapshot[i],
                        (time - sim_time) as f32 / anim_duration as f32,
                    );
                }
            }
        }

        // Draw hover tile
        if let Some(hover_tile) = hover_tile {
            /*if game_state == GameState::AddingRobot {
                draw_robot_info(&mut d, &robots[r], &sprites);
            }*/

            let selection_pos = coord_to_pos(hover_tile.0, hover_tile.1, &field);
            draw_block(
                &mut d,
                selection_pos.x as i32,
                selection_pos.y as i32,
                Color::new(255, 0, 0, ((time * 4.).sin() * 40. + 130.) as u8),
            );
        }

        // Draw UI
        if let Some(r) = selected_robot {
            let selection_pos = coord_to_pos(robots[r].x, robots[r].y, &field);
            draw_block(
                &mut d,
                selection_pos.x as i32,
                selection_pos.y as i32,
                Color::new(255, 0, 0, 130 as u8),
            );
            draw_robot_info(&mut d, &robots[r], &sprites);
        }
    }
}
