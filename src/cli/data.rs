use std::env;
use reqwest as req;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: Id,
    pub snippet: Snippet
}

#[derive(Debug, Deserialize)]
pub struct Id {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub pub_at: String,
    pub title: String,
    #[serde(rename = "description")]
    pub desc: String,
}

// SuperMega youtube channel_id
const CHANNEL_ID: &str = "UCPPc2PdtA7gCMbjYp_i_TKA";

// get video from simple queries
pub async fn get_video(query: String, count: i32) -> Vec<Video> {
    // get api key from .env and format url with query and count
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet&maxResults={}&q={}", api_key, CHANNEL_ID, count, query);

    // send get request, parse and unwrap response as json
    let res = req::get(url).await;
    let data: Value = res.expect("msg").json().await.unwrap();
    let items = &data["items"];
    let json_data: Vec<Video> = serde_json::from_value(items.clone()).unwrap();

    // lol
    json_data
}