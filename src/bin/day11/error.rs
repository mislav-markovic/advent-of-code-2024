use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day11Error {
    #[error("could not parse '{input}' into initial stone line setup: {error_msg}")]
    StoneLineParseError { input: String, error_msg: String },
}
