use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Day09Error {
    #[error("could not parse '{input}' into disk map: {error_msg}")]
    DiskMapParseError { input: String, error_msg: String },
    #[error("could not allocate {how_much} blocks of {value} into region [{from}:{to}]")]
    AllocationError {
        how_much: usize,
        value: usize,
        from: usize,
        to: usize,
    },
}
