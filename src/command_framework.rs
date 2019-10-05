use std::{error, fmt};
use std::fmt::Formatter;
use std::sync::Arc;

use serenity::model::prelude::Message;
use serenity::prelude::{Context, RwLock};

use crate::safe::Safe;
use crate::scheduler::Scheduler;

pub struct CommandManager {
    commands: Vec<&'static Command>
}


/// This struct will be passed for every command
///
/// ctx: The current context
///
/// m: The message without the prefix
///
/// handler: The handler wrapped in a already cloned Arc
#[derive(Clone)]
pub struct CommandArguments<'a> {
    pub ctx: &'a Context,
    pub m: &'a Message,
    pub handler: Arc<RwLock<CommandManager>>,
    pub scheduler: Arc<RwLock<Scheduler>>,
    pub safe: Arc<RwLock<Safe>>,
}

#[derive(Clone)]
pub struct Command {
    pub   key: &'static str,
    pub   description: &'static str,
    pub   help_page: &'static str,
    pub   category: &'static str,
    pub   func: fn(args: CommandArguments) -> CommandResult,
}


#[derive(Debug, Clone)]
pub struct CommandError {
    pub  cmd: &'static Command,
    pub  err: String,
}

pub type CommandResult = Result<bool, CommandError>;

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r##"Command {{ key = "{}", desc = "{}", help = "{}", cat = "{}" }}"##, self.key, self.description, self.help_page, self.category)
    }
}

impl CommandManager {
    pub fn new() -> CommandManager {
        CommandManager {
            commands: vec![],
        }
    }

    pub fn get_command(&self, k: &str) -> Option<&Command> {
        if let Some(command) = self.commands.iter().find(|c| c.key.to_lowercase() == k.to_lowercase()) {
            return Some(command);
        } else {
            return None;
        }
    }

    pub fn register_command(&mut self, cmd: &'static Command) {
        self.commands.push(cmd);
    }

    pub fn get_all_commands(&self) -> &Vec<&'static Command> {
        &self.commands
    }
}

impl<'a> CommandArguments<'a> {
    pub fn new(ctx: &'a Context, m: &'a Message, handler: Arc<RwLock<CommandManager>>, scheduler: Arc<RwLock<Scheduler>>, safe: Arc<RwLock<Safe>>) -> CommandArguments<'a> {
        CommandArguments {
            ctx,
            m,
            handler,
            scheduler,
            safe,
        }
    }
}

impl CommandError {
    pub fn new_str(cmd: &'static Command, err: &str) -> CommandError {
        CommandError {
            cmd,
            err: err.to_owned(),
        }
    }
    pub fn new(cmd: &'static Command, err: String) -> CommandError {
        CommandError {
            cmd,
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
        use crate::command_framework::CommandError;
        match $func {
            Ok(o) => o,
            Err(err) => return Err(CommandError::new($cmd, format!("{}: {}", $extra, err.description().to_owned())))
        }
    }
    };
}