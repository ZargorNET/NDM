use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;

pub static FOX_COMMAND: Command = Command {
    key: "fox",
    description: "Gives you random fox",
    help_page: "",
    category: "Animals",
    func: fox_command,
};

fn fox_command(args: CommandArguments) -> CommandResult {
    let mut res = unwrap_cmd_err!(&FOX_COMMAND, reqwest::get("https://randomfox.ca/floof/"), "could not fetch random fox from service");
    let text = unwrap_cmd_err!(&FOX_COMMAND, res.text(), "could not read body from fox service's response");
    let fox: FoxResponse = unwrap_cmd_err!(&FOX_COMMAND, serde_json::from_str(&text), "could not parse json body from fox's service");

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("Look! A foxy boy!");
            eb.color(Colour::from_rgb(255, 104, 0));
            eb.description("We will never know if it's just a Fury");
            eb.image(fox.image);

            commands::util::add_timestamp(&mut eb);
            commands::util::add_footer(&mut eb, &args);
            eb
        })
    });

    Ok(true)
}

#[derive(Serialize, Deserialize)]
struct FoxResponse {
    image: String
}