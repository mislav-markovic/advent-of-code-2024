use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day05Error {
    #[error("could not parse '{input}' into page order rule: {error_msg}")]
    PageOrderingRuleError { input: String, error_msg: String },

    #[error("could not parse '{input}' into page order list: {error_msg}")]
    PageListError { input: String, error_msg: String },
}
