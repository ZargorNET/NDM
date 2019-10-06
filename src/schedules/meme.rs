use crate::commands;
use crate::scheduler::ScheduleArguments;

#[derive(Serialize, Deserialize)]
struct RedditResponse {
    data: RedditResponseData,
}

#[derive(Serialize, Deserialize)]
struct RedditResponseData {
    children: Vec<RedditResponseChildren>,
}

#[derive(Serialize, Deserialize)]
struct RedditResponseChildren {
    kind: String,
    data: RedditResponseChildrenData,
}

#[derive(Serialize, Deserialize)]
struct RedditResponseChildrenData {
    #[serde(rename = "subreddit_name_prefixed")]
    subreddit: String,
    title: String,
    author: String,
    ups: i32,
    permalink: String,
    url: String,
}

pub fn fetch_memes(args: ScheduleArguments) {
    let mut res = match reqwest::get("https://www.reddit.com/user/zargornet/m/dcbot/.json?sort=top&t=day&limit=100") {
        Ok(r) => r,
        Err(e) => {
            error!("Could not get reddit memes: {}", e);
            return;
        }
    };
    let text = match res.text() {
        Ok(t) => t,
        Err(e) => {
            error!("Could not get reddit meme's body: {}", e);
            return;
        }
    };

    let reddit_res: RedditResponse = match serde_json::from_str(&text) {
        Ok(t) => t,
        Err(e) => {
            error!("Could not parse reddit meme's body {}", e);
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

    info!("Fetched {} memes!", memes.len());

    let mut safe = args.safe.write();
    safe.store(commands::meme::MEME_CACHE_KEY, memes);
}