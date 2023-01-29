use::clap::Parser;

#[clap(author, version, about)]
#[derive(Parser, Debug)]
pub struct Args {
    /// Kind of of video. (e.g. "podcast" for a random podcast episode). Can supply more than one arguments.
    #[arg(short, long)]
    kind: String,

    /// Number of videos to return. (Default: 1)
    #[arg(short, long, default_value_t = 1)]
    count: i32,
}