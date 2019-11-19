use std::sync::Arc;

use crate::commands::about::Statistics;
use crate::scheduler::ScheduleArguments;
use crate::util::safe::keys::commands::STATISTICS_CACHE_KEY;

pub fn update_statistics(args: ScheduleArguments) {
    let cache = Arc::clone(&args.serenity.cache);

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