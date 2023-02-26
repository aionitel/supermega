use clap::Parser;
use dotenv;
use std::process;

mod utils;
mod data;

#[clap(
    name = "SuperMega",
    version = "0.1.0",
    author = "Alex I.",
)]
#[derive(Parser, Debug)]
struct Args {
    /// Kind of of video. (e.g. "podcast" for a random podcast episode). Can supply more than one arguments.
    #[arg(
        help_heading = Some("Video"),
        short,
        long,
        value_name="VIDEO",
        default_value = "any"
    )]
    video: String,

    /// Number of videos to return.
    #[arg(
        help_heading = Some("Count"),
        short,
        long,
        value_name="COUNT",
        default_value = "1"
    )]
    count: i32,

    /// List all possible video tags. (e.g. "Podcast" for a random podcast episode, default is false)
    #[arg(
        help_heading = Some("List"),
        short,
        long,
        default_value = "false"
    )]
    list: bool
}

fn validate_args() -> Args {
    if let args = Args::parse() {
        return args;
    }
    panic!("Problem parsing arguments.");
}

pub async fn run() {
    // init .env and draw art each time it runs
    dotenv::dotenv().ok();
    utils::draw();

    let Args { video, count, list }= validate_args();

    // terminate program right after printing list
    // avoid running rest of program
    if list {
        utils::write();
        process::exit(0);
    }
}