use crate::task::*;

use crate::state::*;
use anyhow::Context;
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
use std::path::Path;
use std::process::Stdio;
use tokio::process::*;

pub fn fun(payload: Payload) {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();

    let mut command = Command::new("echo");

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env_clear()
        .envs(&filtered_env);
    let mut child = command.spawn().unwrap();
    let stdout = child.stdout.take().unwrap();
}
async fn start(payload: Payload) -> anyhow::Result<Task>{
    let tmp = tempfile::TempDir::new().expect("can't get tmp directory");
    let entrypoint = Path::join(tmp.as_ref(), "entrypoint");
    let content = reqwest::blocking::get(payload.src.clone())?;

    let config = Path::join(tmp.as_ref(), "config.json");


    let mut cmd = Command::new(entrypoint);
    if *payload.context.ignore_environment(){
        cmd.env_clear();
    }

    for (key, val) in payload.context.env() {
        if let Some(value) = val {
            cmd.env(key, value);
        } else {
            cmd.env_remove(key);
        }
    }


//let pipes  = mut;

    cmd.args(payload.context.args());

    let state = Default::default();

    let stdio = Default::default();

    let child = cmd.spawn()?;
    child.stdin.fi

    let task = Task {
        payload,
        state,
        stdio,
        tmp
    };
    todo!()
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
