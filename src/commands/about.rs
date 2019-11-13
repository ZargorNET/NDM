use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};

pub static ABOUT_COMMAND: Command = Command {
    key: "about",
    description: "Shows some info about this bot",
    help_page: "",
    category: "Misc",
    func: about_command,
};

pub const STATISTICS_CACHE_KEY: &'static str = "STATISTICS";

#[derive(Clone)]
pub struct Statistics {
    pub num_text_channels: u32,
    pub num_voice_channels: u32,
    pub num_servers: u32,
    pub num_users: u32,
}

fn about_command(args: CommandArguments) -> CommandResult {
    let _ = args.m.channel_id.send_message(args.ctx, |cb| {
        cb.embed(|eb| {
            //eb.description("Hey! I am written in Rust by **ZargorNET** and **Turulix** using serenity-rs.");
            //eb.field("Invite me!", "[click](https://discordapp.com/oauth2/authorize?client_id=277608782123630593&scope=bot&permissions=8&guild_id=0)", true);
            //eb.field("Join my server!", "[click](https://discord.gg/CYVjCvV)", true);

            let avatar_url;
            let username;
            let member;
            let shard_count;
            {
                let cache = args.ctx.cache.read();
                avatar_url = cache.user.avatar_url();
                username = cache.user.name.clone();
                member = cache.member(&args.m.guild_id.unwrap(), cache.user.id);
                shard_count = cache.shard_count;
            } // DROP CACHE LOCK

            let nocolor = Colour::from_rgb(67, 181, 129);

            if let Some(avatar) = avatar_url {
                eb.author(|fb| {
                    fb.name(format!("All about {}", &username));
                    fb.icon_url(avatar)
                });
            }
            if let Some(color) = match member {
                Some(member) => {
                    match member.colour(&args.ctx.cache) {
                        Some(color) => { Some(color) }
                        None => { Some(nocolor) }
                    }
                }
                None => Some(nocolor)
            } { eb.color(color); }
            eb.description(format!("Hello! I am **{}**\n\
                I was written in Rust by **ZargorNET** and **Turulix** useing [serenity-rs](https://github.com/serenity-rs/serenity)\n\
                Type `{}help` to see my commands!\n Join my server [`here`](https://discord.gg/CYVjCvV)\
                , or [`invite`](https://discordapp.com/oauth2/authorize?client_id=277608782123630593&scope=bot&permissions=8&guild_id=0) me to your server!\n\n\
                Some of my features include:\n\
                \
                ```css\n\
                ✅ Memes\n\
                ✅ Nsfw ;)\n\
                ✅ Administration Commands\n\
                ```", username, args.settings.read().default_prefix));


            let statistics;
            {
                statistics = match args.safe.read().get::<Statistics>(STATISTICS_CACHE_KEY) {
                    Some(s) => Some(s.clone()),
                    None => {
                        eb.field("Statistics not available", "", true);
                        None
                    }
                };
            }

            if statistics.is_some() {
                let statistics = statistics.unwrap();
                eb.field("Stats", format!("{} Servers\n{} Shards", statistics.num_servers, shard_count), true);
                eb.field("This Shard", format!("{} Users\n{} Servers", statistics.num_users, statistics.num_servers), true);
                eb.field("Channels", format!("{} Text Channels\n{} Voice Channels", statistics.num_text_channels, statistics.num_voice_channels), true);
            }
            eb.footer(|f| {
                f.text("Last restart")
            });
            eb.timestamp(args.settings.read().start_time.to_rfc3339());
            eb
        })
    });

    Ok(true)
}