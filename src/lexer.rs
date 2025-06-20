use crate::token::{Token, TokenType};
use crate::utils::{CharIter, Pos};

fn try_convert_escape_sequence<'a>(chars: &mut CharIter, pos: &'a mut Pos) -> char {
    let ch = match chars.next() {
        Some(x) => x,
        None => panic!("Found EOF when trying to parse escape sequence. {}", pos)
    };

    let converted = match ch {
        // Literal characters we want to escape.
        '"' | '\\' | '/' => ch,

        // Special whitespace
        'b' => '\x08',
        'f' => '\x0c',

        // Generic whitespace
        'n' => '\n',
        'r' => '\r',
        't' => '\t',

        // Unicode escape sequences
        'u' => {
            let mut hex = String::new();

            for _ in 0..4 {
                match chars.next() {
                    Some(ch) => match ch {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => hex.push(ch),
                        _ => panic!("Invalid character for unicode codepoint: {:?} {}", ch, pos)
                    },
                    None => panic!("Found EOF when trying to convert escape sequence. {}", pos)
                };

                pos.column += 3;
            };

            // We've already verified the hex digits with the match statement,
            // so we can safely unwrap on both cases.
            char::from_u32(u32::from_str_radix(hex.as_str(), 16).unwrap()).unwrap()
        }

        _ => panic!("Invalid escape sequence {:?} {}", ch, pos)
    };

    pos.column += 1;
    
    converted
}

fn try_get_string<'a>(chars: &mut CharIter, pos: &'a mut Pos) -> Token {
    // We know for sure that the first character is a double quote.
    let mut result = String::from(chars.next().unwrap());

    let (line_no, col_no) = (pos.line, pos.column);
    
    loop {
        // Since an EOF results in an unterminated string literal,
        // this is a fatal error and we cannot tokenise the object.
        let ch = match chars.peek() {
            Some(x) => x,
            None => panic!("Found EOF when trying to parse string. {}", pos)
        };

        match ch {
            '\n' => panic!("Found newline when trying to parse string. {}", pos),
            
            // Escape whatever character is after.
            // TODO: Add 'try_convert_escape_sequence()'
            '\\' => {
                chars.next();

                pos.column += 1;
                
                result.push(try_convert_escape_sequence(chars, pos));
            },

            // The string is completed.
            '"' => {
                result.push(chars.next().unwrap());

                return Token::new(
                    TokenType::String,
                    result,
                    line_no,
                    col_no
                );
            }
            
            // Anything else just goes in the string.
            c => {
                result.push(c);
                chars.next();

                pos.column += 1;
            }
        }
    }
}

fn try_grab_integer(chars: &mut CharIter, pos: &mut Pos) -> String {
    let first = chars.next().unwrap();
    let mut result = String::from(first);

    // If we have a negative sign and there is no number after it,
    // this is a fatal EOF error which we need to check for.
    if first == '-' {
        match chars.peek() {
            Some(x) => match x {
                '0'..='9' => result.push(x),
                _ => panic!("Found non-digit after minus sign when trying to parse number. {}", pos)
            },
            None => panic!("Encountered an EOF when trying to parse number. {}", pos)
        };
    }
    
    loop {
        // Since an EOF when parsing an integer isn't fatal,
        // we can let any EOFs we encounter pass silently
        // by breaking.
        match chars.peek() {
            Some(ch) => match ch {
                '0'..='9' => {
                    result.push(ch);
                    chars.next();
                },
                _ => break
            },
            None => break
        }
    }

    result
}

fn try_grab_exponent(chars: &mut CharIter, pos: &mut Pos) -> String {
    chars.next();
    
    let mut result = String::from('e');

    match chars.peek() {
        Some(ch) => match ch {
            '0'..='9' | '-' => {
                result.push_str(try_grab_integer(chars, pos).as_str());
            },
            _ => panic!("Found non-digit after minus sign when trying to parse exponent. {}", pos)
        },
        None => panic!("Encountered EOF when trying to parse exponent of number. {}", pos)
    }

    result
}

