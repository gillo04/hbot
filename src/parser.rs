pub mod lexer;

use lexer::lex;

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Immediate(i16),
    Register(String),
}

// Abstract syntax tree
#[derive(Debug, Clone)]
pub enum Instruction {
    Label(usize, String),
    Nop(usize),
    Mov(usize, Value, Value),

    // Motor
    Fwd(usize),
    Rol(usize),
    Ror(usize),

    // Gun
    Sht(usize),
    Rld(usize),

    // Vision
    See(usize),

    // Control flow
    Jmp(usize, Value),
    Je(usize, Value),
    Jg(usize, Value),
    Jl(usize, Value),

    // Arithmetic and logic
    Add(usize, Value, Value),
    Sub(usize, Value, Value),
    Cmp(usize, Value, Value),
    And(usize, Value, Value),
    Or(usize, Value, Value),
    Xor(usize, Value, Value),
    Not(usize, Value),
}

use lexer::*;
use Instruction::*;
use Value::*;

fn check_arg_c(arg_c: usize, expected: usize) -> bool {
    if arg_c < expected {
        println!("Error: Expected {} arguments, found {}", expected, arg_c);
        return false;
    }
    return true;
}

fn token_to_value(tok: &Token) -> Value {
    match tok.t {
        TokenType::Register => Register(tok.value.clone()),
        TokenType::Immediate => Immediate(tok.value.parse::<i16>().unwrap()),
        TokenType::Identifier => Identifier(tok.value.clone()),
        _ => {
            println!("Couln't convert token {} to value", tok.value);
            Identifier(String::from("ERROR"))
        }
    }
}

fn p_instruction(toks: &Vec<Token>, i: &mut usize) -> Option<Instruction> {
    let j = *i;
    if toks.len() > j && toks[j].t == TokenType::Opcode {
        let mut arg_c = 0;
        if toks.len() > j + 1
            && toks[j + 1].t != TokenType::Opcode
            && toks[j + 1].t != TokenType::Punctuator
        {
            if toks.len() > j + 3
                && toks[j + 2]
                    == (Token {
                        value: String::from(","),
                        t: TokenType::Punctuator,
                        line: 0,
                    })
                && (toks[j + 3].t != TokenType::Opcode && toks[j + 3].t != TokenType::Punctuator)
            {
                // Instruction with two args
                arg_c = 2;
            } else {
                // Instruction with one arg
                arg_c = 1;
            }
        }

        return match toks[j].value.as_str() {
            "nop" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Nop(toks[j].line))
                } else {
                    None
                }
            }
            "mov" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Mov(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }

            "fwd" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Fwd(toks[j].line))
                } else {
                    None
                }
            }
            "rol" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Rol(toks[j].line))
                } else {
                    None
                }
            }
            "ror" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Ror(toks[j].line))
                } else {
                    None
                }
            }

            "sht" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Sht(toks[j].line))
                } else {
                    None
                }
            }
            "rld" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Rld(toks[j].line))
                } else {
                    None
                }
            }

            "see" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(See(toks[j].line))
                } else {
                    None
                }
            }

            "jmp" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jmp(toks[j].line, token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "je" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Je(toks[j].line, token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "jg" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jg(toks[j].line, token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "jl" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jl(toks[j].line, token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }

            "add" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Add(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "sub" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Sub(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "cmp" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Cmp(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "and" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(And(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "or" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Or(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "xor" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Xor(
                        toks[j].line,
                        token_to_value(&toks[j + 1]),
                        token_to_value(&toks[j + 3]),
                    ))
                } else {
                    None
                }
            }
            "not" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Not(toks[j].line, token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            _ => None,
        };
    }

    return None;
}

fn p_label(toks: &Vec<Token>, i: &mut usize) -> Option<Instruction> {
    let j = *i;
    if toks.len() > j + 1
        && toks[j].t == TokenType::Identifier
        && toks[j + 1]
            == (Token {
                t: TokenType::Punctuator,
                value: String::from(":"),
                line: 0,
            })
    {
        *i += 2;
        return Some(Label(toks[j].line, toks[j].value.clone()));
    }
    return None;
}

pub fn parse(source: &String) -> Vec<Instruction> {
    let tokens = lex(&source);
    let mut out: Vec<Instruction> = vec![];
    let mut i = 0;

    let mut label_table: Vec<(String, i16)> = Vec::new();
    let mut inst_count = 0;

    // Parse instructions
    loop {
        if let Some(inst) = p_instruction(&tokens, &mut i) {
            inst_count += 1;
            out.push(inst);
            continue;
        }

        if let Some(inst) = p_label(&tokens, &mut i) {
            if let Label(_, id) = &inst {
                label_table.push((id.clone(), inst_count))
            }
            out.push(inst);
            continue;
        }

        break;
    }

    // Resolve labels
    for inst in out.iter_mut() {
        match inst {
            Jmp(_, val) | Je(_, val) | Jg(_, val) | Jl(_, val) => {
                if let Identifier(id) = val {
                    let mut ip: i16 = 0;
                    for (string, address) in &label_table {
                        if *string == *id {
                            ip = *address;
                        }
                    }
                    *val = Immediate(ip);
                }
            }

            _ => (),
        }
    }

    out.retain(|x| if let Label(_, _) = x { false } else { true });

    for inst in out.iter_mut() {
        println!("{:?}", inst);
    }

    out
}
