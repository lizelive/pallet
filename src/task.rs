use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tempfile::{TempDir, TempPath};
use std::cell::Ref;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string::ParseError;
use thiserror::Error;
use url::Url;

#[derive(Serialize, schemars::JsonSchema, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Source {
    Url(Url),
    Text(String),
}
const ENTRYPOINT_NAME: &str = "entrypoint";

impl Source {
    fn copy_to_temp(&self) -> anyhow::Result<TempDir>{
        let dir = tempfile::tempdir()?;
        let entrypoint = dir.path().join(ENTRYPOINT_NAME);
        let mut file = File::create(entrypoint)?;
        match self {
            Source::Url(url) => {
                let resp = reqwest::blocking::get(url.clone())?;
                let length = resp.copy_to(&mut file)?;
            },
            Source::Text(content) => {
                let utf8 = content.as_bytes();
                file.write_all(utf8)?;
            },
        };
        Ok(dir)
    }
}

#[derive(Serialize, JsonSchema, Deserialize, Debug, PartialEq, Default)]
struct Context {
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
    use_shell: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    /// run using this directory
    cwd: Option<String>,

    config: AutoConfig,
}

pub struct Task {
    /// the interperter to use
    pub interpreter: Option<Interpreter>,

    pub src: Source,

    pub context: Context,
}

#[derive(Serialize, JsonSchema, Deserialize, Debug, PartialEq)]
pub enum Interpreter {
    ///Execute using the Bourne shell, or a compatible shell, assumed to be in the /bin/sh directory
    BourneShell,

    /// Execute the task as Bash shell
    Bash,

    /// Execute using PowerShell
    PowerShell,

    /// execute using env program search path to find it
    Env { command: String },

    /// use a custom shebang line. must start with #!
    Shebang { shebang: String },

    ///Do nothing, but return a non-zero exit status, indicating failure. Used to prevent stand-alone execution of a script file intended for execution in a specific context, such as by the . command from sh/bash, source from csh/tcsh, or as a .profile, .cshrc, or .login file.
    False,

    /// do nothing but return true.
    True,

    /// use embeded gluon interperter
    Gluon,
}


mod transform {
    enum ArgumentTransform {
        /// pass as, escaping as needed to make that happen
        Escape,
    
        /// replace ${var} with env varibles of runner
        /// ignores the set varibles.
        EnvironmentSubtitute,
    
        /// execute bash -s echo arguments with echo argument 
        Evaluate,
    }
}

impl Task {
    

    /// Get a reference to the exec's program.
    pub fn program(&self) -> &Path {
        &self.file
    }

    /// Get a reference to the exec's args.
    pub fn args(&self) -> &[String] {
        self.args.as_slice()
    }

    /// Get a reference to the exec's env.
    pub fn env(&self) -> &HashMap<String, Option<String>> {
        &self.env
    }

    /// Get a reference to the exec's cwd.
    pub fn cwd(&self) -> Option<&String> {
        self.cwd.as_ref()
    }
}

impl Task {
    pub fn set_env(&mut self, name: impl ToString, value: impl ToString) -> &mut Self {
        self.env.insert(name.to_string(), Some(value.to_string()));
        self
    }

    pub fn add_arg(&mut self, arg: impl ToString) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn add_args<S: ToString>(&mut self, args: impl IntoIterator<Item = S>) -> &mut Self {
        let mut this = self;
        for arg in args {
            this = this.add_arg(arg);
        }
        this
    }

    pub fn with_env(
        program: String,
        args: Vec<String>,
        ignore_environment: bool,
        env: HashMap<String, Option<String>>,
        cwd: Option<String>,
    ) -> Self {
        Self {
            file: program,
            args,
            ignore_environment,
            env,
            cwd,
            use_shell: false,
        }
    }

    pub fn to_shell(&self) -> String {
        let mut program = String::new();

        if self.cwd.is_some() || self.ignore_environment || !self.env.is_empty() {
            program += "env";

            if self.ignore_environment {
                program += " --ignore-environment";
            }

            if let Some(cwd) = &self.cwd {
                program+=&format!(" --chdir={}", shlex::quote(cwd));
            }

            for (k, v) in &self.env {
                if let Some(v) = v {
                    let v = shlex::quote(v);
                    program+=&format!(" {}={}", k, v)
                } else {
                    program+= &format!(" --unset={}", k);
                }
            }
        }

        program += &self.file;

        for arg in &self.args {
            program += " ";
            program += &shlex::quote(arg);
        }

        program
    }

    pub fn parse(in_str: &str) -> Option<Self> {
        let args = shlex::split(in_str)?;
        let (program, args) = args.split_first()?;
        let mut program = Self::new(program);
        program.add_args(args);
        Some(program)
    }

    pub fn execute(&self) -> Result<String, ExecuteError> {
        
        let mut command = Command::new(&self.file);
        command.args(&self.args);
        let out = command.output()?;
        if out.status.success() {
            let str = String::from_utf8(out.stdout)?;
            Ok(str)
        } else {
            let str = String::from_utf8(out.stderr)?;
            Ok(str)
            //Err(out.into())
        }
    }

    pub fn new(program: impl ToString) -> Execute {
        Self {
            file: program.to_string(),
            ..Default::default()
        }
    }
}


impl Task for Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_shell())
    }
}

impl From<Task> for String {
    fn from(exec: Execute) -> Self {
        exec.to_shell()
    }
}


impl TryFrom<&str> for Task {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args = shlex::split(value).context("invalid command to parse")?;
        let (program, args) = args.split_first().context("can't split")?;
        let mut program = Self::new(program);
        program.add_args(args);
        Some(program)
    }
}


impl Default for Task {
    fn default() -> Self {
        Self {
            file: "true".to_string(),
            args: Vec::new(),
            ignore_environment: false,
            env: HashMap::new(),
            cwd: None,
            use_shell: false,
        }
    }
}