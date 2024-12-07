use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day07Error {
    #[error("could not parse '{input}' into equation: {error_msg}")]
    EquationParseError { input: String, error_msg: String },
}
