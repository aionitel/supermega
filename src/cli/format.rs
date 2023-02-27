use crate::cli::types::Video;

pub fn print_videos(videos: Vec<Video>) {
    println!("    ▼▼▼ You videos sire ▼▼▼\n");
    for video in videos {
        println!("{:}", video.snippet.title);
        println!("WATCH ➡➡➡ https://youtube.com/watch/?v={:}\n", video.id.video_id);
    }
}