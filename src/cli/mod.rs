use clap::Parser;
use dotenv;
use std::process;

mod data;
mod utils;
mod types;

#[derive(Parser, Debug)]
struct Args {
    /// Query for type of video. (e.g. "podcast" for a random podcast episode). Can supply more than one query argument.
    #[arg(
        help_heading = Some("Query"),
        short,
        long,
        value_name="QUERY",
        default_value = "any"
    )]
    query: String,

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

pub async fn run() {
    // init .env and draw art each time it runs
    dotenv::dotenv().ok();
    utils::draw();

    let Args { query, count, list }= Args::parse();

    // terminate program right after printing list
    // avoid running rest of program
    if list {
        utils::write();
        process::exit(0);
    }
}