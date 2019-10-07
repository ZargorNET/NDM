use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;

pub static BIRB_COMMAND: Command = Command {
    key: "birb",
    description: "Gets a random birb",
    help_page: "#birb",
    category: "Animal",
    func: birb_command,
};

pub const BIRB_CACHE_KEY: &'static str = "birbcache";

fn birb_command(args: CommandArguments) -> CommandResult {
    let random_birb;
    {
        let safe = args.safe.read();
        let birb_cache = match safe.get::<Vec<String>>(BIRB_CACHE_KEY) {
            Some(ks) => ks,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry, no birbs cached yet! Please try again later :)! :bird:");
                return Ok(true);
            }
        };

        random_birb = birb_cache.get(rand::thread_rng().gen_range(0, birb_cache.len())).unwrap().clone();
    } // RELEASE LOCK

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("Is it plane?");
            eb.description("Weird plane. Should do way more noise... It must be coming from AREA 51");
            eb.colour(Colour::from_rgb(221, 46, 68));
            eb.image(random_birb);

            commands::util::add_timestamp(&mut eb);
            commands::util::add_footer(&mut eb, &args);
            eb
        })
    });
    Ok(true)
}