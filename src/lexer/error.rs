use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}
