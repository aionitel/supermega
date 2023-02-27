use serde::Deserialize;

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