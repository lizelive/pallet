use serde::{Deserialize, Serialize};

use async_trait::async_trait;

use crate::state::State;

struct StartOptions{
    capture_stdout: bool,
    capture_stderr: bool,
}


type ATask = Box<dyn Task>;
type ATasking = Box<dyn Tasking>;
type ATasked = Box<dyn Tasked>;

enum StartError {
    InvalidTask,
    NotFound
}

#[async_trait]
/// a piece of work to be done or undertaken.
pub trait Task {
    /// start a task
    async fn start(self) -> Result<ATasking, StartError>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// nicely request a restart 
    Restart,
    
    /// Nicely request pause.
    Pause,

    /// Nicely request exit.
    Stop,

    // Kill dead and clean up.
    Remove,
}

enum ActionError{
    Unimplemented,
    InvalidState,
}

#[async_trait]
/// on ongoing piece fo work
pub trait Tasking {
    async fn termiate(self)-> Box<dyn Tasked>;


    async fn state(self) -> Result<State, ActionError>;
    /// submit an action to a task
    async fn action(&mut self, action: Action) -> Result<State, Box<dyn Tasked>>;
}

pub trait Tasked {
    fn stdout(&self) -> &str;
    fn stderr(&self) -> &str;
}