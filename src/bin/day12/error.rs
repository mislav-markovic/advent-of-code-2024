use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day12Error {
    #[error("could not parse '{input}' into plant type")]
    PlantTypeParseError { input: char },
}
