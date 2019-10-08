use serenity::http::AttachmentType;
use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};

pub static URBAN_COMMAND: Command = Command {
    key: "urban",
    description: "Searches a term on UrbanDictionary",
    help_page: "#urban <term>",
    category: "Misc",
    func: urban_command,
};

#[derive(Serialize, Deserialize)]
struct FullUrbanResponse {
    list: Vec<UrbanResponse>
}



#[derive(Serialize, Deserialize)]
struct UrbanResponse {
    defid: u32,
    permalink: String,
    definition: String,
    example: String,
    author: String,
}

fn urban_command(args: CommandArguments) -> CommandResult {
    let split: Vec<&str> = args.m.content.split_whitespace().collect();
    if split.len() < 2 {
        return Ok(false);
    }
    let term = split.into_iter().skip(1);
    let term: Vec<&str> = term.collect();
    let term = term.join(" ");
    let mut res = unwrap_cmd_err!(&URBAN_COMMAND, reqwest::get( reqwest::Url::parse(&format!("https://api.urbandictionary.com/v0/define?term={}", term)).unwrap()), "could not make request to urban dictionary");

    let text: String = unwrap_cmd_err!(&URBAN_COMMAND, res.text(), "could not read urban dictionary's body");
    let mut uo: FullUrbanResponse = unwrap_cmd_err!(&URBAN_COMMAND, serde_json::from_str(&text), "could not parse urban dictionary's json body");

    if uo.list.len() == 0 {
        let _ = args.m.reply(args.ctx, "Term not found. I'm sorry :c");
        return Ok(true);
    }

    let mut uo = uo.list.first_mut().unwrap();
    uo.definition = uo.definition.replace("[", "");
    uo.definition = uo.definition.replace("]", "");
    uo.example = uo.example.replace("[", "");
    uo.example = uo.example.replace("]", "");


    let mug = unwrap_cmd_err!(&URBAN_COMMAND, super::urbanmug::get_mug(&term), "could not get urban mug");
    let mock_mug_filename = format!("{}.jpg", uo.defid);

    let result = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.thumbnail(format!("attachment://{}", &mock_mug_filename));
            eb.color(Colour::from_rgb(239, 255, 0));
            eb.title(format!(r#"Urban Dictionary: "{}""#, &term));
            eb.url(&uo.permalink);
            eb.description(format!("By user {}", &uo.author));
            eb.field("Definition", super::util::shorten_string(&uo.definition, super::util::DISCORD_EMBED_FIELD_VALE_MAX_LENGTH), false);
            eb.field("Example", super::util::shorten_string(&uo.example, super::util::DISCORD_EMBED_FIELD_VALE_MAX_LENGTH), false);

            super::util::add_timestamp(&mut eb);
            super::util::add_footer(&mut eb, &args);

            eb
        });
        mb.add_file(AttachmentType::Bytes((&mug, &mock_mug_filename)));
        mb
    });

    if result.err().is_some() {
        let _ = args.m.reply(args.ctx, "Sorry! Something went wrong :c");
    }

    Ok(true)
}