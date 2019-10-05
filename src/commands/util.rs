use chrono::{DateTime, Utc};
use serenity::builder::CreateEmbed;

use crate::command_framework::CommandArguments;

pub fn add_footer<'a>(ceb: &'a mut CreateEmbed, args: &CommandArguments) -> &'a mut CreateEmbed {
    ceb.footer(|fb| {
        fb.text(&args.m.author.name);
        if let Some(avatar) = &args.m.author.avatar_url() {
            fb.icon_url(avatar);
        }
        fb
    });
    ceb
}

pub fn add_timestamp(ceb: &mut CreateEmbed) -> &mut CreateEmbed {
    let now: DateTime<Utc> = Utc::now();
    ceb.timestamp(now.to_rfc3339());
    ceb
}