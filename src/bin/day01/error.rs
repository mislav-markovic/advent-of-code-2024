use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day01Error {
    #[error("could not parse '{input}' into location id: {error_msg}")]
    LocationIdParseError { input: String, error_msg: String },
}
