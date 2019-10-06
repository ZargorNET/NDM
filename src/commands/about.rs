use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};

pub static ABOUT_COMMAND: Command = Command {
    key: "about",
    description: "Shows some info about this bot",
    help_page: "#about",
    category: "Misc",
    func: about_command,
};

fn about_command(args: CommandArguments) -> CommandResult {
    let _ = args.m.channel_id.send_message(args.ctx, |cb| {
        cb.embed(|mut eb| {
            eb.title("About this bot");
            eb.description("Hey! I am written in Rust by **ZargorNET** using serenity-rs.");
            eb.color(Colour::from_rgb(67, 181, 129));
            eb.field("Invite me!", "[click](https://discordapp.com/api/oauth2/authorize?client_id=542028360256323605&permissions=8&scope=bot)", true);
            eb.field("Join my server!", "[click](https://discord.gg/BRWTJMY)", true);

            {
                let cache = args.ctx.cache.read();
                if let Some(avatar) = &cache.user.avatar_url() {
                    eb.thumbnail(&avatar);
                }
            }

            super::util::add_timestamp(&mut eb);
            super::util::add_footer(&mut eb, &args);

            eb
        })
    });

    Ok(true)
}