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

pub fn draw_robot(d: &mut RaylibDrawHandle, robot: &Robot, field: &Field) {
    let robot_pos = coord_to_pos(robot.x, robot.y, &field);
    let direction_pos = coord_to_pos(
        robot.x + robot.direction.0,
        robot.y + robot.direction.1,
        &field,
    );
    draw_tile(
        d,
        direction_pos.x as i32,
        direction_pos.y as i32,
        Color::RED,
    );
    draw_block(d, robot_pos.x as i32, robot_pos.y as i32, robot.color);
    d.draw_text(
        robot.name.as_str(),
        robot_pos.x as i32 + 50,
        robot_pos.y as i32 - T_HEIGHT / 2,
        30,
        Color::BLACK,
    );

    // Health bar
    d.draw_rectangle(
        robot_pos.x as i32,
        robot_pos.y as i32 + 10,
        T_WIDTH,
        10,
        Color::RED,
    );
    d.draw_rectangle(
        robot_pos.x as i32,
        robot_pos.y as i32 + 10,
        (T_WIDTH as f32 * (robot.health as f32 / robot.max_health as f32)) as i32,
        10,
        Color::GREEN,
    );
}
