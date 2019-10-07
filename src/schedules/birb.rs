use std::collections::HashSet;
use std::iter::FromIterator;

use crate::scheduler::ScheduleArguments;

pub fn fetch_birbs(args: ScheduleArguments) {
    let mut glob_birbs: HashSet<String> = HashSet::new();

    let http_client = reqwest::Client::new();
    for _ in 0..5 {
        let mut res = match http_client.execute(http_client.get("http://shibe.online/api/birds?count=100").build().unwrap()) {
            Ok(k) => k,
            Err(e) => {
                error!("could not fetch new birbs: {}", e);
                return;
            }
        };

        let text = match res.text() {
            Ok(k) => k,
            Err(e) => {
                error!("could not read body of birb service: {}", e);
                return;
            }
        };

        let birbs: Vec<String> = match serde_json::from_str(&text) {
            Ok(k) => k,
            Err(e) => {
                error!("could not parse json of birb service: {}", e);
                return;
            }
        };

        for birb in birbs {
            glob_birbs.replace(birb);
        }
    }

    let amount = glob_birbs.len();
    let glob_vec: Vec<String> = glob_birbs.into_iter().collect();

    let mut safe = args.safe.write();
    safe.store(crate::commands::animal::birb::BIRB_CACHE_KEY, glob_vec);
    info!("Fetched {} birbs!", amount);
}