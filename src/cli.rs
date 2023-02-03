use::clap::Parser;
use::colored::Colorize;

// to display art
fn supermega() {
    let logo1 = r#"    
    /\  _`\ /\ \/\ \/\  _`\ /\  _`\ /\  _`\     
    \ \,\L\_\ \ \ \ \ \ \L\ \ \ \L\_\ \ \L\ \   
     \/_\__ \\ \ \ \ \ \ ,__/\ \  _\L\ \ ,  /   
       /\ \L\ \ \ \_\ \ \ \/  \ \ \L\ \ \ \\ \  
       \ `\____\ \_____\ \_\   \ \____/\ \_\ \_\
        \/_____/\/_____/\/_/    \/___/  \/_/\/ /
    "#;
    let logo2 = r#"    
    /'\_/`\/\  _`\ /\  _`\ /\  _  \            
    /\      \ \ \L\_\ \ \L\_\ \ \L\ \           
    \ \ \__\ \ \  _\L\ \ \L_L\ \  __ \          
     \ \ \_/\ \ \ \L\ \ \ \/, \ \ \/\ \         
      \ \_\\ \_\ \____/\ \____/\ \_\ \_\        
       \/_/ \/_/\/___/  \/___/  \/_/\/_/    
    "#;
    print!("{}", logo1.bright_red());
    println!("{}", logo2.bright_blue());
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
        required = false,
    )]
    video: String,

    /// Number of videos to return. (Default: 1)
    #[arg(
        help_heading = Some("Count"),
        short,
        long,
        value_name="COUNT",
        required = false
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

fn validate_args() -> Args  {
    let args = Args::parse();
    args
}

pub fn run() {
    let args = validate_args();
    println!("{:?}", args);
}