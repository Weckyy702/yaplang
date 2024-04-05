use crate::utils::InternedString;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LParen,
    RParen,
    Colon,
    ThinArrow,
    ThickArrow,
    Assign,
    Identifier(InternedString),
}
