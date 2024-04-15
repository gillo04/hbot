#[derive(Debug, PartialEq)]
pub enum TokenType {
    Opcode,
    Register,
    Immediate,
    Punctuator,
    Identifier,
}

// Lexical token
#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub t: TokenType,
    pub line: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        return self.value == other.value && self.t == other.t;
    }
}

fn lex_immediate(source: &String, i: &mut usize) -> Option<Token> {
    let mut token = Token {
        value: String::from(""),
        t: TokenType::Immediate,
        line: 0,
    };

    loop {
        let char = source.chars().nth(*i);
        if let Some(c) = char {
            if c.is_digit(10) {
                token.value.push(c);
                *i += 1;
                continue;
            } else if token.value == "" {
                return None;
            }
        }
        return Some(token);
    }
}

fn lex_identifier(source: &String, i: &mut usize) -> Option<Token> {
    let mut j = *i;
    let mut token = Token {
        value: String::from(""),
        t: TokenType::Identifier,
        line: 0,
    };

    loop {
        let char = source.chars().nth(j);
        if let Some(c) = char {
            if c.is_alphanumeric() {
                token.value.push(c);
                j += 1;
                continue;
            } else if token.value == "" {
                return None;
            }
        }
        *i = j;
        return Some(token);
    }
}

fn lex_punctuator(source: &String, i: &mut usize) -> Option<Token> {
    let mut j = *i;
    let mut token = Token {
        value: String::from(""),
        t: TokenType::Punctuator,
        line: 0,
    };

    let punctuators = vec![String::from(","), String::from(":")];

    let char = source.chars().nth(j);
    if let Some(c) = char {
        token.value.push(c);
        if punctuators.contains(&token.value) {
            j += 1;
        } else if token.value == "" {
            return None;
        }
    }
    *i = j;
    return Some(token);
}

pub fn lex(source: &String) -> Vec<Token> {
    let mut i = 0;
    let mut tokens: Vec<Token> = vec![];

    let opcodes = vec![
        String::from("nop"),
        String::from("mov"),
        // Motor
        String::from("fwd"),
        String::from("rol"),
        String::from("ror"),
        // Gun
        String::from("sht"),
        String::from("rld"),
        // Vision
        String::from("see"),
        // Control flow
        String::from("jmp"),
        String::from("je"),
        String::from("jg"),
        String::from("jl"),
        // Arithmetic and logic
        String::from("add"),
        String::from("sub"),
        String::from("cmp"),
        String::from("and"),
        String::from("or"),
        String::from("xor"),
        String::from("not"),
    ];

    let registers = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("x"),
        String::from("y"),
        String::from("ip"),
    ];

    let mut current_line = 0;
    loop {
        // Eat white space
        loop {
            let char = source.chars().nth(i);
            if let Some(c) = char {
                if c == '\n' {
                    current_line += 1;
                }
                if c.is_whitespace() {
                    i += 1;
                    continue;
                }
            }
            break;
        }

        // Lex next token
        let char = source.chars().nth(i);
        if let Some(_) = char {
            // Immediate values
            let tmp = lex_immediate(&source, &mut i);
            if let Some(mut token) = tmp {
                token.line = current_line;
                tokens.push(token);
                continue;
            }

            // Identifiers
            let tmp = lex_identifier(&source, &mut i);
            if let Some(mut token) = tmp {
                token.value = token.value.to_lowercase();
                token.line = current_line;
                if opcodes.contains(&token.value) {
                    token.t = TokenType::Opcode;
                }

                if registers.contains(&token.value) {
                    token.t = TokenType::Register;
                }

                tokens.push(token);
                continue;
            }

            // Punctuators
            let tmp = lex_punctuator(&source, &mut i);
            if let Some(mut token) = tmp {
                token.line = current_line;
                tokens.push(token);
                continue;
            }
        }
        return tokens;
    }
}
