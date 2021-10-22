use serde::{Deserialize, Serialize, Serializer};


use std::cell::Ref;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use thiserror::Error;

use schemars::JsonSchema;

mod state;

mod traits;

mod execute;
use execute::Execute;

mod autoconfig;

mod task;
use task::Task;

#[derive(Serialize, JsonSchema, Deserialize, Debug, PartialEq)]
struct ExecStatus{

}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("command is not valid format")]
    InvalidCommand
}

pub enum AnyTask{
    Execute(Execute),
    Interpret(Task),
}



trait Std {
    /// Get a reference to the output's stdout.
    fn stdout(&self) -> &str;
    /// Get a reference to the output's stderr.
    fn stderr(&self) -> &str;
}



use std::process::{Command, Output};
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("command is not valid format")]
    InvalidCommand,

    #[error("failed to start with io error {0}")]
    FailedToRun(#[from] std::io::Error),

    #[error("run failed")]
    RunFailed(Output),

    #[error("ran sucessfully but coulden't parse the utf-8 output")]
    ResultDecode(#[from] FromUtf8Error),
}

impl From<Output> for ExecuteError {
    fn from(o: Output) -> Self {
        assert!(o.status.success(), "this is sucess");
        Self::RunFailed(o)
    }
}

impl<T> From<Option<T>> for ExecuteError {
    fn from(o: Option<T>) -> Self {
        assert!(o.is_none(), "Some isn't an error");
        Self::InvalidCommand
    }
}


#[cfg(test)]
mod tests {
    use crate::execute::Execute;

    use super::*;
    #[test]
    fn simple_exec() {
        let cmd = Execute::new("true").add_arg("hello world");
    }
}