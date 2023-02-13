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

// get video from simple queries like "letsplay"
pub async fn get_video(query: &str, count: i32) -> String {
    // get api key from .env and format url with query and count
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet&maxResults={}&q={}", api_key, channel_id, count, query);

    let data = req::get(url).await.expect("get text idfk").text().await.unwrap();
    data
}