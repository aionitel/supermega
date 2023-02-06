mod art;

use art::draw;
use::clap::Parser;
use::colored::Colorize;
use std::time::Duration;
use std::thread::sleep;


#[clap(
    name = "SuperMega",
    version = "0.1.0",
    author = "Alex I.",
    about = "SuperMega video picker",
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

    /// Number of videos to return. (Default: 1)
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

pub fn run() {
    // print nice logo before running clap
    draw();
    let Args { video, count, list } = validate_args();
    println!("{:?}", video);
}