use crate::{commands, util};
use crate::commands::animal::rabbit::Rabbit;
use crate::scheduler::ScheduleArguments;
use crate::util::safe::keys::commands::RABBIT_CACHE_KEY;

pub fn fetch_rabbits(args: ScheduleArguments) {
    let mut ret: Vec<commands::animal::rabbit::Rabbit> = Vec::new();
    let mut after: String = "".to_string();
    for _i in 0..3 {
        let rabbit_res = match util::reddit::fetch_reddit_images(format!("https://www.reddit.com/r/Rabbits/top/.json?sort=top&t=day&limit=100&after={}", after).as_str()) {
            Ok(k) => k,
            Err(e) => {
                error!("RABBIT SCHEDULER: could not fetch rabbits from reddit: {:#?}", e);
                return;
            }
        };

        for res in rabbit_res.data.children {
            if res.data.post_hint != "image" {
                continue;
            }

            ret.push(Rabbit {
                title: res.data.title,
                url: res.data.url,
            });
        }
        if let Some(result) = match rabbit_res.data.after {
            Some(after) => Some(after),
            None => None
        } {
            after = result;
        } else {
            break
        }
    }

    ret.shrink_to_fit();

    info!("RABBIT SCHEDULER: Successfully fetched {} rabbits!", ret.len());

    let mut safe = args.safe.write();
    safe.store(RABBIT_CACHE_KEY, ret);
}