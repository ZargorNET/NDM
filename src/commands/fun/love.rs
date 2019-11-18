use rand::{Rng, SeedableRng};
use serenity::model::guild::Member;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands::category::Category;

pub static LOVE_COMMAND: Command = Command {
    key: "love",
    description: "Its a match! maybe.",
    help_page: "[<optional: lover>] <loved: @User>",
    category: Category::Fun,
    func: love_command,
};

fn love_command(args: CommandArguments) -> CommandResult {
    let cache;
    {
        cache = args.ctx.cache.read().clone();
    } // Drop Lock

    let user1: Member;
    let user2: Member;

    match args.m.mentions.len() {
        0 => {
            return Ok(false);
        }
        1 => {
            user1 = cache.member(args.m.guild_id.unwrap(), args.m.author.id).unwrap();
            user2 = cache.member(args.m.guild_id.unwrap(), args.m.mentions.get(0).unwrap()).unwrap();
        }
        _ => {
            user1 = cache.member(args.m.guild_id.unwrap(), args.m.mentions.get(0).unwrap()).unwrap();
            user2 = cache.member(args.m.guild_id.unwrap(), args.m.mentions.get(1).unwrap()).unwrap();
        }
    }

    let mut rnd = rand::rngs::StdRng::seed_from_u64(user1.user.read().id.0 + user2.user.read().id.0).gen_range(0, 101);
    if user1.user.read().id.0 == user2.user.read().id.0 {
        rnd = 101;
    }
    let _ = args.m.channel_id.send_message(args.ctx, |f| {
        let mut message;
        if rnd < 45 {
            message = "Dude get someone else than that.";
        } else if rnd < 75 {
            message = "Yea would smash.";
        } else if rnd < 100 {
            message = "Its okay. You can keep it.";
        } else {
            message = "Would smash even if a guy.";
            if rnd == 101 {
                message = "Loving yourself huh? Pathetic.";
            }
        }
        f.content(format!("{}", message))
    });
    Ok(true)
}