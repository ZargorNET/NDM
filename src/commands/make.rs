use crate::command_framework::prelude::*;

pub static MAKE_COMMAND: Command = Command {
    key: "make",
    description: "Shows you all available image commands",
    help_page: "",
    category: Category::Misc,
    func: make_command,
};

fn make_command(args: CommandArguments) -> CommandResult {
    let handler = args.handler.read();
    let mut cmds = handler.get_all_commands().clone();
    cmds.sort_by(|a, b| a.category.to_string().cmp(&b.category.to_string()));
    cmds.retain(|c| c.category == Category::GeneratedImage);

    super::help::print_cmds(&args, cmds, "Image Generation");

    Ok(MarkAsSucceeded)
}