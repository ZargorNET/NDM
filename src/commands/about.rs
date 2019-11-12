use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};

pub static ABOUT_COMMAND: Command = Command {
    key: "about",
    description: "Shows some info about this bot",
    help_page: "",
    category: "Misc",
    func: about_command,
};

fn about_command(args: CommandArguments) -> CommandResult {
    let _ = args.m.channel_id.send_message(args.ctx, |cb| {
        cb.embed(|mut eb| {
            eb.title("About this bot");
            eb.description("Hey! I am written in Rust by **ZargorNET** and **Turulix** using serenity-rs.");
            eb.field("Invite me!", "[click](https://discordapp.com/oauth2/authorize?client_id=277608782123630593&scope=bot&permissions=8&guild_id=0)", true);
            eb.field("Join my server!", "[click](https://discord.gg/CYVjCvV)", true);

            {
                let nocolor = Colour::from_rgb(67, 181, 129);
                let cache = args.ctx.cache.read();
                if let Some(avatar) = &cache.user.avatar_url() {
                    eb.thumbnail(&avatar);
                }
                //TODO Color
            }

            super::util::add_timestamp(&mut eb);
            super::util::add_footer(&mut eb, &args);

            eb
        })
    });

    Ok(true)
}