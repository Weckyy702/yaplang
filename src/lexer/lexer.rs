use std::mem;

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
            match c {
                '(' | ')' => break,
                c if c.is_whitespace() => break,
                _ => (),
            }
            s.push(c);
            self.consume();
        }
        s.into()
    }

    fn next_token(&mut self) -> Token {
        macro_rules! ok {
            ($ret:expr) => {{
                self.consume();
                return $ret;
            }};
        }

        match self.peek().expect("Can peek") {
            '(' => ok!(Token::LParen),
            ')' => ok!(Token::RParen),
            ':' => {
                self.consume();
                match self.peek() {
                    Some('=') => ok!(Token::Assign),
                    _ => return Token::Colon,
                }
            }
            '-' => {
                self.consume();
                match self.peek() {
                    Some('>') => ok!(Token::ThinArrow),
                    _ => return Token::Identifier("-".into()),
                }
            }
            '=' => {
                self.consume();
                match self.peek() {
                    Some('>') => ok!(Token::ThickArrow),
                    _ => return Token::Identifier("=".into()),
                }
            }
            _ => Token::Identifier(self.read_identifier()),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if self.is_done() {
            return None;
        }

        Some(self.next_token())
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::lexer::token::Token;

    fn lex(s: &str) -> Vec<Token> {
        Lexer::new(s.chars()).collect()
    }

    #[test]
    fn empty_string() {
        assert_eq!(lex(""), [])
    }

    #[test]
    fn comment() {
        assert_eq!(lex("#this is a comment"), [])
    }

    #[test]
    fn multiline_comment() {
        assert_eq!(
            lex(r#"
            #this is comment line 1
            #this is comment line 2
        "#),
            []
        )
    }

    #[test]
    fn parentheses() {
        use Token::*;
        assert_eq!(
            lex("(()(()))"),
            [LParen, LParen, RParen, LParen, LParen, RParen, RParen, RParen,]
        )
    }

    #[test]
    fn complex() {
        use Token::*;

        macro_rules! ident {
            ($e:expr) => {
                Identifier(($e).into())
            };
        }

        assert_eq!(
            lex(r#"
# this is a comment
:= a 1
:= b 2
:= c (+ a b)
∈ c ℕ
        "#),
            [
                Assign,
                ident!("a"),
                ident!("1"),
                Assign,
                ident!("b"),
                ident!("2"),
                Assign,
                ident!("c"),
                LParen,
                ident!("+"),
                ident!("a"),
                ident!("b"),
                RParen,
                ident!("∈"),
                ident!("c"),
                ident!("ℕ")
            ]
        )
    }
}
