use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("Data not found error")]
    NotFound,
    #[error("Invalid input error")]
    InvalidInput,
}
