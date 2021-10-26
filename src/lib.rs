/// A portable unit of code designed to be executed remotely

use thiserror::Error;

mod state;
mod config;
mod task;
mod exec;
mod sniff;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("command is not valid format")]
    InvalidCommand
}

// #[derive(Error, Debug)]
// pub enum ExecuteError {
//     #[error("command is not valid format")]
//     InvalidCommand,

//     #[error("failed to start with io error {0}")]
//     FailedToRun(#[from] std::io::Error),

//     #[error("run failed")]
//     RunFailed(Output),

//     #[error("ran sucessfully but coulden't parse the utf-8 output")]
//     ResultDecode(#[from] FromUtf8Error),
// }

// impl From<Output> for ExecuteError {
//     fn from(o: Output) -> Self {
//         assert!(o.status.success(), "this is sucess");
//         Self::RunFailed(o)
//     }
// }

// impl<T> From<Option<T>> for ExecuteError {
//     fn from(o: Option<T>) -> Self {
//         assert!(o.is_none(), "Some isn't an error");
//         Self::InvalidCommand
//     }
// }
