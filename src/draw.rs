use super::*;

// Draws tile of isometric grid
pub fn draw_tile(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    let pts = [
        Vector2::new(x as f32, y as f32),
        Vector2::new((x + T_WIDTH / 2) as f32, (y - T_HEIGHT / 2) as f32),
        Vector2::new((x + T_WIDTH) as f32, y as f32),
        Vector2::new((x + T_WIDTH / 2) as f32, (y + T_HEIGHT / 2) as f32),
    ];
    d.draw_triangle(pts[0], pts[3], pts[2], color);
    d.draw_triangle(pts[0], pts[2], pts[1], color);

    d.draw_line_ex(pts[0], pts[1], T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[1], pts[2], T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[2], pts[3], T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[3], pts[0], T_BORDER, Color::BLACK);
}

pub fn draw_block(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    let pts = [
        Vector2::new(x as f32, y as f32),
        Vector2::new((x + T_WIDTH) as f32, y as f32),
        Vector2::new((x + T_WIDTH / 2) as f32, (y + T_HEIGHT / 2) as f32),
    ];

    let height = Vector2::new(0., -T_HEIGHT as f32 / 2.);
    draw_tile(d, x, y + height.y as i32, color);

    d.draw_triangle(pts[0], pts[2], pts[2] + height, color);
    d.draw_triangle(pts[0] + height, pts[0], pts[2] + height, color);

    d.draw_triangle(pts[2], pts[1], pts[1] + height, color);
    d.draw_triangle(pts[2] + height, pts[2], pts[1] + height, color);

    d.draw_line_ex(pts[0], pts[2], T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[2], pts[1], T_BORDER, Color::BLACK);

    d.draw_line_ex(pts[0] + height, pts[2] + height, T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[2] + height, pts[1] + height, T_BORDER, Color::BLACK);

    d.draw_line_ex(pts[0], pts[0] + height, T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[1], pts[1] + height, T_BORDER, Color::BLACK);
    d.draw_line_ex(pts[2], pts[2] + height, T_BORDER, Color::BLACK);
}

pub fn draw_plane(d: &mut RaylibDrawHandle, field: &Field) {
    let grass1 = Color::new(124, 181, 24, 255);
    let grass2 = Color::new(92, 128, 1, 255);

    for i in 0..field.width {
        let x0 = field.x - i * T_WIDTH / 2;
        let y0 = field.y + i * T_HEIGHT / 2;
        for j in 0..field.height {
            draw_tile(
                d,
                x0 + j * T_WIDTH / 2,
                y0 + j * T_HEIGHT / 2,
                if (i + j) % 2 == 0 { grass1 } else { grass2 },
            );
        }
    }
}

pub fn draw_robot(d: &mut RaylibDrawHandle, robot: &Robot, field: &Field, sprites: &Texture2D) {
    let robot_pos = coord_to_pos(robot.x, robot.y, &field);

    // Draw sprite
    let texture_rect = match robot.direction {
        (0, -1) => Rectangle {
            x: 300.,
            y: 0.,
            width: 100.,
            height: 100.,
        },
        (1, 0) => Rectangle {
            x: 500.,
            y: 0.,
            width: 100.,
            height: 100.,
        },
        (0, 1) => Rectangle {
            x: 400.,
            y: 0.,
            width: 100.,
            height: 100.,
        },
        (-1, 0) => Rectangle {
            x: 600.,
            y: 0.,
            width: 100.,
            height: 100.,
        },
        _ => Rectangle {
            x: 0.,
            y: 0.,
            width: 100.,
            height: 100.,
        },
    };

    d.draw_texture_pro(
        sprites,
        texture_rect,
        Rectangle {
            x: robot_pos.x,
            y: robot_pos.y - T_WIDTH as f32 + T_HEIGHT as f32 / 2.,
            width: T_WIDTH as f32,
            height: T_WIDTH as f32,
        },
        Vector2::zero(),
        0.,
        robot.color,
    );

    // Health bar
    let health_pos = Vector2::new(robot_pos.x, robot_pos.y + 10.);
    d.draw_rectangle(
        health_pos.x as i32,
        health_pos.y as i32,
        T_WIDTH,
        30,
        Color::RED,
    );
    d.draw_rectangle(
        health_pos.x as i32,
        health_pos.y as i32,
        (T_WIDTH as f32 * (robot.health as f32 / robot.max_health as f32)) as i32,
        30,
        Color::GREEN,
    );
    d.draw_rectangle(
        health_pos.x as i32,
        health_pos.y as i32 + 30,
        (T_WIDTH as f32 * (robot.bullets as f32 / 50.)) as i32,
        10,
        Color::BLUE,
    );

    d.draw_text(
        robot.name.as_str(),
        health_pos.x as i32,
        health_pos.y as i32,
        30,
        Color::BLACK,
    );
}
