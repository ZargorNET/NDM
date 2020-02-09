use std::io::Read;

use reqwest::Response;
use serenity::http::AttachmentType;

use crate::command_framework::prelude::*;
use crate::util::image::{ImageStorage, Template};
use crate::util::image::feature::FeatureType;

pub mod command_gen;

fn image_gen(args: CommandArguments) -> CommandResult {
    let mut split = args.m.content.split_whitespace();

    let key = split.next().unwrap();
    let required_features = args.image.get_required_features(key).unwrap();
    let user_images_count = required_features.iter().filter(|f| f.kind == FeatureType::UserImage).count();
    let mut template = args.image.start_building(key).unwrap();

    let http = reqwest::Client::new();

    let mentions = super::util::parse_mentions(&args.m.content);
    for (i, feature) in required_features.into_iter().enumerate() {
        match feature.kind {
            FeatureType::Image => { unwrap_cmd_err!(args.command, template.set_image(&feature.key), "could not set image"); }
            FeatureType::SplitText => {
                let mut t = String::new();
                for mut next in split.by_ref() {
                    let mut stop = false;
                    if next.ends_with(",") {
                        next = &next[..next.len() - 1];
                        stop = true;
                    }

                    t.push_str(next);
                    t.push_str(" ");

                    if stop {
                        break;
                    }
                }
                unwrap_cmd_err!(args.command, template.set_text(&feature.key, t), "could not set split text");
            }
            FeatureType::UserImage => {
                let user = if i == 0 && feature.default_user.unwrap_or_default() == true && mentions.len() != user_images_count {
                    args.m.author.clone()
                } else {
                    // OK PARSE NEXT MENTION
                    let next = match split.by_ref().next() {
                        Some(s) => s,
                        None => {
                            let _ = args.m.channel_id.send_message(args.ctx, |mb| {
                                mb.embed(|mut eb| {
                                    eb.title(format!(r#"Meme Maker: "{}""#, key));
                                    eb.description(format!("Please specify following parameters: \n``{}{} {}``", args.settings.default_prefix, key, print_template_features(&args.image, key)));

                                    super::util::add_timestamp(&mut eb);
                                    super::util::add_footer(&mut eb, &args);
                                    eb
                                })
                            });
                            return Ok(MarkAsWrongUsage);
                        }
                    };

                    let mention = match super::util::parse_mentions(next).get(0) {
                        Some(s) => s.clone(),
                        None => return Ok(PrintUsage)
                    };

                    match args.m.mentions.iter().find(|m| m.id.0 == mention.parse::<u64>().unwrap()) {
                        Some(s) => s.clone(),
                        None => {
                            let _ = args.m.reply(args.ctx, "User not found");
                            return Ok(MarkAsFailed);
                        }
                    }
                };

                let mut avatar_url = match user.static_avatar_url() {
                    Some(s) => s,
                    None => {
                        let _ = args.m.reply(args.ctx, "Sorry at least one of your specified users(or yourself) don't have a valid avatar");
                        return Ok(MarkAsFailed);
                    }
                };

                avatar_url = avatar_url.replace(".webp?size=1024", ".png?size=128"); // IMAGE LIB DOES NOT FULLY SUPPORT .WEBP

                let url = unwrap_cmd_err!(args.command, reqwest::Url::parse(&avatar_url), "could not build avatar url");
                let req = unwrap_cmd_err!(args.command, http.get(url).build(), "could not build avatar command");
                let mut res: Response = unwrap_cmd_err!(args.command, http.execute(req), "could not execute avatar request");
                let mut buf = Vec::new();
                unwrap_cmd_err!(args.command, res.read_to_end(&mut buf), "could not read avatar request's body");
                let img = unwrap_cmd_err!(args.command, image::load_from_memory(&buf), "could not transform avatar image to DynamicImage struct");
                unwrap_cmd_err!(args.command, template.set_user_image(&feature.key, img), "could not set user image");
            }
            FeatureType::Text => {
                let mut t = String::new();

                for next in split.by_ref() {
                    t.push_str(next);
                    t.push_str(" ");
                }
                unwrap_cmd_err!(args.command, template.set_text(&feature.key, t), "could not set text");
            }
        }
    }

    let template: Template = unwrap_cmd_err!(args.command, template.build(), "could not build template");
    let img_buf: Vec<u8> = unwrap_cmd_err!(args.command, template.apply(), "could not apply template");

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.image("attachment://make_image.png");
            super::util::add_timestamp(&mut eb);
            super::util::add_footer(&mut eb, &args);
            eb
        });
        mb.add_file(AttachmentType::Bytes { data: img_buf.into(), filename: "make_image.png".to_string() });
        mb
    });

    Ok(MarkAsSucceeded)
}

fn print_template_features(images: &ImageStorage, template_key: &str) -> String {
    let mut buf = "".to_owned();

    let req_features = images.get_required_features(template_key).unwrap();
    for f in req_features {
        match f.kind {
            FeatureType::UserImage => {
                if f.default_user.unwrap_or_default() == true {
                    buf.push_str(&format!("[<{}:@User>] ", f.key));
                } else {
                    buf.push_str(&format!("<{}:@User> ", f.key));
                }
            }
            FeatureType::SplitText => {
                buf.push_str(&format!("<{}:Text>, ", f.key));
            }
            FeatureType::Text => {
                buf.push_str(&format!("<{}:Text> ", f.key));
            }
            FeatureType::Image => {}
        }
    }

    if buf.ends_with(", ") {
        buf.drain((buf.len() - 2)..);
    }
    if buf.ends_with(" ") { buf.drain(buf.len() - 1..); }


    buf
}