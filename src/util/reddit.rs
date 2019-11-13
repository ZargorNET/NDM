use std::error;

pub fn fetch_reddit_images(url: &str) -> Result<RedditResponse, Box<dyn error::Error>> {
    let mut res = reqwest::get(url)?;
    let text = res.text()?;
    let reddit_res: RedditResponse = serde_json::from_str(&text)?;

    Ok(reddit_res)
}


#[derive(Serialize, Deserialize)]
pub struct RedditResponse {
    pub data: RedditResponseData,
}

#[derive(Serialize, Deserialize)]
pub struct RedditResponseData {
    pub children: Vec<RedditResponseChildren>,
    pub after: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct RedditResponseChildren {
    pub kind: String,
    pub data: RedditResponseChildrenData,
}

#[derive(Serialize, Deserialize)]
pub struct RedditResponseChildrenData {
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit: String,
    pub title: String,
    pub author: String,
    pub ups: u32,
    pub permalink: String,
    pub url: String,
    #[serde(default)]
    pub post_hint: String,
    pub num_comments: u32
}
