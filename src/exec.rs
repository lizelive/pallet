
use crate::task::*;

use crate::state::*;
use async_trait::async_trait;


enum StartError {
    InvalidTask,
    NotFound
}


enum ActionError{
    Unimplemented,
    InvalidState,
}

enum PollError {
    Invalid
}

#[async_trait]
/// a piece of work to be done or undertaken.
pub trait Executor {
    /// start a task
    async fn start(&mut self, task: Payload) -> Result<Task, StartError>;

    /// merge with task
    async fn termiate(&mut self, task: Task)-> Result<Output, ActionError>;

    /// submit an action to a task
    async fn update(&mut self, task: Task, action: Option<Action>) -> Result<Task, Output>;
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