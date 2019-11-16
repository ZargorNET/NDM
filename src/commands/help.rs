use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::util::enums::category::Category;

pub static HELP_COMMAND: Command = Command {
    key: "help",
    description: "",
    help_page: "",
    category: Category::Misc,
    show_on_help: false,
    func: help_command,
};

fn help_command(args: CommandArguments) -> CommandResult {
    let handler = args.handler.read();
    let mut cmds = handler.get_all_commands().clone();
    cmds.sort_by(|a, b| a.category.to_string().cmp(&b.category.to_string()));
    let _ = args.m.channel_id.send_message(&args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("Help");
            let mut s = String::new();
            for cmd in cmds.into_iter() {
                if cmd.show_on_help == false { continue; }
                if cmd.help_page == "" {
                    s.push_str(&format!("``{}{}`` => {}\n", args.settings.read().default_prefix, cmd.key, cmd.description));
                } else {
                    s.push_str(&format!("``{}{} {}`` => {}\n", args.settings.read().default_prefix, cmd.key, cmd.help_page, cmd.description));
                }
            }
            eb.description(s);
            eb.color(Colour::from_rgb(39, 174, 96));
            super::util::add_footer(&mut eb, &args);
            super::util::add_timestamp(&mut eb);
            eb
        })
    });
    Ok(true)
}