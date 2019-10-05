use crate::command_framework::{Command, CommandError};

mod util;
pub mod help;
pub mod animal;
pub mod meme;

pub const CATEGORY_IMAGES: &'static str = "Images";

pub static ERROR_CMD_TEST: Command = Command {
    key: "test",
    description: "",
    help_page: "",
    category: "",
    func: |_| {
        Err(CommandError::new_str(&ERROR_CMD_TEST, "test error"))
    },
};