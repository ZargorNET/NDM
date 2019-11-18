use std::collections::HashMap;
use std::sync::Arc;

use reqwest::StatusCode;
use serenity::cache::Cache;
use serenity::prelude::RwLock;

use crate::scheduler::ScheduleArguments;
use crate::util::safe::keys::other::SERENITY_CACHE_KEY;

pub fn update_topgg(args: ScheduleArguments) {
    let http_client = reqwest::Client::new();
    let cache;
    {
        cache = match args.safe.read().get::<Arc<RwLock<Cache>>>(SERENITY_CACHE_KEY) {
            Some(s) => s.clone(),
            None => {
                error!("TOPGG_UPDATE SCHEDULER: could not get serenity cache");
                return;
            }
        };
    } // DROP LOCK
    let cache = cache.read();
    use std::env::var;
    let topgg_token = match var("TOP_GG_TOKEN") {
        Ok(k) => k,
        Err(e) => {
            error!("TOPGG_UPDATE SCHEDULER: could not get \"TOP_GG_TOKEN\": {}", e);
            return;
        }
    };
    let mut map = HashMap::new();
    map.insert("server_count", cache.all_guilds().len());
    map.insert("shard_count", cache.shard_count as usize);
    let mut res = match http_client.post(format!("https://top.gg/api/bots/{}/stats", cache.user.id.0).as_str()).json(&map).header("Authorization", topgg_token).send() {
        Ok(k) => k,
        Err(e) => {
            error!("TOPGG_UPDATE SCHEDULER: Could not update top.gg stats: {}", e);
            return;
        }
    };
    let res_text = match res.text() {
        Ok(k) => k,
        Err(e) => {
            error!("TOPGG_UPDATE SCHEDULER: Could not read body of TopGGResponse: {}", e);
            return;
        }
    };
    let _gg_response: TopGGResponse = match serde_json::from_str(&res_text) {
        Ok(k) => k,
        Err(e) => {
            error!("TOP_GG_UPDATE SCHEDULER: Could not parse body of TopGGResponse: {}", e);
            return;
        }
    };

    match res.status() {
        StatusCode::UNAUTHORIZED => {
            error!("TOPGG_UPDATE SCHEDULER: Unauthorized to update stats check token!");
            return;
        }
        StatusCode::BAD_REQUEST => {
            error!("TOPGG_UPDATE SCHEDULER: Bad Request.");
            return;
        }
        StatusCode::OK => {
            info!("TOPGG_UPDATE SCHEDULER: Successfully updated top.gg stats.")
        }
        StatusCode::FORBIDDEN => {
            error!("TOPGG_UPDATE SCHEDULER: Forbidden! Does this bot exist on top.gg?");
            return;
        }
        _ => {
            warn!("TOPGG_UPDATE SCHEDULER: Return status: {}", res.status().as_str())
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TopGGResponse {
    error: Option<String>
}