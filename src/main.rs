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
use crate::scheduler::Scheduler;
use crate::util::safe::Safe;

mod util;
mod scheduler;
#[macro_use]
mod command_framework;
mod commands;
mod schedules;

pub struct StaticSettings {
    pub default_prefix: String,
    pub start_time: DateTime<Utc>,
}

struct Handler {
    ch: Arc<RwLock<CommandManager>>,
    safe: Arc<RwLock<Safe>>,
    image: Arc<util::image::ImageStorage>,
    settings: Arc<StaticSettings>,
}

impl Handler {
    fn new(ch: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, image: Arc<util::image::ImageStorage>) -> Handler {
        let settings = Arc::new(StaticSettings {
            default_prefix: "+".to_string(),
            start_time: Utc::now(),
        });

        Handler {
            ch,
            safe,
            image,
            settings,
        }
    }
}

impl Handler {
    fn update_activity(&self, ctx: &Context) {
        ctx.set_activity(Activity::playing(&format!("on {} servers! | {}help", ctx.cache.read().all_guilds().len(), self.settings.default_prefix)));
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
        if !msg.content.starts_with(&self.settings.default_prefix) {
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
                            eb.content(format!("Invalid syntax! Try: ``{}{} {}``", self.settings.default_prefix, cmd.key, cmd.help_page));
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
        self.update_activity(&ctx);
        info!("Shard {} started!", ctx.shard_id);
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
        command_handler.register_command(commands::fun::meme::MEME_COMMAND.clone());
        command_handler.register_command(commands::about::ABOUT_COMMAND.clone());
        command_handler.register_command(commands::fun::urban::URBAN_COMMAND.clone());
        command_handler.register_command(commands::animal::fox::FOX_COMMAND.clone());
        command_handler.register_command(commands::animal::birb::BIRB_COMMAND.clone());
        command_handler.register_command(commands::fun::chuck::CHUCK_COMMAND.clone());
        command_handler.register_command(commands::fun::urbanmug::URBANMUG_COMMAND.clone());
        command_handler.register_command(commands::animal::rabbit::RABBIT_COMMAND.clone());
        command_handler.register_command(commands::animal::aww::AWW_COMMAND.clone());
        command_handler.register_command(commands::fun::love::LOVE_COMMAND.clone());
        command_handler.register_command(commands::make::MAKE_COMMAND.clone());

        commands::image_gen::command_gen::register_images(&mut command_handler, images.as_ref());

        for command in command_handler.get_all_commands().iter() {
            info!("Registered command: {}", command.key);
        }
    }
    let command_handler = Arc::new(RwLock::new(command_handler));
    let safe = Arc::new(RwLock::new(Safe::new()));

    let handler = Handler::new(Arc::clone(&command_handler), Arc::clone(&safe), images);

    // START CLIENT
    info!("Starting client");
    let mut client = Client::new(&discord_token, handler).expect("Could not create Client");

    let scheduler = Scheduler::new(Arc::clone(&command_handler), Arc::clone(&safe), Arc::clone(&client.cache_and_http));
    start_scheduler(&scheduler);

    client.start_shards(2).expect("Could not start discord client");
}

fn start_scheduler(scheduler: &Scheduler) {
    scheduler.clear_all();
    scheduler.schedule_repeated(1 * 60 * 30, schedules::update_statistics); // EVERY 30 MINUTES
    scheduler.schedule_repeated(1200, schedules::fetch_memes); // EVERY 20 MINUTES
    scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_dogs); // EVERY 24 HOURS
    scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_birbs); // EVERY 24 HOURS
    scheduler.schedule_repeated(24 * 60 * 60, schedules::fetch_rabbits); // EVERY 24 HOURS
    scheduler.schedule_repeated(12 * 60 * 60, schedules::fetch_aww); // EVERY 12 HOURS
    scheduler.schedule_repeated(1 * 60 * 60, schedules::update_topgg); // EVERY 1 HOUR
}