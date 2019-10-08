use crate::{commands, util};
use crate::commands::animal::aww::Aww;
use crate::scheduler::ScheduleArguments;

pub fn fetch_aww(args: ScheduleArguments) {
    let reddit_res = match util::reddit::fetch_reddit_images("https://www.reddit.com/r/aww/top/.json?sort=top&limit=100&t=day") {
        Ok(k) => k,
        Err(e) => {
            error!("AWW SCHEDULER: could not fetch aww from reddit: {:#?}", e);
            return;
        }
    };

    let mut ret: Vec<commands::animal::aww::Aww> = Vec::with_capacity(reddit_res.data.children.len());

    for res in reddit_res.data.children {
        if res.data.post_hint != "image" {
            continue;
        }

        ret.push(Aww {
            url: res.data.url,
            permalink: res.data.permalink,
            title: res.data.title,
            author: res.data.author,
        });
    }

    ret.shrink_to_fit();

    info!("AWW SCHEDULER: Fetched {} awws", ret.len());
    let mut safe = args.safe.write();
    safe.store(commands::animal::aww::AWW_CACHE_KEY, ret);
}