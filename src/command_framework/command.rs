use std::fmt;
use std::fmt::Formatter;

use super::prelude::*;

#[derive(Clone)]
pub struct Command {
    pub key: &'static str,
    pub description: &'static str,
    pub help_page: &'static str,
    pub category: Category,
    pub func: fn(args: CommandArguments) -> CommandResult,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r##"Command {{ key = "{}", desc = "{}", help = "{}", cat = "{}" }}"##, self.key, self.description, self.help_page, self.category.to_string())
    }
}