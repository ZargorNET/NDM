#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, Utc};
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

pub const SERENITY_CACHE_SAFE_KEY: &'static str = "SERENITY_CACHE";

pub struct StaticSettings {
    default_prefix: String,
    start_time: DateTime<Utc>,
}

struct Handler {
    ch: Arc<RwLock<CommandManager>>,
    scheduler: Arc<RwLock<Scheduler>>,
    safe: Arc<RwLock<Safe>>,
    image: Arc<util::image::ImageStorage>,
    settings: Arc<RwLock<StaticSettings>>,
}

impl Handler {
    fn new(ch: CommandManager, image: Arc<util::image::ImageStorage>) -> Handler {
        let ch = Arc::new(RwLock::new(ch));
        let safe = Arc::new(RwLock::new(Safe::new()));
        let scheduler = Scheduler::new(Arc::clone(&ch), Arc::clone(&safe));
        let settings = Arc::new(RwLock::new(StaticSettings {
            default_prefix: "+".to_string(),
            start_time: Utc::now(),
        }));

        Handler {
            ch,
            scheduler,
            safe,
            image,
            settings,
        }
    }
}

impl Handler {
    fn update_activity(&self, ctx: &Context) {
        ctx.set_activity(Activity::playing(&format!("on {} servers! | {}help", ctx.cache.read().all_guilds().len(), self.settings.read().default_prefix)));
    }
}

impl EventHandler for Handler {
    //noinspection RsTraitImplementation
    fn guild_create(&self, ctx: Context, _guild: Guild, _b: bool) {
        self.update_activity(&ctx);
    }
    //noinspection RsTraitImplementation
    fn guild_delete(&self, ctx: Context, _incomplete: PartialGuild, _full: Option<Arc<RwLock<Guild>>>) {
        self.update_activity(&ctx);
    }


    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.is_private() {
            return;
        }

        info!("[Message] {}: {}", msg.author.name, msg.content_safe(&ctx.cache));
        //TODO: Server config prefix
        if !msg.content.starts_with(&self.settings.read().default_prefix) {
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
                None => return
            }
        } // DROP READ LOCK
        {
            let args = CommandArguments::new(&ctx,
                                             &msg,
                                             Arc::clone(&self.ch),
                                             Arc::clone(&self.scheduler),
                                             Arc::clone(&self.safe),
                                             Arc::clone(&self.image),
                                             Arc::clone(&self.settings), &cmd);
            match (cmd.func)(args) {
                Ok(print_usage) => {
                    if print_usage {
                        let _ = msg.react(&ctx, ReactionType::from("✅"));
                    } else {
                        let _ = msg.react(&ctx, ReactionType::from("❌"));
                        let _ = msg.channel_id.send_message(&ctx, |eb| {
                            eb.content(format!("Invalid syntax! Try: ``{}{} {}``", self.settings.read().default_prefix, cmd.key, cmd.help_page));
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
        { self.safe.write().store(SERENITY_CACHE_SAFE_KEY, Arc::clone(&ctx.cache)); }

        let scheduler = Arc::clone(&self.scheduler);
        let mut scheduler = scheduler.write();
        scheduler.clear_all();
        scheduler.schedule_repeated(1 * 60 * 30, schedules::update_statistics); // EVERY 30 MINUTES
        scheduler.schedule_repeated(1200, schedules::fetch_memes); // EVERY 20 MINUTES
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_dogs); // EVERY 24 HOURS
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_birbs); // EVERY 24 HOURS
        scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_rabbits); // EVERY 24 HOURS
        scheduler.schedule_repeated(12 * 60 * 60, schedules::fetch_aww); // EVERY 12 HOURS
        scheduler.schedule_repeated(1 * 60 * 60, schedules::update_topgg); // EVERY 1 HOUR
        self.update_activity(&ctx);
        info!("Bot started!");
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

    // LOAD IMAGES
    let templates_path = Path::new("./templates/");
    let images = Arc::new(util::image::ImageStorage::load(templates_path).expect("could not create image storage"));

    // REGISTER COMMANDS
    let mut command_handler = CommandManager::new();
    {
        command_handler.register_command(commands::help::HELP_COMMAND.clone());
        command_handler.register_command(commands::animal::cat::CAT_COMMAND.clone());
        command_handler.register_command(commands::animal::dog::DOG_COMMAND.clone());
        command_handler.register_command(commands::animal::dog::DOG_BREEDS_COMMAND.clone());
        //command_handler.register_command(commands::animal::dog_cat_war::DOG_CAT_WAR_COMMAND.clone());
        command_handler.register_command(commands::meme::MEME_COMMAND.clone());
        command_handler.register_command(commands::about::ABOUT_COMMAND.clone());
        command_handler.register_command(commands::urban::URBAN_COMMAND.clone());
        command_handler.register_command(commands::animal::fox::FOX_COMMAND.clone());
        command_handler.register_command(commands::animal::birb::BIRB_COMMAND.clone());
        command_handler.register_command(commands::chuck::CHUCK_COMMAND.clone());
        command_handler.register_command(commands::urbanmug::URBANMUG_COMMAND.clone());
        command_handler.register_command(commands::animal::rabbit::RABBIT_COMMAND.clone());
        command_handler.register_command(commands::animal::aww::AWW_COMMAND.clone());

        commands::image_gen::command_gen::register_images(&mut command_handler, images.as_ref());

        for command in command_handler.get_all_commands().iter() {
            info!("Registered command: {}", command.key);
        }
    }

    // START CLIENT
    info!("Starting client");
    let mut client = Client::new(&discord_token, Handler::new(command_handler, images)).expect("Could not create Client");
    client.start_autosharded().expect("Could not start discord client");
}
