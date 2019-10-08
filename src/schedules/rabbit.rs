use crate::{commands, util};
use crate::commands::animal::rabbit::Rabbit;
use crate::scheduler::ScheduleArguments;

pub fn fetch_rabbits(args: ScheduleArguments) {
    let rabbit_res = match util::reddit::fetch_reddit_images("https://www.reddit.com/r/Rabbits/top/.json?sort=top&t=day&limit=100") {
        Ok(k) => k,
        Err(e) => {
            error!("RABBIT SCHEDULER: could not fetch rabbits from reddit: {:#?}", e);
            return;
        }
    };

    let mut ret: Vec<commands::animal::rabbit::Rabbit> = Vec::with_capacity(rabbit_res.data.children.len());

    for res in rabbit_res.data.children {
        if res.data.post_hint != "image" {
            continue;
        }

        ret.push(Rabbit {
            title: res.data.title,
            url: res.data.url,
        });
    }

    ret.shrink_to_fit();

    info!("RABBIT SCHEDULER: Successfully fetched {} rabbits!", ret.len());

    let mut safe = args.safe.write();
    safe.store(commands::animal::rabbit::RABBIT_CACHE_KEY, ret);
}