use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::prelude::*;
use crate::commands;

pub static RABBIT_COMMAND: Command = Command {
    key: "rabbit",
    description: "Gets you an hoppyboi",
    help_page: "",
    category: Category::Animals,
    func: rabbit_command,
};

fn rabbit_command(args: CommandArguments) -> CommandResult {
    let rabbit;

    {
        let safe = args.safe.read();
        let rabbits = match safe.get::<Vec<Rabbit>>() {
            Some(s) => s,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry, no rabbits fetched yet :c Try again later :rabbit2:");
                return Ok(MarkAsFailed);
            }
        };
        rabbit = rabbits[rand::thread_rng().gen_range(0, rabbits.len())].clone();
    }

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("Look! A sweet hoppyboi");
            eb.description(rabbit.title);
            eb.image(rabbit.url);
            eb.colour(Colour::from_rgb(255, 255, 255));

            commands::util::add_timestamp(&mut eb);
            commands::util::add_footer(&mut eb, &args);
            eb
        })
    });

    Ok(MarkAsSucceeded)
}

#[derive(Clone)]
pub struct Rabbit {
    pub title: String,
    pub url: String,
}