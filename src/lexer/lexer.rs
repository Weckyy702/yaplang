use std::mem;

use crate::lexer::error::Error;
use crate::lexer::token::Token;
use crate::utils::InternedString;

#[derive(Debug)]
pub struct Lexer<I> {
    it: I,
    next: Option<char>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new<It>(it: It) -> Self
    where
        It: IntoIterator<IntoIter = I>,
    {
        let mut it = it.into_iter();
        let next = it.next();

        Self { it, next }
    }

    fn is_done(&self) -> bool {
        self.next.is_none()
    }

    fn peek(&self) -> Option<char> {
        self.next
    }

    fn peek_or_eof(&self) -> Result<char, Error> {
        self.peek().ok_or(Error::UnexpectedEOF)
    }

    fn consume(&mut self) -> char {
        mem::replace(&mut self.next, self.it.next()).expect("Lexer: Can consume character")
    }

    fn skip_whitespace(&mut self) {
        let mut in_comment = false;
        while let Some(c) = self.peek() {
            match c {
                '#' => in_comment = true,
                '\n' => in_comment = false,
                _ if in_comment => (),
                c if c.is_whitespace() => (),
                _ => break,
            }
            self.consume();
        }
    }

    fn read_identifier(&mut self) -> InternedString {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                break;
            }
            s.push(c);
            self.consume();
        }
        s.into()
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        macro_rules! ok {
            ($ret:expr) => {{
                self.consume();
                return Ok($ret);
            }};
        }

        match self.peek_or_eof()? {
            '(' => ok!(Token::LParen),
            ')' => ok!(Token::RParen),
            ':' => {
                self.consume();
                match self.peek() {
                    Some('=') => ok!(Token::Assign),
                    _ => return Ok(Token::Colon),
                }
            }
            '-' => {
                self.consume();
                match self.peek() {
                    Some('>') => ok!(Token::ThinArrow),
                    _ => return Ok(Token::Identifier("-".into())),
                }
            }
            '=' => {
                self.consume();
                match self.peek() {
                    Some('>') => ok!(Token::ThickArrow),
                    _ => return Ok(Token::Identifier("=".into())),
                }
            }
            _ => Ok(Token::Identifier(self.read_identifier())),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if self.is_done() {
            return None;
        }

        Some(self.next_token())
    }
}
