use std::env;

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

pub async fn get_video() {
    let api_key = env::var("API_KEY").expect("API_KEY must be set.");
}