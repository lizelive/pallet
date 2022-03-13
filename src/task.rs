/// The payload data format

use std::collections::HashMap;
use std::process::ChildStderr;
use tempfile::TempDir;
use tokio::process::{ChildStdin, ChildStdout};
use url::Url;

use crate::config::Config;
use crate::state::State;



// #[derive(Serialize, Deserialize)]
// #[derive(Debug, PartialEq)]
// #[serde(untagged)]
// pub enum Source {
//     Url(Url),
//     Text(String),
// }

// impl Source {
//     fn copy_to_temp(&self) -> anyhow::Result<TempDir> {
//         let dir = tempfile::tempdir()?;
//         let entrypoint = dir.path().join(ENTRYPOINT_NAME);
//         let mut file = File::create(entrypoint)?;
//         match self {
//             Source::Url(url) => {
//                 let resp = reqwest::blocking::get(url.clone())?;
//                 let length = resp.copy_to(&mut file)?;
//             }
//             Source::Text(content) => {
//                 let utf8 = content.as_bytes();
//                 file.write_all(utf8)?;
//             }
//         };
//         Ok(dir)
//     }
// }

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone, PartialEq)]
enum Expansion {
    /// pass as, escaping as needed to make that happen
    Escape,

    /// replace ${var} with env varibles of runner
    /// ignores the set varibles.
    EnvironmentSubtitute,

    /// execute bash -s echo arguments with echo argument
    Evaluate,
}

impl Default for Expansion {
    fn default() -> Self {
        Expansion::Escape
    }
}

#[derive(Builder)]
#[derive(Getters)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[getset(get = "pub")]
pub struct Context {
    #[serde(default)]
    args: Vec<String>,

    #[serde(default)]
    /// should i ignore all current enviroment varible
    ignore_environment: bool,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    /// as set of enviorment varibles to set.
    env: HashMap<String, Option<String>>,

    /// should we try using shell to expand arguments / env varibles
    expand: Expansion,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /// run using this directory
    cwd: Option<String>,

    stdin: Option<IoMode>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum IoMode {
    Pipe,
    Ignore,
}

#[derive(Serialize, Deserialize)]
#[derive(Builder)]
#[derive(Getters)]
#[derive(Debug)]
pub struct Payload {
    /// which interperter to use
    //pub interpreter: Option<Interpreter>,

    /// the code to run
    /// tries to parse as uri
    /// respects shebang
    pub src: Url,

    ///
    pub context: Context,

    pub config: Config,
}

#[derive(MutGetters, Setters)]
#[derive(Default)]
pub struct StdioPipes {
    stdin: Option<pipe::PipeWriter>,
    stdout: Option<pipe::PipeReader>,
    stderr: Option<pipe::PipeWriter>,
}

impl std::fmt::Debug for StdioPipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdioPipes").field("stdin", &self.stdin.is_some()).field("stdout", &self.stdout.is_some()).field("stderr", &self.stderr.is_some()).finish()
    }
}

#[derive(Getters)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StdioCaptured {
    stdout: Option<Box<[u8]>>,
    stderr: Option<Box<[u8]>>,
}

#[derive(Debug)]
#[derive(Getters)]
pub struct Task {
    state: State,
    payload: Payload,
    stdio: StdioPipes,
    tmp: TempDir,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Getters)]
pub struct Output {
    state: State,
    payload: Payload,
    stdio: StdioCaptured,
}

pub trait SecretProvider{

}

