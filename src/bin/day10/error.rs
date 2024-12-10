use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day10Error {
    #[error("could not parse '{input}' into topographic map: {error_msg}")]
    MapParseError { input: String, error_msg: String },
}
