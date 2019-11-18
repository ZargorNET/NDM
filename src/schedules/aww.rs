use crate::{commands, util};
use crate::commands::animal::aww::Aww;
use crate::scheduler::ScheduleArguments;
use crate::util::safe::keys::commands::AWW_CACHE_KEY;

pub fn fetch_aww(args: ScheduleArguments) {
    let mut after: String = "".to_string();
    let mut ret: Vec<commands::animal::aww::Aww> = Vec::new();
    // Fetch 3 Sites of Reddit
    for _i in 0..3 {
        let reddit_res = match util::reddit::fetch_reddit_images(format!("https://www.reddit.com/r/aww/top/.json?sort=top&limit=100&t=day&after={}", after).as_str()) {
            Ok(k) => k,
            Err(e) => {
                error!("AWW SCHEDULER: could not fetch aww from reddit: {:#?}", e);
                return;
            }
        };

        for res in reddit_res.data.children {
            if res.data.post_hint != "image" {
                continue;
            }

            ret.push(Aww {
                url: res.data.url,
                permalink: res.data.permalink,
                title: res.data.title,
                author: res.data.author,
                like_ammount: res.data.ups,
                comments_ammount: res.data.num_comments,
            });
        }

        if let Some(result) = match reddit_res.data.after {
            Some(after) => Some(after),
            None => None
        } {
            after = result;
        } else {
            break
        }
    }
    ret.shrink_to_fit();
    info!("AWW SCHEDULER: Fetched {} awws", ret.len());
    let mut safe = args.safe.write();
    safe.store(AWW_CACHE_KEY, ret);
}