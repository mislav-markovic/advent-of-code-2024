use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day08Error {
    #[error("could not parse '{input}' into city map of antenna frequencies: {error_msg}")]
    CityMapParseError { input: String, error_msg: String },
}