fn try_get_number(chars: &mut CharIter, pos: &mut Pos) -> Token {
    // Get the integer body of the number.
    let mut result = try_grab_integer(chars, pos);

    let next = chars.peek();
    
    // If we've encountered an EOF, that's the full number.
    if next.is_none() {
        return Token::new(
            TokenType::Int,
            result,
            pos.line,
            pos.column
        );
    }

    match next.unwrap() {
        // If we have an integer and exponent like '1e5',
        // we need to verify and append the exponent.
        'e' | 'E' => {
            result.push_str(try_grab_exponent(chars, pos).as_str());
        },

        // If we have a decimal like '5.6',
        // we need to verify and append the decimal part.
        '.' => {
            result.push(chars.next().unwrap());
            
            match chars.peek() {
                Some(ch) => match ch {
                    '0'..='9' => result.push_str(try_grab_integer(chars, pos).as_str()),
                    _ => panic!("Found non-digit after decimal point when trying to parse exponent. {}", pos)
                },
                None => panic!("Encountered EOF when trying to parse decimal part of a number. {}", pos)
            }

            // If there's an exponent part, we need that as well.
            // If nothing's there, we can just pass quietly.
            match chars.peek() {
                Some(ch) => match ch {
                    'e' | 'E' => result.push_str(try_grab_exponent(chars, pos).as_str()),
                    _ => {}
                },
                None => {}
            }

            return Token::new(
                TokenType::Float,
                result,
                pos.line,
                pos.column
            );
        },
        _ => {}
    }

    Token::new(
        TokenType::Int,
        result,
        pos.line,
        pos.column
    )
}

fn try_get_name(chars: &mut CharIter, pos: &mut Pos) -> Token {
    // The first character is safe.
    let mut result = String::from(chars.next().unwrap());

    // Grab any valid variable name characters.
    loop {
        match chars.peek() {
            Some(ch) => match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    result.push(ch);
                    chars.next();

                    pos.column += 1;
                },
                _ => break
            },
            None => break
        }
    }

    Token::new(
        TokenType::Name,
        result,
        pos.line,
        pos.column
    )
}

pub fn tokenise(text: &str) -> Vec<Token> {
    let mut chars = CharIter::new(text);

    let mut tokens: Vec<Token> = vec![];
    
    let mut pos = Pos {
        line: 1,
        column: 1
    };

    loop {
        let ch = match chars.peek() {
            Some(x) => x,
            None => break
        };

        let token: Token = match ch {
            // Newlines are special whitespace because they indicate
            // we need to go to the next line.
            '\n' => {
                chars.next();

                pos.line += 1;
                pos.column = 0;

                continue;
            },

            // All other whitespace is irrelevant, so we can skip it.
            ' ' | '\t' | '\r' => {
                chars.next();

                pos.column += 1;
                continue;
            },

            '"'                         => try_get_string(&mut chars, &mut pos),
            '0'..='9' | '-'             => try_get_number(&mut chars, &mut pos),
            'a'..='z' | 'A'..='Z' | '_' => try_get_name(&mut chars, &mut pos),

            '{' | '}' | '[' | ']' | ',' | ':' => {
                chars.next();
                pos.column += 1;

                Token::new(
                    match ch {
                        '{' => TokenType::LBrace,
                        '}' => TokenType::RBrace,
                        '[' => TokenType::LSqBrac,
                        ']' => TokenType::RSqBrac,
                        ',' => TokenType::Comma,
                        ':' => TokenType::Colon,
                        _ => todo!()
                    },
                    ch.to_string(),
                    pos.line,
                    pos.column
                )
            }
            
            c => panic!("Unrecognised character: {:?}", c)
        };

        tokens.push(token);
    }

    tokens
}