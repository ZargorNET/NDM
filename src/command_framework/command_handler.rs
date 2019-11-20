use crate::command_framework::CommandAction;
use crate::Handler;

use super::prelude::*;

pub(crate) fn handle_command(handler: &Handler, ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    }
    if msg.is_private() {
        return;
    }

    info!("[Message] {}: {}", msg.author.name, msg.content_safe(&ctx.cache));
    //TODO: Server config prefix
    if !msg.content.starts_with(&handler.settings.default_prefix) {
        return;
    }

    let mut msg = msg.clone();
    msg.content = msg.content[1..].to_string();
    let msg_split: Vec<&str> = msg.content.split_whitespace().collect();

    let cmd;
    {
        let command_manager_arc = Arc::clone(&handler.ch);
        let command_manager = command_manager_arc.read();
        match command_manager.get_command(msg_split[0]) {
            Some(c) => cmd = c.clone(),
            None => return
        }
    } // DROP READ LOCK
    {
        let args = CommandArguments::new(&ctx,
                                         &msg,
                                         Arc::clone(&handler.ch),
                                         Arc::clone(&handler.safe),
                                         Arc::clone(&handler.image),
                                         Arc::clone(&handler.settings), &cmd);
        match (cmd.func)(args) {
            Ok(action) => {
                match action {
                    CommandAction::MarkAsSucceeded => {
                        let _ = msg.react(&ctx, ReactionType::from("✅"));
                    },
                    CommandAction::MarkAsFailed => {
                        let _ = msg.react(&ctx, ReactionType::from("❌"));
                    },
                    CommandAction::PrintUsage => {
                        let _ = msg.react(&ctx, ReactionType::from("❌"));
                        let _ = msg.channel_id.send_message(&ctx, |eb| {
                            eb.content(format!("Invalid syntax! Try: ``{}{} {}``", &handler.settings.default_prefix, cmd.key, cmd.help_page));
                            eb
                        });
                    },
                }
            }
            Err(err) => {
                let _ = msg.react(&ctx, ReactionType::from("❌"));
                let _ = msg.reply(&ctx, "I'm sorry, I failed... There was an error executing the command. Please try again later!");
                error!("Could not execute command: {:#?}", err);
            }
        }
    }
}