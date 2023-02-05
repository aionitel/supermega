use::clap::Parser;
use::colored::Colorize;
use std::time::Duration;
use std::thread::sleep;

// to print art
fn supermega() {
    let logo1 = r#"    
    .▄▄ · ▄• ▄▌ ▄▄▄·▄▄▄ .▄▄▄  
    ▐█ ▀. █▪██▌▐█ ▄█▀▄.▀·▀▄ █·
    ▄▀▀▀█▄█▌▐█▌ ██▀·▐▀▀▪▄▐▀▀▄ 
    ▐█▄▪▐█▐█▄█▌▐█▪·•▐█▄▄▌▐█•█▌
     ▀▀▀▀  ▀▀▀ .▀    ▀▀▀ .▀  ▀ "#;
    let logo2 = r#"    
    • ▌ ▄ ·. ▄▄▄ . ▄▄ •  ▄▄▄· 
    ·██ ▐███▪▀▄.▀·▐█ ▀ ▪▐█ ▀█ 
    ▐█ ▌▐▌▐█·▐▀▀▪▄▄█ ▀█▄▄█▀▀█ 
    ██ ██▌▐█▌▐█▄▄▌▐█▄▪▐█▐█▪ ▐▌
    ▀▀  █▪▀▀▀ ▀▀▀ ·▀▀▀▀  ▀  ▀ 
    "#;
    for line in logo1.lines() {
        println!("{}", line.bright_red());
        sleep(Duration::from_millis(15));
    }
    for line in logo2.lines() {
        println!("{}", line.bright_blue());
        sleep(Duration::from_millis(15));
    }
}

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
    )]
    video: String,

    /// Number of videos to return. (Default: 1)
    #[arg(
        help_heading = Some("Count"),
        short,
        long,
        value_name="COUNT",
    )]
    count: i32,

    /// List all possible video tags. (e.g. "Podcast" for a random podcast episode.)
    #[arg(
        help_heading = Some("List"),
        short,
        long,
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
    supermega();
    let Args { video, count, list } = validate_args();
    println!("Args: video: {:?}, count: {:?}, list: {:?}", video, count, list);
}