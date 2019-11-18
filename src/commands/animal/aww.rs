use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;
use crate::commands::category::Category;
use crate::util::safe::keys::commands::AWW_CACHE_KEY;

pub static AWW_COMMAND: Command = Command {
    key: "aww",
    description: "r/aww",
    help_page: "",
    category: Category::Animals,
    func: aww_command,
};

fn aww_command(args: CommandArguments) -> CommandResult {
    let aww;

    {
        let safe = args.safe.read();
        let awws = match safe.get::<Vec<Aww>>(AWW_CACHE_KEY) {
            Some(s) => s,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry! No awws fetched yet :c Please try again later :dog2:");
                return Ok(true);
            }
        };
        aww = awws[rand::thread_rng().gen_range(0, awws.len())].clone();
    }

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title(format!("Awwwww <3 by u/{}", aww.author));
            eb.description(aww.title);
            eb.image(aww.url);
            eb.url(format!("https://reddit.com{}", aww.permalink));
            eb.color(Colour::new(0x947867));

            //commands::util::add_timestamp(&mut eb);
            commands::util::add_footer_text(&mut eb, &args, format!("👍 {} | 💬 {}", aww.like_ammount, aww.comments_ammount));

            eb
        })
    });
    Ok(true)
}

#[derive(Clone)]
pub struct Aww {
    pub url: String,
    pub permalink: String,
    pub title: String,
    pub author: String,
    pub like_ammount: u32,
    pub comments_ammount: u32
}