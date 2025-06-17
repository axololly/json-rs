use std::{fmt::{Debug, Display}, slice::Iter};

use crate::token::Token;

pub struct CharIter<'a> {
    remaining: &'a str,
    next: Option<char>
}

impl<'a> CharIter<'a> {
    pub fn new(s: &'a str) -> CharIter<'a> {
        let mut chars = s.chars();
        let next = chars.next();

        CharIter {
            remaining: chars.as_str(),
            next: next
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.next
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.remaining.chars();

        let next = self.next;
        self.next = chars.next();
        self.remaining = chars.as_str();

        next
    }
}

impl<'a> Debug for CharIter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ch) = self.next {
            let s = String::from(ch) + self.remaining;

            write!(f, "CharIter({:?})", s).unwrap();
        }
        else {
            write!(f, "CharIter(\"\")").unwrap();
        }

        Ok(())
    }
}

pub struct TokenIter<'a> {
    remaining: Iter<'a, Token>,
    next: Option<&'a Token>
}

impl<'a> TokenIter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> TokenIter<'a> {
        let mut iter = tokens.iter();
        let next = iter.next();

        TokenIter {
            remaining: iter,
            next: next
        }
    }

    pub fn peek(&self) -> Option<&'a Token> {
        self.next
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        
        self.next = self.remaining.next();

        next
    }
}

pub struct Pos {
    pub line: u32,
    pub column: u32
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "[Line: {}, Column: {}]",
            self.line,
            self.column
        ).as_str()).unwrap();

        Ok(())
    }
}