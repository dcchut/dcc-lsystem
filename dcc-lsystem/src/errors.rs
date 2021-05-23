use crate::ArenaId;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LSystemError {
    #[error("attempted to use unknown token `{0}`")]
    UnknownToken(String),
    #[error("attempted to construct invalid token `{0}`")]
    InvalidToken(String),
    #[error("invalid arena ID `{0:?}`")]
    InvalidArenaId(ArenaId),
    #[error("invalid rule `{0}`")]
    InvalidRule(String),
    #[error("axiom has not been defined")]
    MissingAxiom,
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("there was an unexpected error in another thread")]
    ThreadError,
    #[error("there was an unexpected error: {source}")]
    Other {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[cfg(feature = "image_renderer")]
impl From<gifski::Error> for LSystemError {
    fn from(e: gifski::Error) -> Self {
        LSystemError::Other {
            source: Box::new(e),
        }
    }
}
