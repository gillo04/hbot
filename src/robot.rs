use super::*;

use crate::parser::Instruction::*;
use crate::parser::Value::*;
use crate::parser::*;

pub struct Core {
    pub source: String,
    pub instructions: Vec<Instruction>,
    // Registers
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub ip: i16,

    pub dir: i16,
    pub cx: i16,
    pub cy: i16,

    // Flags
    pub e: bool,
    pub g: bool,
    pub l: bool,
}

impl Default for Core {
    fn default() -> Core {
        Core {
            source: String::from(""),
            instructions: vec![],
            a: 0,
            b: 0,
            c: 0,
            ip: 0,

            dir: 0,
            cx: 0,
            cy: 0,

            e: false,
            g: false,
            l: false,
        }
    }
}

impl Core {
    pub fn compile(&mut self) {
        self.instructions = parse(&self.source);
    }
}

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub direction: (i32, i32),

    pub max_health: i32,
    pub health: i32,
    pub bullets: i32,

    pub team: i32,
    pub color: Color,
    pub name: String,

    pub core: Core,
    pub gun_loaded: bool,
    pub aoi: Vec<(i32, i32)>, // Area of influence
}

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            x: 0,
            y: 0,
            direction: (0, 1),

            max_health: 100,
            health: 100,
            bullets: 100,

            team: 1,
            color: Color::RED,
            name: String::from("Unnamed"),

            core: Core {
                ..Default::default()
            },
            gun_loaded: true,
            aoi: vec![],
        }
    }
}

enum Action {
    None,
    Move(i32, i32),
    Shoot(usize), // Index of target robot
}

impl Robot {
    pub fn resolve_value(&self, val: &Value) -> i16 {
        match val {
            Immediate(imm) => *imm,
            Identifier(id) => 0, // TODO: this shouldn't happen in the future
            Register(reg) => match reg.as_str() {
                "a" => self.core.a,
                "b" => self.core.b,
                "c" => self.core.c,
                "ip" => self.core.ip,
                _ => {
                    println!("Invalid register");
                    0
                }
            },
        }
    }

    pub fn store_value(&mut self, dest: &Value, src: i16) {
        if let Register(reg) = dest {
            match reg.as_str() {
                "a" => self.core.a = src,
                "b" => self.core.b = src,
                "c" => self.core.c = src,
                "ip" => self.core.ip = src,

                "dir" => self.core.dir = src,
                "cx" => self.core.cx = src,
                "cy" => self.core.cy = src,
                _ => {
                    println!("Invalid register");
                }
            };
            return;
        }
        dbg!("Dest is not a register");
    }

    pub fn update_flags(&mut self, val: i16) {
        if val == 0 {
            self.core.e = true;
            self.core.g = false;
            self.core.l = false;
        } else if val > 0 {
            self.core.e = false;
            self.core.g = true;
            self.core.l = false;
        } else {
            self.core.e = false;
            self.core.g = false;
            self.core.l = true;
        }
    }

    pub fn draw_core(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(50, 50, 800, 2000, Color::BLACK);
        d.draw_text(
            format!(
                "A: {:0>5} B: {:0>5} C: {:0>5} IP: {:0>5}",
                self.core.a, self.core.b, self.core.c, self.core.ip
            )
            .as_str(),
            50,
            50,
            40,
            Color::WHITE,
        );

        let line: usize = match self.core.instructions[self.core.ip as usize] {
            Nop(line) => line,
            Mov(line, _, _) => line,
            Add(line, _, _) => line,
            Sub(line, _, _) => line,
            Cmp(line, _, _) => line,
            And(line, _, _) => line,
            Or(line, _, _) => line,
            Xor(line, _, _) => line,
            Not(line, _) => line,

            Jmp(line, _) => line,
            Je(line, _) => line,
            Jg(line, _) => line,
            Jl(line, _) => line,

            Fwd(line) => line,
            Rol(line) => line,
            Ror(line) => line,

            Sht(line) => line,
            Rld(line) => line,

            Rad(line) => line,
            Chk(line) => line,
            _ => 0,
        };

        let ip_y = 90 + line as i32 * 60;
        d.draw_rectangle(50, ip_y, 800, 40, Color::GRAY);
        d.draw_text(self.core.source.as_str(), 50, 90, 40, Color::WHITE);
    }
}

