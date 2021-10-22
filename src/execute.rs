use anyhow::Context;
use serde::{Deserialize, Serialize, Serializer};
use std::{cell::Ref, process::Command};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use thiserror::Error;
use schemars::JsonSchema;

use crate::{ExecuteError, ParseError};

pub type Path = String;//Box<std::path::Path>;

#[derive(Serialize, JsonSchema, Deserialize, Debug, PartialEq)]
pub struct Execute {
    file: Path,

    #[serde(default)]
    args: Vec<String>,

    #[serde(default)]
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
}






impl Execute {
    

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

impl Execute {
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


impl Display for Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_shell())
    }
}

impl From<Execute> for String {
    fn from(exec: Execute) -> Self {
        exec.to_shell()
    }
}


impl TryFrom<&str> for Execute {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args = shlex::split(value).context("invalid command to parse")?;
        let (program, args) = args.split_first().context("can't split")?;
        let mut program = Self::new(program);
        program.add_args(args);
        Some(program)
    }
}


impl Default for Execute {
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