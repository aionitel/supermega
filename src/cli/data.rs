use std::env;
use std::fmt::Error;
use reqwest as req;

#[derive(Debug)]
struct Video {
    id: String,
    title: String,
    desc: String,
    thumbnail: String,
    published_at: String,
    video_id: String,
}

// SuperMega youtube channel_id
const channel_id: &str = "UCPPc2PdtA7gCMbjYp_i_TKA";

pub async fn get_video(query: &str, count: i32) -> Result<String, Error> {
    // get api key from .env and format url with query and count
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet&maxResults=10&q=plays", api_key, channel_id);

    let data = req::get(url).await.expect("get text idfk").text().await.unwrap();
}