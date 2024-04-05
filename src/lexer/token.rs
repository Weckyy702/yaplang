use crate::utils::InternedString;

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Colon,
    ThinArrow,
    ThickArrow,
    Assign,
    Identifier(InternedString),
}
