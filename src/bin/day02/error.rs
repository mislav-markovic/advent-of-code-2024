use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day02Error {
    #[error("could not parse '{input}' into location id: {error_msg}")]
    LevelParseError { input: String, error_msg: String },

    #[error("could not parse '{input}' into location id: {error_msg}")]
    ReportParseError { input: String, error_msg: String },
}
