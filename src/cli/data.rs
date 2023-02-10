use std::env;
use reqwest::Client;

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

pub async fn get_video() -> Result<(), reqwest::Error> {
    // init reqwest client and get youtube api key from env
    let client = Client::new();
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");

    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet&maxResults=10&q=plays", api_key, channel_id);

    let res = client.get(url).send().await.expect("failed to get response").json().await.expect("idk");
    println!("{:?}", res);
    
    Ok(())
}