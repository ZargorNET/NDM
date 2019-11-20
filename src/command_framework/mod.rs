pub use command::*;
pub use command_args::*;
pub use error::*;

#[macro_use]
mod error;
mod command;
mod command_args;
pub mod command_handler;
pub mod prelude;

pub struct CommandManager {
    commands: Vec<Command>
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

    pub fn register_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn get_all_commands(&self) -> &Vec<Command> {
        &self.commands
    }
}
