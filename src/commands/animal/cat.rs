use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;

pub static CAT_COMMAND: Command = Command {
    key: "cat",
    description: "Gives you a random cat",
    help_page: "#cat",
    category: &commands::CATEGORY_IMAGES,
    func: cat_command,
};

fn cat_command(args: CommandArguments) -> CommandResult {
    let mut res = unwrap_cmd_err!(&CAT_COMMAND, reqwest::get("htta://aws.random.cat/meow"), "could not connect to cat service");


    let text = unwrap_cmd_err!(&CAT_COMMAND, res.text(), "could not read cat service's body");

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        file: String
    }

    let cat_url: WebResponse = unwrap_cmd_err!(&CAT_COMMAND, serde_json::from_str(&text), "could not parse cat service's json body");

    // POST IT
    let _ = args.m.channel_id.send_message(args.ctx, |mb| mb.embed(|mut eb| {
        eb.title("Cats, cats and more cats!");
        eb.description("Meeeeeeeeeeeeeeow");
        eb.color(Colour::from_rgb(255, 154, 136));
        eb.field("CATS VS DOGS", "Registered vote for CATS! ``#dcwar``", true);
        eb.image(cat_url.file);

        commands::util::add_footer(&mut eb, &args);
        commands::util::add_timestamp(&mut eb);

        eb
    }));

    super::add_cat_sup(&args);

    Ok(true)
}