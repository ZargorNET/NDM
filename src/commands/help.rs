use serenity::model::channel::Reaction;
use serenity::utils::Colour;

use crate::command_framework::CommandAction::MarkAsSucceeded;
use crate::command_framework::prelude::*;
use crate::util::eventwaiter::{EventAction, ReactionEvent, ResponseAccess};

pub static HELP_COMMAND: Command = Command {
    key: "help",
    description: "This help page",
    help_page: "",
    category: Category::Misc,
    func: help_command,
};

fn help_command(args: CommandArguments) -> CommandResult {
    let handler = args.handler.read();
    let mut cmds = handler.get_all_commands().clone();
    cmds.sort_by(|a, b| a.category.to_string().cmp(&b.category.to_string()));
    cmds.retain(|c| c.category.show_on_help());
    print_cmds(&args, cmds, "Help");
    Ok(MarkAsSucceeded)
}

//noinspection DuplicatedCode
pub(super) fn print_cmds(args: &CommandArguments, cmds: Vec<Command>, title: &str) {
    let message = args.m.channel_id.send_message(&args.ctx, |mb| {
        mb.embed(|mut eb| {
            let mut s = String::new();
            for cmd in cmds.iter() {
                if Category::Animals.get_category_emoji() != cmd.category.get_category_emoji() {
                    continue;
                }
                eb.title("Help ".to_string() + &cmd.category.to_string());
                if cmd.help_page == "" {
                    s.push_str(&format!("``{}{}`` => {}\n", args.settings.default_prefix, cmd.key, cmd.description));
                } else {
                    s.push_str(&format!("``{}{} {}`` => {}\n", args.settings.default_prefix, cmd.key, cmd.help_page, cmd.description));
                }
            }
            eb.description(s);
            eb.color(Colour::from_rgb(39, 174, 96));
            super::util::add_footer(&mut eb, &args);
            super::util::add_timestamp(&mut eb);
            eb
        })
    });
    let mut emotes: Vec<String> = vec![];
    let msg = message.as_ref().unwrap();

    for cmd in cmds {
        let s = cmd.category.get_category_emoji().to_string();
        if !emotes.contains(&s) {
            let _ = msg.react(args.ctx, ReactionType::from(s.clone()));
            emotes.push(s);
        }
    }
    {
        args.event_waiter.register_event(ReactionEvent {
            access: ResponseAccess::Everyone,
            timeout: 0,
            eventwaiter: Arc::clone(&args.event_waiter),
            author: args.m.author.clone(),
            message: message.unwrap(),
            handler: Arc::clone(&args.handler),
            settings: Arc::clone(&args.settings),
            callback: update_help,
        })
    }
}

//noinspection DuplicatedCode
pub fn update_help(ctx: &Context, event: &mut ReactionEvent, reaction: &Reaction) -> EventAction {
    let _ = event.message.clone().edit(ctx, |m| {
        m.embed(|mut embed| {
            let mut s = String::new();
            for cmd in event.handler.read().get_all_commands().iter() {
                if cmd.category.get_category_emoji() != reaction.emoji.as_data().as_str() {
                    continue;
                }
                embed.title("Help ".to_string() + &cmd.category.to_string());
                if cmd.help_page == "" {
                    s.push_str(&format!("``{}{}`` => {}\n", event.settings.default_prefix, cmd.key, cmd.description));
                } else {
                    s.push_str(&format!("``{}{} {}`` => {}\n", event.settings.default_prefix, cmd.key, cmd.help_page, cmd.description));
                }
            }
            embed.description(s);
            embed.color(Colour::from_rgb(39, 174, 96));
            embed.footer(|fb| {
                fb.text(&event.author.name);
                if let Some(avatar) = &event.author.avatar_url() {
                    fb.icon_url(avatar);
                }
                fb
            });
            super::util::add_timestamp(&mut embed);
            embed
        })
    });
    EventAction::Keep
}