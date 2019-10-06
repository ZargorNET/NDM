use crate::command_framework::{Command, CommandError};

mod util;
pub mod help;
pub mod animal;
pub mod meme;
pub mod about;
pub mod urban;

pub const CATEGORY_IMAGES: &'static str = "Images";

pub static ERROR_CMD_TEST: Command = Command {
    key: "test",
    description: "",
    help_page: "",
    category: "",
    func: |a| {
        Ok(true)
    },
};