use std::collections::HashMap;
use std::fmt::Debug;

use crate::token::{Token, TokenType as TT};
use crate::utils::TokenIter;

pub enum Node {
    Integer(i64),
    String(String),
    Float(f64),
    Bool(bool),
    Null,

    Array(Vec<Node>),
    Object(HashMap<String, Node>),

    Empty
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Integer(n) => n.to_string(),
            Self::String(s) => s.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Null => "null".to_string(),
            Self::Array(arr) => format!("{:?}", arr),
            Self::Object(map) => {
                let parts: Vec<String> = map.iter().map(
                    |(name, value)| format!("{}: {:?}", name, value)
                ).collect();

                format!("{{{}}}", parts.join(", "))
            },
            Self::Empty => "EMPTY".to_string()
        };
        
        write!(f, "{}", s).unwrap();

        Ok(())
    }
}

fn parse_simple(token: &Token) -> Node {
    match token.tok_type {
        TT::Int => {
            let result = match str::parse::<i64>(&token.value) {
                Ok(x) => x,
                Err(_) => panic!("Failed to parse integer token's internal value: {}", token)
            };

            Node::Integer(result)
        },

        TT::Float => {
            let result = match str::parse::<f64>(&token.value) {
                Ok(x) => x,
                Err(_) => panic!("Failed to parse float token's internal value: {}", token)
            };

            Node::Float(result)
        },
        
        TT::String => Node::String(token.value.clone()),

        TT::Name => match token.value.as_str() {
            "true"  => Node::Bool(true),
            "false" => Node::Bool(false),
            "null"  => Node::Null,

            _ => panic!("Failed to parse undefined name: {:?} ({})", token.value, token)
        },

        _ => panic!("Cannot parse token with invalid type: {}", token)
    }
}

fn parse_array(tokens: &mut TokenIter) -> Node {
    let mut body: Vec<Node> = Vec::new();
    
    // This is safe.
    let start = tokens.next().unwrap();

    loop {
        let token = match tokens.peek() {
            Some(x) => x,
            None => panic!("Encountered an EOF while trying to build array. {}", start.pos())
        };

        let node: Node = match token.tok_type {
            TT::LSqBrac => parse_array(tokens),
            TT::LBrace  => parse_object(tokens),

            TT::RSqBrac => break,
            
            TT::Int | TT::String | TT::Float | TT::Name => parse_simple(tokens.next().unwrap()),

            _ => panic!("Invalid token for an array: {}", token)
        };

        body.push(node);

        let next = match tokens.peek() {
            Some(t) => t,
            None => panic!("Encountered an EOF while trying to build array. {}", start.pos())
        };

        match next.tok_type {
            TT::Comma => {
                tokens.next();
            }

            TT::RSqBrac => {
                tokens.next();
                break;
            }

            _ => panic!("Unrecognised token after parsing array item: {} {}", token, token.pos())
        }
    }

    Node::Array(body)
}

fn parse_pair(tokens: &mut TokenIter, start: &Token) -> (String, Node) {
    // Get the string key
    let name = match tokens.next() {
        Some(t) => {
            if t.tok_type != TT::String {
                panic!("Expected a property name (string), got back the token {} {}", t, start.pos())
            }

            t.value.clone()
        }
        None => panic!("Encountered an EOF while trying to build object property. {}", start.pos())
    };

    // Check for a colon
    match tokens.next() {
        Some(t) => {
            if t.tok_type != TT::Colon {
                panic!("Expected a colon, got back the token {} {}", t, start.pos())
            }
        },
        None => panic!("Encountered an EOF while trying to build object property. {}", start.pos())
    };

    let peeked = match tokens.peek() {
        Some(t) => t,
        None => panic!("Encountered an EOF while trying to build object property. {}", start.pos())
    };

    let value = match peeked.tok_type {
        TT::LBrace  => parse_object(tokens),
        TT::LSqBrac => parse_array(tokens),
        TT::Int | TT::String | TT::Float | TT::Name => parse_simple(tokens.next().unwrap()),

        _ => panic!("Invalid token for an object property: {}", peeked)
    };

    (name, value)
}

fn parse_object(tokens: &mut TokenIter) -> Node {
    let mut body: HashMap<String, Node> = HashMap::new();

    // This will always be a '{'
    let mut start = tokens.next().unwrap();

    // This is the end of the object
    start = match tokens.peek() {
        Some(t) => {
            if t.tok_type == TT::RBrace {
                return Node::Object(body);
            }

            t
        },
        None => panic!("Encountered an EOF when trying to parse object. {}", start.pos())
    };

    let (name, value) = parse_pair(tokens, start);

    body.insert(name, value);

    loop {
        start = match tokens.next() {
            Some(t) => t,
            None => panic!("Encountered an EOF when trying to parse object pair. {}", start.pos())
        };

        match start.tok_type {
            TT::RBrace => break,
            TT::Comma  => {
                let (name, value) = parse_pair(tokens, start);

                body.insert(name, value);
            },

            _ => panic!("Encountered invalid token when trying to parse object. {} {}", start, start.pos())
        }
    }

    Node::Object(body)
}

pub fn parse(token_vec: &Vec<Token>) -> Node {
    let mut tokens = TokenIter::new(&token_vec);

    let first = match tokens.peek() {
        Some(t) => t,
        None => return Node::Empty
    };

    let out = match first.tok_type {
        TT::Int | TT::Float | TT::String | TT::Name => parse_simple(first),
        TT::LBrace => parse_object(&mut tokens),
        TT::LSqBrac => parse_array(&mut tokens),
        
        _ => panic!("Invalid starting token: {}", first)
    };

    if !tokens.peek().is_none() {
        let remaining: Vec<&Token> = tokens.collect();
        panic!("Tokens iterator was not entirely consumed!\nLeftover tokens: {:?}", remaining)
    }

    out
}