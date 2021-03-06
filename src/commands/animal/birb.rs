use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::prelude::*;
use crate::commands;

pub static BIRB_COMMAND: Command = Command {
    key: "birb",
    description: "Gets a random birb",
    help_page: "",
    category: Category::Animals,
    func: birb_command,
};

const BIRB_SLOGANS: &'static [&'static str] = &[
    "Weird plane. Should do way more noise... It must be coming from AREA 51",
    "BIRB OR IS IT BIRD",
    "NEEEEOUUWW PLANE 21 READY TO BOARD PASSENGERS",
    "WHY CAN'T I BE A SUPERSPORTS CAR *sad noises*",
    "Maybe I can speak. But maybe you're just hallucinating...",
];

fn birb_command(args: CommandArguments) -> CommandResult {
    let random_birb;
    {
        let safe = args.safe.read();
        let birb_cache = match safe.get::<Vec<String>>() {
            Some(ks) => ks,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry, no birbs cached yet! Please try again later :)! :bird:");
                return Ok(MarkAsFailed);
            }
        };

        random_birb = birb_cache.get(rand::thread_rng().gen_range(0, birb_cache.len())).unwrap().clone();
    } // RELEASE LOCK

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("Is it plane?");
            let mut ran = rand::thread_rng();
            let index = ran.gen_range(0, BIRB_SLOGANS.len());
            eb.description(BIRB_SLOGANS[index]);
            eb.colour(Colour::from_rgb(221, 46, 68));
            eb.image(random_birb);

            commands::util::add_timestamp(&mut eb);
            commands::util::add_footer(&mut eb, &args);
            eb
        })
    });
    Ok(MarkAsSucceeded)
}