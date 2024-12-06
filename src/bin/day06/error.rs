use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day06Error {
    #[error("could not parse '{input}' into map: {error_msg}")]
    MapParseError { input: String, error_msg: String },

    #[error("guard double defined: first at ({},{}), second at ({},{})", first_post.0, first_post.1, second_pos.0, second_pos.1)]
    GuardDoubleDefinedError {
        first_post: (usize, usize),
        second_pos: (usize, usize),
    },

    #[error("map does not have guards initial position marked")]
    GuardMissingInitialPosition,

    #[error("guard at ({},{}) can not step to ({},{})", current_position.0, current_position.1, next_position.0, next_position.1)]
    GuardStepError {
        current_position: (usize, usize),
        next_position: (i32, i32),
    },

    #[error("failed to simulte guard movements: {why}")]
    MovementSimulationError { why: String },
}
