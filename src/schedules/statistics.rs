use std::sync::Arc;

use serenity::cache::Cache;
use serenity::prelude::RwLock;

use crate::commands::about::{Statistics, STATISTICS_CACHE_KEY};
use crate::scheduler::ScheduleArguments;
use crate::SERENITY_CACHE_SAFE_KEY;

pub fn update_statistics(args: ScheduleArguments) {
    let cache;

    {
        cache = match args.safe.read().get::<Arc<RwLock<Cache>>>(SERENITY_CACHE_SAFE_KEY) {
            Some(s) => s.clone(),
            None => {
                error!("STATISTIC SCHEDULER: could not get serenity cache");
                return;
            }
        };
    } // DROP LOCK

    let cache = cache.read();

    let servers = cache.guilds.len();
    let users = cache.users.len();
    let mut text_channels = 0u32;
    let mut voice_channels = 0u32;
    for c in cache.channels.iter().map(|c| c.1.read()) {
        match c.kind {
            serenity::model::channel::ChannelType::Text => text_channels += 1,
            serenity::model::channel::ChannelType::Voice => voice_channels += 1,
            _ => {}
        }
    }

    let statistics = Statistics {
        num_text_channels: text_channels,
        num_voice_channels: voice_channels,
        num_servers: servers as u32,
        num_users: users as u32,
    };

    args.safe.write().store(STATISTICS_CACHE_KEY, statistics);

    info!("STATISTIC SCHEDULER: Successfully updated statistics!");
}