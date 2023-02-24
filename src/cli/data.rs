use std::env;
use reqwest as req;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Video {
    id: String,
    title: String,
    desc: String,
    thumbnail: String,
    published_at: String,
}

// SuperMega youtube channel_id
const CHANNEL_ID: &str = "UCPPc2PdtA7gCMbjYp_i_TKA";

// get video from simple queries like "letsplay"
pub async fn get_video(query: &str, count: i32) {
    // get api key from .env and format url with query and count
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet&maxResults={}&q={}", api_key, CHANNEL_ID, count, query);

    // send get request, parse and unwrap response as json
    let res = req::get(url).await;
    let data: Value = res.expect("idfk").json().await.unwrap();
    let items = &data["items"];
    println!("{:?}", items);
}