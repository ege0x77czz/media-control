# media-control

a chill rust library for controlling media playback on windows via winrt.

grabs info from spotify, youtube, vlc, whatever you got playing. pause it, skip it, do your thing.

## installation

add this to your `Cargo.toml`:

```toml
[dependencies]
media-control = { git = "https://github.com/ege0x77czz/media-control" }
tokio = { version = "1", features = ["rt", "macros"] }
```

## usage

```rust
use media_control::mediacontroller;

#[tokio::main]
async fn main() {
    let controller = mediacontroller::new().await.unwrap();
    
    let info = controller.read().await.unwrap();
    println!("{} - {}", info.artist, info.title);
    
    controller.next().await.unwrap();
}
```

## api

### mediacontroller

**new()** - creates a new controller instance

**read()** - gets current media info (title, artist, album, etc.)

**play()** - resumes playback

**pause()** - pauses playback

**toggle()** - toggles play/pause

**stop()** - stops playback

**next()** - skips to next track

**previous()** - goes back to previous track

**status()** - returns current playback status

**is_playing()** - returns true if something is playing

**app_name()** - returns the app thats playing (spotify, chrome, etc.)

### mediainfo

```rust
pub struct mediainfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub track_number: u32,
    pub genres: Vec<String>,
    pub playback_status: playbackstatus,
}
```

### playbackstatus

```rust
pub enum playbackstatus {
    closed,
    opened,
    changing,
    stopped,
    playing,
    paused,
}
```

## cli example

theres a demo in the examples folder:

```bash
cargo run --example demo read
cargo run --example demo play
cargo run --example demo pause
cargo run --example demo next
cargo run --example demo prev
cargo run --example demo status
```

## requirements

- windows 10/11
- rust 1.70+

## license

mit

