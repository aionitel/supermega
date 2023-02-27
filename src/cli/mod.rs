use clap::Parser;
use dotenv;

mod data;
mod utils;
mod types;
mod format;

#[derive(Parser, Debug)]
struct Args {
    /// Query for type of video. (e.g. "podcast" for a random podcast episode). Can supply more than one query argument.
    #[arg(
        help_heading = Some("Prompt"),
        short,
        long,
        value_name="PROMPT",
        default_value = "any"
    )]
    prompt: String,

    /// Number of videos to return.
    #[arg(
        help_heading = Some("Count"),
        short,
        long,
        value_name="COUNT",
        default_value = "1"
    )]
    count: i32
}

pub async fn run() {
    // init .env and draw art each time it runs
    dotenv::dotenv().ok();
    utils::draw();

    let Args { prompt, count }= Args::parse();

    // get videos and print them nicely
    let videos = data::get_video(prompt, count).await;
    format::print_videos(videos);
}