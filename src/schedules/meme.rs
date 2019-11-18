use crate::{commands, util};
use crate::scheduler::ScheduleArguments;

pub fn fetch_memes(args: ScheduleArguments) {
    let mut memes: Vec<commands::fun::meme::Meme> = Vec::new();
    let mut after: String = "".to_string();
    // Fetch 3 Sites of Reddit
    for _i in 0..3 {
        let reddit_res = match util::reddit::fetch_reddit_images(format!("https://www.reddit.com/user/turulix/m/notdankmemer/.json?sort=top&t=day&limit=100&after={}", after).as_str()) {
            Ok(k) => k,
            Err(e) => {
                error!("MEME SCHEDULER: could not fetch reddit memes: {}", e);
                return;
            }
        };

        for meme in reddit_res.data.children.into_iter().map(|m| m.data) {
            let mut url = "https://reddit.com".to_owned();
            url.push_str(&meme.permalink);

            memes.push(commands::fun::meme::Meme {
                title: meme.title,
                url,
                image: meme.url,
                author: meme.author,
                subreddit: meme.subreddit,
                upvotes: meme.ups,
            })
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

    memes.shrink_to_fit();
    info!("MEME SCHEDULER: Fetched {} memes!", memes.len());
    let mut safe = args.safe.write();
    safe.store(commands::fun::meme::MEME_CACHE_KEY, memes);
}