use rand::Rng;
use serenity::utils::Colour;

use crate::command_framework::prelude::*;
use crate::commands;

pub static DOG_COMMAND: Command = Command {
    key: "dog",
    description: "Shows you a dog :)!",
    help_page: "[<optional: breed>]",
    category: Category::Animals,
    func: dog_command,
};

pub static DOG_BREEDS_COMMAND: Command = Command {
    key: "dogbreeds",
    description: "Shows you all available breeds",
    help_page: "",
    category: Category::Animals,

    func: dog_breed_command,
};

pub struct DogCache {
    pub breeds: Vec<DogBreed>
}

pub struct DogBreed {
    pub name: String,
    pub images: Vec<String>,
}

fn dog_command(args: CommandArguments) -> CommandResult {
    let split: Vec<&str> = args.m.content.split_whitespace().collect();

    #[derive(Serialize, Deserialize)]
    struct DogResponse {
        message: String,
        status: String,
    }

    let dog_url;

    {
        let safe = args.safe.read();
        let dog_cache = match safe.get::<DogCache>() {
            Some(s) => s,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry, no dogs cached yet! Please try again later :dog2:");
                return Ok(MarkAsFailed);
            }
        };

        let dog_breed;

        if split.len() == 1 {
            // RANDOM DOG
            let index = rand::thread_rng().gen_range(0, dog_cache.breeds.len());
            dog_breed = dog_cache.breeds.get(index).unwrap();
        } else if split.len() == 2 {
            dog_breed = match dog_cache.breeds.iter().find(|b| b.name.to_lowercase() == split[1].to_lowercase()) {
                Some(s) => s,
                None => {
                    let _ = args.m.reply(args.ctx, format!("Dog breed not found! View all breeds using ``{}dogbreeds``", args.settings.default_prefix));
                    return Ok(MarkAsFailed);
                }
            };
        } else { return Ok(PrintUsage); }

        let index = rand::thread_rng().gen_range(0, dog_breed.images.len());
        dog_url = dog_breed.images.get(index).unwrap().clone();
    }
    let _ = args.m.channel_id.send_message(args.ctx, |cb| {
        cb.embed(|mut eb| {
            eb.title("Woofy boy!");
            eb.image(dog_url);

            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);

            eb.color(Colour::new(0x947867));
            eb
        });
        cb
    });

    Ok(MarkAsSucceeded)
}

fn dog_breed_command(args: CommandArguments) -> CommandResult {
    let mut breeds: Vec<String> = Vec::new();

    {
        let safe = args.safe.read();
        let dog_cache = match safe.get::<DogCache>() {
            Some(s) => s,
            None => {
                let _ = args.m.reply(args.ctx, "Sorry, no dog breeds cached yet! Please try again later");
                return Ok(MarkAsFailed);
            }
        };

        for breed in dog_cache.breeds.iter() {
            breeds.push(breed.name.clone());
        }
    }

    breeds.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let s = breeds.join("\n");

    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.field("All available dog breeds", s, true);
            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);
            eb
        })
    });
    Ok(MarkAsSucceeded)
}