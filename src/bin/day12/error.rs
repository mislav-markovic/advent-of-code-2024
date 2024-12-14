use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day12Error {
    #[error("could not parse '{input}' into garden setup: {error_msg}")]
    GardenParseError { input: String, error_msg: String },

    #[error("could not parse '{input}' into plant type")]
    PlantTypeParseError { input: char },
}
