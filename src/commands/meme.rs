use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;

pub static MEME_COMMAND: Command = Command {
    key: "meme",
    description: "Gets a random meme",
    help_page: "",
    category: "Fun",
    func: meme_command,
};

pub struct Meme {
    pub title: String,
    pub url: String,
    pub image: String,
    pub author: String,
    pub subreddit: String,
    pub upvotes: u32,
}

pub const MEME_CACHE_KEY: &'static str = "memecache";

fn meme_command(args: CommandArguments) -> CommandResult {
    let safe = args.safe.read();
    let meme_cache = safe.get::<Vec<Meme>>(MEME_CACHE_KEY);

    if meme_cache.is_none() {
        let _ = args.m.channel_id.say(args.ctx, "No memes fetched yet! Try again later");
        return Ok(true);
    }
    let meme_cache = meme_cache.unwrap();

    let index = rand::thread_rng().gen_range(0, meme_cache.len());
    let meme: &Meme = meme_cache.get(index).unwrap();

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            let r = rand::thread_rng().gen_range(0, 255);
            let g = rand::thread_rng().gen_range(0, 255);
            let b = rand::thread_rng().gen_range(0, 255);
            eb.color(Colour::from_rgb(r, g, b));
            eb.title(&meme.title);
            eb.description(format!("{} 👍 {}", &meme.subreddit, &meme.upvotes));
            eb.url(&meme.url);
            eb.image(&meme.image);

            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);
            eb
        })
    });

    Ok(true)
}