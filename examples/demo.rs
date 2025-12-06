use media_control::mediacontroller;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("read");

    let controller = match mediacontroller::new().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to init media controller: {}", e);
            return;
        }
    };

    match command {
        "read" => {
            match controller.read().await {
                Ok(info) => {
                    println!("now playing:");
                    println!("  title:        {}", info.title);
                    println!("  artist:       {}", info.artist);
                    println!("  album:        {}", info.album);
                    println!("  album artist: {}", info.album_artist);
                    println!("  track:        {}", info.track_number);
                    println!("  status:       {}", info.playback_status);
                    if !info.genres.is_empty() {
                        println!("  genres:       {}", info.genres.join(", "));
                    }
                }
                Err(e) => eprintln!("failed to read media info: {}", e),
            }
        }
        "play" => {
            match controller.play().await {
                Ok(true) => println!("resumed playback"),
                Ok(false) => println!("failed to resume"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "pause" => {
            match controller.pause().await {
                Ok(true) => println!("paused playback"),
                Ok(false) => println!("failed to pause"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "toggle" => {
            match controller.toggle().await {
                Ok(true) => println!("toggled playback"),
                Ok(false) => println!("failed to toggle"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "stop" => {
            match controller.stop().await {
                Ok(true) => println!("stopped playback"),
                Ok(false) => println!("failed to stop"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "next" | "skip" => {
            match controller.next().await {
                Ok(true) => println!("skipped to next track"),
                Ok(false) => println!("failed to skip"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "prev" | "previous" => {
            match controller.previous().await {
                Ok(true) => println!("went back to previous track"),
                Ok(false) => println!("failed to go back"),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "status" => {
            match controller.status() {
                Ok(status) => println!("status: {}", status),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "app" => {
            match controller.app_name() {
                Ok(name) => println!("playing from: {}", name),
                Err(e) => eprintln!("error: {}", e),
            }
        }
        "thumbnail" | "cover" | "art" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("cover.png");
            match controller.save_thumbnail(path).await {
                Ok(_) => println!("saved cover art to {}", path),
                Err(e) => eprintln!("failed to save cover: {}", e),
            }
        }
        _ => {
            println!("media-control - control your tunes from the terminal");
            println!();
            println!("usage: demo <command> [args]");
            println!();
            println!("commands:");
            println!("  read            show whats currently playing (default)");
            println!("  play            resume playback");
            println!("  pause           pause playback");
            println!("  toggle          toggle play/pause");
            println!("  stop            stop playback");
            println!("  next            skip to next track");
            println!("  prev            go to previous track");
            println!("  status          show playback status");
            println!("  app             show which app is playing");
            println!("  thumbnail [path] save cover art (default: cover.png)");
        }
    }
}

