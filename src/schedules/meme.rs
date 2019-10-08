use crate::{commands, util};
use crate::scheduler::ScheduleArguments;

pub fn fetch_memes(args: ScheduleArguments) {
    let reddit_res = match util::reddit::fetch_reddit_images("https://www.reddit.com/user/zargornet/m/dcbot/.json?sort=top&t=day&limit=100") {
        Ok(k) => k,
        Err(e) => {
            error!("MEME SCHEDULER: could not fetch reddit memes: {}", e);
            return;
        }
    };

    let mut memes: Vec<commands::meme::Meme> = Vec::with_capacity(100);

    for meme in reddit_res.data.children.into_iter().map(|m| m.data) {
        let mut url = "https://reddit.com".to_owned();
        url.push_str(&meme.permalink);

        memes.push(commands::meme::Meme {
            title: meme.title,
            url,
            image: meme.url,
            author: meme.author,
            subreddit: meme.subreddit,
            upvotes: meme.ups,
        })
    }

    info!("MEME SCHEDULER: Fetched {} memes!", memes.len());

    let mut safe = args.safe.write();
    safe.store(commands::meme::MEME_CACHE_KEY, memes);
}