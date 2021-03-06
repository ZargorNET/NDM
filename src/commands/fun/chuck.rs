use serenity::utils::Colour;

use crate::command_framework::prelude::*;
use crate::commands;

pub static CHUCK_COMMAND: Command = Command {
    key: "chuck",
    description: "Gets you a random chuck norris joke",
    help_page: "",
    category: Category::Fun,
    func: chuck_command,
};

fn chuck_command(args: CommandArguments) -> CommandResult {
    let first = "Chuck";
    let second = "Norris";

    let mut res = unwrap_cmd_err!(&CHUCK_COMMAND, reqwest::get(&format!("http://api.icndb.com/jokes/random")), "could not fetch chuck norris joke from service");
    let text = unwrap_cmd_err!(&CHUCK_COMMAND, res.text(), "could not read chuck norris joke's body");

    let joke: ChuckNorrisResponse = unwrap_cmd_err!(&CHUCK_COMMAND, serde_json::from_str(&text), "could not parse json from chuck norris joke");
    let joke = joke.value.joke.replace("&quot;", "**");

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title(format!("A {} {} joke", &first, &second));
            eb.description(joke);
            eb.colour(Colour::from_rgb(45, 15, 63));

            commands::util::add_timestamp(&mut eb);
            commands::util::add_footer(&mut eb, &args);
            eb
        })
    });

    Ok(MarkAsSucceeded)
}

#[derive(Serialize, Deserialize)]
struct ChuckNorrisResponse {
    value: ChuckNorrisResponseValue
}

#[derive(Serialize, Deserialize)]
struct ChuckNorrisResponseValue {
    id: u32,
    joke: String,
}