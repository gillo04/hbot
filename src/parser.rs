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
    Label(String),
    Nop,
    Mov(Value, Value),

    // Motor
    Fwd,
    Rol,
    Ror,

    // Gun
    Sht,
    Rld,

    // Vision
    See,

    // Control flow
    Jmp(Value),
    Je(Value),
    Jg(Value),
    Jl(Value),

    // Arithmetic and logic
    Add(Value, Value),
    Sub(Value, Value),
    Cmp(Value, Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Not(Value),
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
                    Some(Nop)
                } else {
                    None
                }
            }
            "mov" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Mov(
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
                    Some(Fwd)
                } else {
                    None
                }
            }
            "rol" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Rol)
                } else {
                    None
                }
            }
            "ror" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Ror)
                } else {
                    None
                }
            }

            "sht" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Sht)
                } else {
                    None
                }
            }
            "rld" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(Rld)
                } else {
                    None
                }
            }

            "see" => {
                if check_arg_c(arg_c, 0) {
                    *i += 1;
                    Some(See)
                } else {
                    None
                }
            }

            "jmp" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jmp(token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "je" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Je(token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "jg" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jg(token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }
            "jl" => {
                if check_arg_c(arg_c, 1) {
                    *i += 2;
                    Some(Jl(token_to_value(&toks[j + 1])))
                } else {
                    None
                }
            }

            "add" => {
                if check_arg_c(arg_c, 2) {
                    *i += 4;
                    Some(Add(
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
                    Some(Not(token_to_value(&toks[j + 1])))
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
            })
    {
        *i += 2;
        return Some(Label(toks[j].value.clone()));
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
            if let Label(id) = &inst {
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
            Jmp(val) | Je(val) | Jg(val) | Jl(val) => {
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

    out.retain(|x| if let Label(_) = x { false } else { true });

    for inst in out.iter_mut() {
        println!("{:?}", inst);
    }

    out
}
