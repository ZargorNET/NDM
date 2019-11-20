use std::{error, fmt};
use std::fmt::Formatter;

use super::prelude::*;

pub type CommandResult = Result<CommandAction, CommandError>;

pub enum CommandAction {
    MarkAsSucceeded,
    MarkAsFailed,
    PrintUsage,
}

#[derive(Debug, Clone)]
pub struct CommandError {
    pub cmd: Command,
    pub err: String,
}

#[allow(dead_code)]
impl CommandError {
    pub fn new_str(cmd: &Command, err: &str) -> CommandError {
        CommandError {
            cmd: cmd.clone(),
            err: err.to_owned(),
        }
    }
    pub fn new(cmd: &Command, err: String) -> CommandError {
        CommandError {
            cmd: cmd.clone(),
            err,
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"error in command "{}": {}"#, self.cmd.key, self.err)
    }
}


impl error::Error for CommandError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

macro_rules! unwrap_cmd_err {
    ($cmd:expr, $func:expr, $extra:expr) => {
    {
        #[allow(unused_imports)]
        use std::error::Error;
        use crate::command_framework::CommandError;
        match $func {
            Ok(o) => o,
            Err(err) => return Err(CommandError::new($cmd, format!("{}: {}", $extra, err.to_string())))
        }
    }
    };
}