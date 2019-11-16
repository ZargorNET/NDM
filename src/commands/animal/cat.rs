use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;
use crate::util::enums::category::Category;

pub static CAT_COMMAND: Command = Command {
    key: "cat",
    description: "Gives you a random cat",
    help_page: "",
    category: Category::Animals,
    show_on_help: true,
    func: cat_command,
};

/*const CAT_SLOGANS: &'static [&'static str] = &[
    "MEEOW I DIDN'T DO ANYTHING",
    "MEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEOW",
    "FOOOOD NOW HUMAN!",
    "WHERE ARE MY HUMANS? THEY NEED TO PAT ME",
    "MEOW",
    "ME OWL",
    "YOU DON'T SEE MY BC I AM STEALTH MASTER",
    "SNEAK, SNEAK, SNEAK",
];*/


fn cat_command(args: CommandArguments) -> CommandResult {
    let mut res = unwrap_cmd_err!(&CAT_COMMAND, reqwest::get("https://aws.random.cat/meow"), "could not connect to cat service");
    let text = unwrap_cmd_err!(&CAT_COMMAND, res.text(), "could not read cat service's body");

    #[derive(Serialize, Deserialize)]
    struct WebResponse {
        file: String
    }

    let cat_url: WebResponse = unwrap_cmd_err!(&CAT_COMMAND, serde_json::from_str(&text), "could not parse cat service's json body");

    // POST IT
    let _ = args.m.channel_id.send_message(args.ctx, |mb| mb.embed(|mut eb| {
        eb.title("Awwwww!");
        //let mut ran = rand::thread_rng();
        //let index = ran.gen_range(0, CAT_SLOGANS.len());
        //eb.description(CAT_SLOGANS[index]);
        eb.color(Colour::from_rgb(255, 154, 136));
        eb.image(cat_url.file);

        commands::util::add_footer(&mut eb, &args);
        commands::util::add_timestamp(&mut eb);

        eb
    }));

    //super::add_cat_sup(&args);

    Ok(true)
}