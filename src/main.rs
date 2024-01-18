#![feature(exclusive_range_pattern)]

use lasso::{Spur, ThreadedRodeo};
use std::{sync::Arc, iter::Peekable, str::Chars};

pub type Intern = Spur;
pub type InternAllocator = Arc<ThreadedRodeo>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Token {
    pub tag: TokenTag,
    pub txt: Intern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenTag {
    ListL,
    ListR,
    ConsL,
    ConsR,
    Comment,
    Symbol,
    HorizontalSpace,
    VerticalSpace,
    NumberLiteral,
    StringLiteral,
}

#[derive(Debug)]
struct TokenIterator<'a> {
    alloc: InternAllocator,
    source: Peekable<Chars<'a>>,
}

impl<'a> From<&'a str> for TokenIterator<'a> {
    fn from(string: &'a str) -> Self {
        return Self {
            alloc: Arc::new(ThreadedRodeo::default()),
            source: string.chars().peekable(),
        };
    }
}

impl Iterator for TokenIterator<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(&head) = self.source.peek() {
            match head {
                '\n' | '\r' => Some(self.get_vertical_space()),
                ' ' | '\t' => Some(self.get_horizontal_space()),
                '0'..'9'   => Some(self.get_number()),
                '"'        => Some(self.get_string()),
                '('        => Some(self.get(TokenTag::ListL, "(")),
                ')'        => Some(self.get(TokenTag::ListR, ")")),
                '['        => Some(self.get(TokenTag::ConsL, "[")),
                ']'        => Some(self.get(TokenTag::ConsR, "]")),
                _          => todo!(),
            }
        } else {
            None
        }
    }
}

impl TokenIterator<'_> {
    fn get_string(&mut self) -> Token { todo!() }
    fn get_number(&mut self) -> Token { todo!() }
    fn get_horizontal_space(&mut self) -> Token { todo!() }
    fn get_vertical_space(&mut self) -> Token { todo!() }
    fn get(&mut self, tag: TokenTag, slice: &str) -> Token {
        let txt = self.alloc.get_or_intern(slice);
        self.step();
        return Token { tag, txt };
    }
    fn step(&mut self) {
        self.source.next();
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
}
