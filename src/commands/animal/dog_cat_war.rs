use serenity::utils::Colour;

use crate::command_framework::{Command, CommandArguments, CommandResult};
use crate::commands;

pub static DOG_CAT_WAR_COMMAND: Command = Command {
    key: "dcwar",
    description: "The epic war between dogs and cats",
    help_page: "",
    category: "Animals",
    func: dog_war_command,
};

fn dog_war_command(args: CommandArguments) -> CommandResult {
    let _ = args.m.channel_id.send_message(args.ctx, |mb| {
        mb.embed(|mut eb| {
            eb.title("DOG - CAT WAR");
            eb.description("THE EPIC FIGHT BETWEEN DOGS AND CATS");
            eb.color(Colour::new(0x39ff14));

            let dcwar = super::get_dog_cat_sup(&args);

            eb.field("DOGS", format!("{} votes!", dcwar.dog_sup), true);
            eb.field("CATS", format!("{} votes!", dcwar.cat_sup), true);

            // CALCULATE
            let s_to_print;
            let total = dcwar.dog_sup + dcwar.cat_sup;
            if total > 0 {
                let dog: f32 = dcwar.dog_sup as f32 / total as f32;
                let cat: f32 = dcwar.cat_sup as f32 / total as f32;

                let dog_count = (10 as f32 * dog).round() as i32;
                let cat_count = (10 as f32 * cat).round() as i32;

                let mut dog_s = String::new();
                let mut cat_s = String::new();

                for _ in 0..dog_count {
                    dog_s.push_str(":large_orange_diamond:");
                }
                for _ in 0..cat_count {
                    cat_s.push_str(":large_blue_diamond:");
                }


                if dog > cat {
                    s_to_print = format!("🐶{}{}🐱\nDOGS ARE WINNING", dog_s, cat_s);
                } else {
                    s_to_print = format!("🐶{}{}🐱\nCATS ARE WINNING", dog_s, cat_s);
                }
            } else {
                s_to_print = "No votes yet".to_owned();
            }

            eb.field("SUMMARY", s_to_print, false);

            commands::util::add_footer(&mut eb, &args);
            commands::util::add_timestamp(&mut eb);
            eb
        })
    });
    Ok(true)
}