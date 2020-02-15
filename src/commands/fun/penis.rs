use rand::prelude::StdRng;
use rand::Rng;
use serenity::model::misc::Mentionable;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::command_framework::CommandAction::{MarkAsSucceeded, PrintUsage};
use crate::commands::category::Category;

pub static PENIS_COMMAND: Command = Command {
    key: "penis",
    description: "We all know who got the longest ;)",
    help_page: "<user: @User>",
    category: Category::Fun,
    func: penis_command,
};

fn penis_command(args: CommandArguments) -> CommandResult {
    return match args.m.mentions.get(0) {
        Some(t) => {
            let mut rng: StdRng = rand::SeedableRng::seed_from_u64(t.id.0);
            let random_value = rng.gen_range(0, 30);
            let mut penis_string = "8".to_string();
            for _ in 0..random_value {
                penis_string.push('=');
            }
            penis_string.push('D');
            if t.id.0 == 262702226693160970 || t.id.0 == 141268459991334912 || t.id.0 == 148363937598013440 {
                penis_string = "8===============================D".to_string();
            } else if t.id.0 == 241998290206326785 {
                penis_string = "8D".to_string();
            } else if t.id.0 == 277608782123630593 {
                penis_string = "8bit".to_string();
            } else if t.id.0 == 324838112213729280 {
                penis_string = "4bit".to_string();
            } else if t.id.0 == 647048181636399135 {
                penis_string = "2bit".to_string();
            }
            let _ = args.m.channel_id.send_message(args.ctx, |f| {
                f.content(t.mention() + "'s Size: " + penis_string.as_str())
            });
            Ok(MarkAsSucceeded)
        }
        None => {
            Ok(PrintUsage)
        }
    };
}