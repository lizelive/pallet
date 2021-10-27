use crate::task::*;

use crate::state::*;
use async_trait::async_trait;

pub const ENTRYPOINT_NAME: &str = "entrypoint";

use crate::errors::*;

#[async_trait]
/// a piece of work to be done or undertaken.
pub trait Executor {
    type State: Sized;
    /// start a task
    async fn start(&mut self, task: Payload) -> Result<Task, StartError>;

    /// merge with task
    async fn termiate(&mut self, task: Task) -> Result<Output, ActionError>;

    /// submit an action to a task
    /// does not wait for action to finish
    /// should be called periodicly
    async fn update(&mut self, task: Task, action: Option<Action>) -> Result<Task, Output>;
}

use std::collections::HashMap;
use std::env;
use std::process::Stdio;
use tokio::process::*;

pub fn fun() {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();

    let mut command = Command::new("printenv");

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .env_clear()
        .envs(&filtered_env);
    let mut child = command.spawn().unwrap();
    let stdout = child.stdout.take().unwrap();
}

fn start(payload: Payload) -> (Task, Run{
    
    Command::new(program)
}

/// manages running enviorments
struct Forman {

}

// impl Task {
//     pub fn new() -> Self {
//         Command::new("wc")
//                                 .stdin(Stdio::piped())
//                                 .stdout(Stdio::piped())
//                                 .spawn();
//         std::process::Stdio
//         todo!()
//     }
// }
