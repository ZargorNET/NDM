use std::collections::HashMap;
use std::error::Error;

use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandError, CommandResult};
use crate::commands;

pub static DOG_COMMAND: Command = Command {
    key: "dog",
    description: "Shows you a dog :)!",
    help_page: "#dog [<optional: breed>]",
    category: "Images",
    func: dog_command,
};

pub static DOG_BREEDS_COMMAND: Command = Command {
    key: "dogbreeds",
    description: "Shows you all available breeds",
    help_page: "#dogbreeds",
    category: "Images",
    func: dog_breed_command,
};

const DOG_SLOGANS: &'static [&'static str] = &[
    "WHO LET THE DOGS OUT? WOOF WOOF",
    "MEOW I'M JUST A CAT",
    "Happiness is a warm puppy :)",
    "NEED FOOD, GOT FOOD, NEED PETS",
    "WOOF.",
    "WOOOF?",
    "WOOOOOOOOOOOOOOOOOOOOOOOOOOF",
    "PLEASE PET ME",
    "i luv you. woof",
    "where is my hoooman?",
    "i want pettttssssss"
];

fn dog_command(args: CommandArguments) -> CommandResult {
    let split: Vec<&str> = args.m.content.split_whitespace().collect();

    let dog_url: String;

    #[derive(Serialize, Deserialize)]
    struct DogResponse {
        message: String,
        status: String,
    }

    if split.len() == 1 {
        // RANDOM DOG
        let mut dog_res = unwrap_cmd_err!(&DOG_COMMAND, reqwest::get("https://dog.ceo/api/breeds/image/random"), "could not get random dog from service");
        let dog_res: String = unwrap_cmd_err!(&DOG_COMMAND, dog_res.text(), "could not get random dog's body text from service");

        let dog: DogResponse = unwrap_cmd_err!(&DOG_COMMAND, serde_json::from_str(&dog_res), "could not parse random dog's body text json");

        if dog.status != "success" {
            return Err(CommandError::new_str(&DOG_COMMAND, "dog service did not successfully return"));
        }

        dog_url = dog.message;
    } else if split.len() == 2 {
        let mut dog_res = unwrap_cmd_err!(&DOG_COMMAND, reqwest::get(&format!("https://dog.ceo/api/breed/{}/images/random", split[1].to_lowercase())), "could not get dog from service");
        let dog_res: String = unwrap_cmd_err!(&DOG_COMMAND, dog_res.text(), "could not get dog's body text from service");

        let dog: DogResponse = unwrap_cmd_err!(&DOG_COMMAND, serde_json::from_str(&dog_res), "could not parse dog's body text json");

        if dog.status != "success" {
            let _ = args.m.channel_id.say(args.ctx, "Dog breed unknown. Show all breeds using ``#dogbreeds``");
            return Ok(true);
        }

        dog_url = dog.message;
    } else { return Ok(false); }

    let _ = args.m.channel_id.send_message(args.ctx, |cb| {
        cb.embed(|mut eb| {
            eb.title("GIVE ME DA WOOF!");
            let mut ran = rand::thread_rng();
            let index = ran.gen_range(0, DOG_SLOGANS.len());
            eb.description(DOG_SLOGANS[index]);
            eb.field("DOGS VS CATS", "Registered vote for DOGS! ``#dcwar``", true);
            eb.image(dog_url);

            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);

            eb.color(Colour::new(0x947867));
            eb
        });
        cb
    });

    super::add_dog_sup(&args);

    Ok(true)
}

fn dog_breed_command(args: CommandArguments) -> CommandResult {
    let mut breeds_res = unwrap_cmd_err!(&DOG_BREEDS_COMMAND, reqwest::get("https://dog.ceo/api/breeds/list/all"), "could not get breed list from service");
    let breeds_res: String = unwrap_cmd_err!(&DOG_BREEDS_COMMAND, breeds_res.text(), "could not read service's dog breed list body");

    #[derive(Serialize, Deserialize)]
    struct DogBreeds {
        message: HashMap<String, serde_json::Value>
    }

    let breeds: DogBreeds = unwrap_cmd_err!(&DOG_BREEDS_COMMAND, serde_json::from_str(&breeds_res), "could not parse dog breed's body to json");

    let mut s = "".to_owned();
    for (k, _) in breeds.message.iter() {
        s.push_str(&format!("{}\n", k));
    }
    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.field("All available dog breeds", s, false);
            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);
            eb
        })
    });
    Ok(true)
}