use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day09Error {
    #[error("could not parse '{input}' into city map of antenna frequencies: {error_msg}")]
    DiskMapParseError { input: String, error_msg: String },
}