pub fn step_robot(i: usize, robots: &mut Vec<Robot>, field: &Field) -> Action {
    let mut action = Action::None;
    if robots[i].core.instructions.len() == 0 {
        return action;
    }
    if robots[i].core.ip as usize >= robots[i].core.instructions.len() {
        robots[i].core.ip = 0;
    }
    let inst = robots[i].core.instructions[robots[i].core.ip as usize].clone();
    match inst {
        // General purpouse
        Nop(_) => {}

        Mov(_, dest, src) => {
            let src = robots[i].resolve_value(&src);
            robots[i].store_value(&dest, src);
        }

        Add(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) + robots[i].resolve_value(&src);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        Sub(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) - robots[i].resolve_value(&src);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        Cmp(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) - robots[i].resolve_value(&src);
            robots[i].update_flags(res);
        }

        And(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) & robots[i].resolve_value(&src);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        Or(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) | robots[i].resolve_value(&src);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        Xor(_, dest, src) => {
            let res = robots[i].resolve_value(&dest) ^ robots[i].resolve_value(&src);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        Not(_, dest) => {
            let res = !robots[i].resolve_value(&dest);
            robots[i].store_value(&dest, res);
            robots[i].update_flags(res);
        }

        // Control flow
        Jmp(_, dest) => {
            let dest = robots[i].resolve_value(&dest);
            robots[i].core.ip = dest - 1;
        }

        Je(_, dest) => {
            if robots[i].core.e {
                let dest = robots[i].resolve_value(&dest);
                robots[i].core.ip = dest - 1;
            }
        }

        Jg(_, dest) => {
            if robots[i].core.g {
                let dest = robots[i].resolve_value(&dest);
                robots[i].core.ip = dest - 1;
            }
        }

        Jl(_, dest) => {
            if robots[i].core.l {
                let dest = robots[i].resolve_value(&dest);
                robots[i].core.ip = dest - 1;
            }
        }

        // Motor
        Fwd(_) => {
            action = Action::Move(
                std::cmp::min(
                    field.width - 1,
                    std::cmp::max(0, robots[i].x + robots[i].direction.0),
                ),
                std::cmp::min(
                    field.height - 1,
                    std::cmp::max(0, robots[i].y + robots[i].direction.1),
                ),
            );
        }

        Rol(_) => match robots[i].direction {
            (0, 1) => robots[i].direction = (1, 0),
            (1, 0) => robots[i].direction = (0, -1),
            (0, -1) => robots[i].direction = (-1, 0),
            (-1, 0) => robots[i].direction = (0, 1),
            (_, _) => println!("Robot has impossible direction"),
        },

        Ror(_) => match robots[i].direction {
            (0, 1) => robots[i].direction = (-1, 0),
            (-1, 0) => robots[i].direction = (0, -1),
            (0, -1) => robots[i].direction = (1, 0),
            (1, 0) => robots[i].direction = (0, 1),
            (_, _) => println!("Robot has impossible direction"),
        },

        Sht(_) => {
            if robots[i].bullets > 0 && robots[i].gun_loaded {
                robots[i].bullets -= 1;
                robots[i].gun_loaded = false;

                let x = robots[i].x * robots[i].direction.0;
                let y = robots[i].y * robots[i].direction.1;

                let mut min_dist = i32::MAX;
                let mut target: Option<usize> = None;
                for j in 0..robots.len() {
                    if j == i {
                        continue;
                    }
                    let xe = robots[j].x * robots[i].direction.0;
                    let ye = robots[j].y * robots[i].direction.1;
                    let dist = (x.abs() - xe.abs()).abs() + (y.abs() - ye.abs()).abs();
                    if robots[i].direction.0 == 0 {
                        if robots[i].x == robots[j].x && y < ye && dist < min_dist {
                            min_dist = dist;
                            target = Some(j);
                        }
                    } else {
                        if x < xe && robots[i].y == robots[j].y && dist < min_dist {
                            min_dist = dist;
                            target = Some(j);
                        }
                    }
                }
                if let Some(t) = target {
                    action = Action::Shoot(t);
                }
            }
        }

        Rld(_) => {
            robots[i].gun_loaded = true;
        }

        _ => {
            println!("Unhandled instruction");
        }
    };
    robots[i].core.ip += 1;
    return action;
}

pub fn step_game(robots: &mut Vec<Robot>, field: &Field) {
    let mut actions = vec![];
    for i in 0..robots.len() {
        actions.push(step_robot(i, robots, field));
    }

    // Execute all moves
    for i in 0..robots.len() {
        if let Action::Move(x, y) = actions[i] {
            robots[i].x = x;
            robots[i].y = y;
        }
    }

    // Execute all shots
    for i in 0..robots.len() {
        if let Action::Shoot(target) = actions[i] {
            robots[target].health -= 10;
        }
    }

    robots.retain(|r| r.health > 0);
}
