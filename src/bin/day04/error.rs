use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day04Error {
    #[error("could not parse '{input}' into word puzzle: {error_msg}")]
    PuzzleParseError { input: String, error_msg: String },
}
