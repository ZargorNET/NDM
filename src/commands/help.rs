use crate::command_framework::{Command, CommandArguments, CommandResult};

pub static HELP_COMMAND: Command = Command {
    key: "help",
    description: "hey",
    help_page: "",
    category: "",
    func: help_command,
};

fn help_command(args: CommandArguments) -> CommandResult {
    let handler = args.handler.read();
    let cmds = handler.get_all_commands();
    let mut s = String::new();
    for command in cmds.iter() {
        s.push_str(&format!("{} - {} - ``{}``\n", command.key, command.description, command.help_page));
    }
    let _ = args.m.reply(args.ctx, s);
    Ok(true)
}