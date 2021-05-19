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
    InvalidArenaID(ArenaId),
    #[error("invalid rule `{0}`")]
    InvalidRule(String),
    #[error("axiom has not been defined")]
    MissingAxiom,
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "image_renderer")]
    #[error("{0}")]
    RenderError(&'static str),
    #[cfg(feature = "image_renderer")]
    #[error("gifski error")]
    GifskiError(#[from] gifski::Error),
}
