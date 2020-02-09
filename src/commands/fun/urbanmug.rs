use std::error;

use serenity::http::AttachmentType;
use serenity::utils::Colour;

use crate::command_framework::prelude::*;

pub static URBANMUG_COMMAND: Command = Command {
    key: "urbanmug",
    description: "Gets a nice mug",
    help_page: "<text>",
    category: Category::Fun,
    func: mug_command,
};

fn mug_command(args: CommandArguments) -> CommandResult {
    let split: Vec<&str> = args.m.content.split_whitespace().collect();

    if split.len() < 2 {
        return Ok(PrintUsage);
    }

    let s = split[1..split.len()].join(" ");

    let mug = match get_mug(&s) {
        Ok(s) => s,
        Err(_) => {
            let _ = args.m.reply(args.ctx, "Sorry, it looks like UrbanDictionary has some problems... Please try again later!");
            return Ok(MarkAsFailed);
        }
    };

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title(format!(r#"The "{}" mug"#, &s));
            eb.color(Colour::from_rgb(239, 255, 0));
            eb.image("attachment://urban_mug.jpg");

            super::super::util::add_timestamp(&mut eb);
            super::super::util::add_footer(&mut eb, &args);

            eb
        });
        mb.add_file(AttachmentType::Bytes { data: mug.into(), filename: "urban_mug.jpg".to_string() });
        mb
    });

    Ok(MarkAsSucceeded)
}

pub fn get_mug(term: &str) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut term = term.to_owned();
    if term.len() > 32 {
        term = term[0..32].to_owned();
    }

    let mut res = reqwest::get(&format!("https://renderer.udimg.com/mug/all.json?background-color=fff200&word={}", term))?;
    let text = res.text()?;
    let mug: MugResponse = serde_json::from_str(&text)?;

    let mug_front = mug.front;
    let mug_front: Vec<&str> = mug_front.split(",").collect();
    let mug_front = mug_front[1];

    let mug_data = base64::decode(&mug_front)?;

    Ok(mug_data)
}

#[derive(Serialize, Deserialize)]
struct MugResponse {
    front: String
}