use::clap::Parser;

#[clap(
    name = "SuperMega",
    version = "0.1.0",
    author = "Alex I.",
    about = "SuperMega video picker"
)]
#[derive(Parser, Debug)]
pub struct Args {
    /// Kind of of video. (e.g. "podcast" for a random podcast episode). Can supply more than one arguments.
    #[arg(short, long)]
    kind: String,

    /// Number of videos to return. (Default: 1)
    #[arg(short, long, default_value_t = 1)]
    count: i32,
}

pub fn run() {
    let args = Args::parse();
    println!("Args: {:?}", args);
}