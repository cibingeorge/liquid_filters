use thiserror::Error;
use std::result;

/// Convenience type alias for parse errors
#[allow(dead_code)]
pub type Result<T, E = Error> = result::Result<T, E>;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("template compile error {0}")]
    CompileError(String),
    #[error("template render error {0}")]
    RenderingError(String),
    #[error("internal error: {0}")]
    InternalError(String),
    #[error("Error decoding json string: {0}")]
    JsonDecodeError(String),

}
