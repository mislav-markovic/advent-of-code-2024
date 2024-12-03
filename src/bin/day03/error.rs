use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day03Error {
    #[error("could not parse '{input}' into mul instruction: {error_msg}")]
    MulInstrParseError { input: String, error_msg: String },
}
