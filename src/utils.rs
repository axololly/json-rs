use std::fmt::Debug;

pub struct CharIter<'a> {
    remaining: &'a str,
    next: Option<char>
}

#[allow(dead_code)]
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

    pub fn as_str(&self) -> &str {
        self.remaining
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