//! all the errors you can go!
use thiserror::Error;

pub enum StartError {
    InvalidTask,
    NotFound,
}

pub enum ActionError {
    Unimplemented,
    InvalidState,
}

pub enum PollError {
    Invalid,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("this action is unimplemented")]
    Unimplemented,

    #[error("failed to get port")]
    NetConfigFailed,

    #[error("writing config didn't work")]
    WriteConfigFailed,

    #[error("task is in an invalid state")]
    InvalidState,

    #[error("command is not valid format")]
    InvalidCommand,

    #[error("failed to start with io error {0}")]
    CantAcessFile(#[from] std::io::Error),

    #[error("run failed")]
    RunFailed,
}