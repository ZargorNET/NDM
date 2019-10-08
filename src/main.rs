extern crate base64;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate log_panics;
extern crate rand;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serenity;
extern crate simplelog;
extern crate tempfile;
extern crate time;

use std::fs::File;
use std::sync::Arc;

use serenity::model::prelude::*;
use serenity::prelude::*;
use simplelog::{CombinedLogger, Config, LevelFilter, TerminalMode, TermLogger, WriteLogger};

use crate::command_framework::{CommandArguments, CommandManager};
use crate::safe::Safe;
use crate::scheduler::Scheduler;

mod util;
mod safe;
mod scheduler;
#[macro_use]
mod command_framework;
mod commands;
mod schedules;


struct Handler {
    ch: Arc<RwLock<CommandManager>>,
    scheduler: Arc<RwLock<Scheduler>>,
    safe: Arc<RwLock<Safe>>,
}

impl Handler {
    fn new(ch: CommandManager) -> Handler {
        let ch = Arc::new(RwLock::new(ch));
        let safe = Arc::new(RwLock::new(Safe::new()));
        let scheduler = Scheduler::new(Arc::clone(&ch), Arc::clone(&safe));
        Handler {
            ch,
            scheduler,
            safe,
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if !msg.content.starts_with("#") {
            return;
        }

        let mut msg = msg.clone();
        msg.content = msg.content[1..].to_string();
        let msg_split: Vec<&str> = msg.content.split_whitespace().collect();

        let cmd;

        {
            let command_manager_arc = Arc::clone(&self.ch);
            let command_manager = command_manager_arc.read();
            match command_manager.get_command(msg_split[0]) {
                Some(c) => cmd = c.clone(),
                None => {
                    let _ = msg.reply(&ctx, "Command not found!");
                    return;
                }
            }
        } // DROP READ LOCK
        {
            let args = CommandArguments::new(&ctx, &msg, Arc::clone(&self.ch), Arc::clone(&self.scheduler), Arc::clone(&self.safe));
            match (cmd.func)(args) {
                Ok(print_usage) => {
                    if print_usage {
                        let _ = msg.react(&ctx, ReactionType::from("✅"));
                    } else {
                        let _ = msg.react(&ctx, ReactionType::from("❌"));
                        let _ = msg.channel_id.send_message(&ctx, |eb| {
                            eb.content(format!("Invalid syntax! Try: ``{}``", cmd.help_page));
                            eb
                        });
                    }
                }
                Err(err) => {
                    let _ = msg.react(&ctx, ReactionType::from("❌"));
                    let _ = msg.reply(&ctx, "I'm sorry, I failed... There was an error executing the command. Please try again later!");
                    error!("Could not execute command: {:#?}", err);
                }
            }
        }
    }

    fn ready(&self, ctx: Context, _red: Ready) {
        let scheduler = Arc::clone(&self.scheduler);
        let mut scheduler = scheduler.write();
        scheduler.clear_all();
        scheduler.schedule_repeated(1200, schedules::fetch_memes); // EVERY 20 MINUTES
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_dogs); // EVERY 24 HOURS
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_birbs); // EVERY 24 HOURS
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_rabbits); // EVERY 24 HOURS
        scheduler.schedule_repeated(12 * 60 * 60, schedules::fetch_aww); // EVERY 12 HOURS
        ctx.set_activity(Activity::playing("trying to outperform NDM 1.0..."));
    }
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("latest.log").unwrap()),
        ]
    ).unwrap();
    log_panics::init();

    info!("Reading environment variables...");
    // VARS
    let discord_token;
    {
        use std::env::var;
        discord_token = var("DISCORD_TOKEN").expect("Need DISCORD_TOKEN var");
    }

    // REGISTER COMMANDS
    let mut command_handler = CommandManager::new();
    {
        command_handler.register_command(&commands::help::HELP_COMMAND);
        command_handler.register_command(&commands::ERROR_CMD_TEST);
        command_handler.register_command(&commands::animal::cat::CAT_COMMAND);
        command_handler.register_command(&commands::animal::dog::DOG_COMMAND);
        command_handler.register_command(&commands::animal::dog::DOG_BREEDS_COMMAND);
        command_handler.register_command(&commands::animal::dog_cat_war::DOG_CAT_WAR_COMMAND);
        command_handler.register_command(&commands::meme::MEME_COMMAND);
        command_handler.register_command(&commands::about::ABOUT_COMMAND);
        command_handler.register_command(&commands::urban::URBAN_COMMAND);
        command_handler.register_command(&commands::animal::fox::FOX_COMMAND);
        command_handler.register_command(&commands::animal::birb::BIRB_COMMAND);
        command_handler.register_command(&commands::chuck::CHUCK_COMMAND);
        command_handler.register_command(&commands::urbanmug::URBANMUG_COMMAND);
        command_handler.register_command(&commands::animal::rabbit::RABBIT_COMMAND);
        command_handler.register_command(&commands::animal::aww::AWW_COMMAND);


        for command in command_handler.get_all_commands().iter() {
            info!("Registered command: {}", command.key);
        }
    }

    // START CLIENT
    info!("Starting client");
    let mut client = Client::new(&discord_token, Handler::new(command_handler)).expect("Could not create Client");
    client.start().expect("Could not start discord client");
}
