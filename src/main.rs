use::colored::Colorize;
use::clap::Parser;

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

#[derive(Parser, Debug)]
struct Args {
    // main category (e.g. "Justin")
    #[arg(short, long)]
    video: String,

    // number of videos to return
    #[arg(short, long, default_value_t = 1)]
    count: i32,
}

fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args);
}