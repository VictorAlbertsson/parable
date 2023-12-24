#![feature(iter_advance_by)]

use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    ListL,          // Special
    ListR,          // Special
    ConsL,          // Special
    ConsR,          // Special
    ConsC,          // Space-delimited or Special?
    Symbol(String), // Space-delimited
    Number(String), // Space-delimited
    String(String), // Space-delimited
}

#[derive(Debug)]
struct TokenIterator<'a> {
    buffer: Peekable<Chars<'a>>,
}

impl<'a> From<&'a str> for TokenIterator<'a> {
    fn from(source: &'a str) -> Self {
        return Self {
            buffer: source.chars().peekable(),
        };
    }
}

impl TokenIterator<'_> {
    fn take_symbol(&mut self) -> Option<Token> {
        let mut symbol_body = String::new();
        while let Some(head) = self.buffer.next_if(valid_symbol) {
            symbol_body.push(head);
        }

        return if symbol_body.is_empty() {
            None
        } else {
            Some(Token::Symbol(symbol_body))
        };

        fn valid_symbol(c: &char) -> bool {
            return (*c == '-') || c.is_alphanumeric();
        }
    }

    fn take_number(&mut self) -> Option<Token> {
        let mut number_body = String::new();
        while let Some(head) = self.buffer.next_if(valid_number) {
            // Decimals are split by `.`, how do I handle that?
            number_body.push(head);
        }

        return if number_body.is_empty() {
            None
        } else {
            Some(Token::Number(number_body))
        };

        fn valid_number(c: &char) -> bool {
            return c.is_ascii_digit();
        }
    }
    fn take_string(&mut self) -> Option<Token> {
        let mut string_body = String::new();
        if let Some('"') = self.buffer.next() {
            while let Some(head) = self.buffer.next() {
                match head {
                    '"' => break,
                    '\\' => {
                        if let Some(next_head) = self.buffer.next() {
                            string_body.push(next_head);
                        } else {
                            todo!("[TODO] String tried to escape <EOI>");
                        }
                    }
                    _ => {
                        string_body.push(head);
                    }
                }
            }
        } else {
            return None;
        }

        return Some(Token::String(string_body));
    }
    fn take_literal(&mut self) -> Option<Token> {
        if let Some(head) = self.buffer.next() {
            match head {
                '(' => Some(Token::ListL),
                ')' => Some(Token::ListR),
                '[' => Some(Token::ConsL),
                ']' => Some(Token::ConsR),
                '.' => Some(Token::ConsC),
                _ => todo!("[TODO] `take_literal()` Invalid syntax"),
            }
        } else {
            todo!("[TODO] `take_literal()` End of input")
        }
    }
}

impl Iterator for TokenIterator<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // TODO: Trim whitespace
        return if let Some(head) = self.buffer.peek() {
            match head {
                '"' => self.take_string(),
                c if c.is_ascii_digit() => self.take_number(),
                // TODO: Support symbols like `+` and `-`
                c if c.is_alphabetic() => self.take_symbol(),
                _ => self.take_literal(),
            }
        } else {
            None
        };

        // fn valid_digit(c: char) -> bool {
        //     todo!()
        // }
        // fn valid_alphabetic(c: char) -> bool {
        //     todo!()
        // }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{Token, TokenIterator};

    #[test]
    fn take_symbol_1() {
        let input: &str = "foo";
        let mut iter = TokenIterator::from(input);
        assert_eq!(iter.take_symbol(), Some(Token::Symbol("foo".to_string())));
    }
    #[test]
    fn take_symbol_2() {
        let input: &str = "foo bar";
        let mut iter = TokenIterator::from(input);
        assert_eq!(iter.take_symbol(), Some(Token::Symbol("foo".to_string())));
    }
    #[test]
    fn take_symbol_3() {
        let input: &str = "(foo)";
        let mut iter = TokenIterator::from(input);
        assert_eq!(iter.take_symbol(), None);
    }
    // #[test]
    // fn take_string() {}
}
