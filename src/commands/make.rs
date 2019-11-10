use std::io::Read;
use std::sync::Arc;

use reqwest::Response;
use serenity::http::AttachmentType;
use serenity::model::user::User;
use serenity::prelude::RwLock;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::util::image::feature::FeatureType;
use crate::util::image::partial::PartialFeature;
use crate::util::image::Template;

pub static MAKE_COMMAND: Command = Command {
    key: "make",
    description: "Make a meme",
    help_page: "#make",
    category: "Fun",
    func: make_command,
};


fn make_command(args: CommandArguments) -> CommandResult {
    let split: Vec<&str> = args.m.content.split_whitespace().collect();

    if split.len() == 1 {
        let _ = args.m.channel_id.send_message(args.ctx, |mb| {
            mb.embed(|mut eb| {
                let mut buf = "".to_owned();
                for s in args.image.get_all_keys() {
                    buf.push_str(&s);
                    buf.push_str(" | ");
                }
                let to_print = &buf[0..buf.len() - 3];

                eb.title("Please specify which meme you want to generate");
                eb.description(format!("``#make <{}>``", to_print));

                super::util::add_timestamp(&mut eb);
                super::util::add_footer(&mut eb, &args);

                eb
            })
        });
        return Ok(true);
    }

    let make_name = split[1].to_lowercase();
    let required_features = match args.image.get_required_features(&make_name) {
        Some(s) => s,
        None => {
            let _ = args.m.reply(args.ctx, "Meme template not found, sorry.");
            return Ok(true);
        }
    };
    let feature_count = required_features.len();

    let mut images: Vec<(String, PartialFeature)> = Vec::new();
    let mut text: Option<(String, PartialFeature)> = None;

    for f in required_features {
        match f.kind {
            FeatureType::Image => {
                images.push((f.key.clone(), f));
            }
            FeatureType::Text => {
                if text.is_some() {
                    warn!("MAKE CMD: Got 2 times or more text features");
                }
                text = Some((f.key.clone(), f));
            }
        };
        //
    }

    if split.len() < 2 + feature_count {
        let mut buf = "".to_owned();
        for (k, _v) in images {
            buf.push_str(&format!("<{}:@User> ", k));
        }

        if text.is_some() {
            buf.push_str(&format!("<{}:Text> ", text.unwrap().0));
        }

        let _ = args.m.channel_id.send_message(args.ctx, |mb| {
            mb.embed(|mut eb| {
                eb.title(format!(r#"Meme Maker: "{}""#, &make_name));
                eb.description(format!("Please specify following parameters: \n``#make {} {}``", &make_name, buf));

                super::util::add_timestamp(&mut eb);
                super::util::add_footer(&mut eb, &args);
                eb
            })
        });
        return Ok(true);
    }
    let mut tp = args.image.start_building(&make_name).unwrap();

    let mut args_index = 2usize;
    let mut mention_index = 0usize;

    let mentioned_user_ids = super::util::parse_mentions(&args.m.content);
    let mut mentions: Vec<Arc<RwLock<User>>> = Vec::with_capacity(mentioned_user_ids.len());
    for id in mentioned_user_ids {
        let l = args.ctx.cache.read();
        let user = match l.user(id.parse::<u64>().unwrap()) {
            Some(s) => s,
            None => continue
        };
        mentions.push(user);
    }

    let http = reqwest::Client::new();
    for (k, _v) in images {
        let mut avatar;
        {
            let mention = match mentions.get(mention_index) {
                Some(s) => s,
                None => {
                    let _ = args.m.reply(args.ctx, "Sorry, you've entered an invalid target user! Please try again");
                    return Ok(true);
                }
            };
            let mention = mention.read();
            avatar = match mention.avatar_url() {
                Some(s) => s,
                None => {
                    let _ = args.m.reply(args.ctx, "Sorry, at least one of your target users doesn't have a custom avatar");
                    return Ok(true);
                }
            };
        } // RELEASE LOCK
        avatar = avatar.replace(".webp?size=1024", ".png?size=512"); // IMAGE LIB DOES NOT FULLY SUPPORT .WEBP

        let url = unwrap_cmd_err!(&MAKE_COMMAND, reqwest::Url::parse(&avatar), "could not build avatar url");
        let req = unwrap_cmd_err!(&MAKE_COMMAND, http.get(url).build(), "could not build avatar command");
        let mut res: Response = unwrap_cmd_err!(&MAKE_COMMAND, http.execute(req), "could not execute avatar request");
        let mut buf = Vec::new();
        unwrap_cmd_err!(&MAKE_COMMAND, res.read_to_end(&mut buf), "could not read avatar request's body");
        let img = unwrap_cmd_err!(&MAKE_COMMAND, image::load_from_memory(&buf), "could not transform avatar image to DynamicImage struct");
        unwrap_cmd_err!(&MAKE_COMMAND, tp.set_image(&k, img), "could not set image");

        args_index += 1;
        mention_index += 1;
    }

    if text.is_some() {
        let text = text.unwrap();

        let other = split[args_index..split.len()].to_vec();
        let other: String = other.join(" ");

        unwrap_cmd_err!(&MAKE_COMMAND, tp.set_text(&text.0, other), "could not set text");
    }

    let template: Template = unwrap_cmd_err!(&MAKE_COMMAND, tp.build(), "could not build template");
    let img_buf: Vec<u8> = unwrap_cmd_err!(&MAKE_COMMAND, template.apply(), "could not apply template");

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.image("attachment://make_image.png");
            super::util::add_timestamp(&mut eb);
            super::util::add_footer(&mut eb, &args);
            eb
        });
        mb.add_file(AttachmentType::Bytes((&img_buf, "make_image.png")));
        mb
    });

    Ok(true)
}