
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Int,
    String,
    Float,
    Name,    // true, false, null
    LSqBrac,
    RSqBrac,
    LBrace,
    RBrace,
    Comma,
    Colon
}

pub struct Token {
    line_no: u32,
    col_no: u32,
    pub tok_type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(tok_type: TokenType, value: String, line: u32, column: u32) -> Token {
        Token {
            tok_type: tok_type,
            value: value,
            line_no: line,
            col_no: column
        }
    }

    pub fn pos(&self) -> String {
        format!("[Line: {}, Column: {}]", self.line_no, self.col_no)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.len() == 1 {
            write!(f, "Token({:?})", self.value).unwrap();
            return Ok(());
        }
        
        write!(f,
            "Token(type = '{:?}', value = {:?})",
            self.tok_type,
            self.value
        ).unwrap();

        Ok(())
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
"Token(
    type = '{:?}',
    value = {:?},
    line = {},
    column = {}
)",
            self.tok_type,
            self.value,
            self.line_no,
            self.col_no
        ).unwrap();
        
        Ok(())
    }
}
