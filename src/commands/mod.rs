use crate::command_framework::Command;

mod util;
pub mod help;
pub mod animal;
pub mod meme;
pub mod about;
pub mod urban;
pub mod chuck;
pub mod urbanmug;

pub const CATEGORY_IMAGES: &'static str = "Images";

pub static ERROR_CMD_TEST: Command = Command {
    key: "test",
    description: "",
    help_page: "",
    category: "",
    func: |_a| {
        Ok(true)
    },
};