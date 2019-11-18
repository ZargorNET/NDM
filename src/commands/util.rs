use chrono::{DateTime, Utc};
use serenity::builder::CreateEmbed;

use crate::command_framework::CommandArguments;

/// https://discordapp.com/developers/docs/resources/channel#embed-limits
#[allow(dead_code)]
pub const DISCORD_EMBED_TITLE_MAX_LENGTH: usize = 256;
#[allow(dead_code)]
pub const DISCORD_EMBED_DESC_MAX_LENGTH: usize = 2048;
pub const DISCORD_EMBED_FIELD_VALE_MAX_LENGTH: usize = 1024;

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

pub fn add_footer_text<'a>(ceb: &'a mut CreateEmbed, args: &CommandArguments, text: String) -> &'a mut CreateEmbed {
    ceb.footer(|fb| {
        fb.text(format!("{}  •  {}", &args.m.author.name, text).to_string());
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

pub fn shorten_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        return s.to_owned();
    }

    let s = &s[0..max_length - 1 - 3];
    let mut s = s.to_owned();
    s.push_str("...");
    s
}

/// Returns a Vec with user ids
pub fn parse_mentions(s: &str) -> Vec<String> {
    lazy_static! {
        static ref REG: regex::Regex = regex::Regex::new(r#"<@(?P<id>[0-9]+)>"#).expect("could not compile regex");
    }
    let mut ret = Vec::new();
    let res = REG.captures_iter(s);
    for c in res {
        let id = match c.name("id") {
            Some(s) => s,
            None => continue
        };
        ret.push(id.as_str().to_owned());
    }
    ret
}