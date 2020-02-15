use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::command_framework::CommandAction::{MarkAsFailed, MarkAsSucceeded, PrintUsage};
use crate::commands::category::Category;

pub static SAY_COMMAND: Command = Command {
    key: "say",
    description: "I say what you want me to say",
    help_page: "<Message>",
    category: Category::Fun,
    func: say_command,
};

fn say_command(args: CommandArguments) -> CommandResult {
    let v: Vec<&str> = args.m.content.splitn(2, ' ').collect();
    return match v.get(1) {
        Some(t) => {
            let _ = args.m.channel_id.send_message(args.ctx, |f| {
                f.content(t)
            });
            Ok(MarkAsSucceeded)
        }
        None => {
            Ok(PrintUsage)
        }
    };
}