use windows::{
    Media::Control::{
        GlobalSystemMediaTransportControlsSession,
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionMediaProperties,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    },
};

#[derive(Debug, Clone)]
pub struct mediainfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub track_number: u32,
    pub genres: Vec<String>,
    pub playback_status: playbackstatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum playbackstatus {
    closed,
    opened,
    changing,
    stopped,
    playing,
    paused,
}

impl From<GlobalSystemMediaTransportControlsSessionPlaybackStatus> for playbackstatus {
    fn from(status: GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> Self {
        match status {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Closed => playbackstatus::closed,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Opened => playbackstatus::opened,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Changing => playbackstatus::changing,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped => playbackstatus::stopped,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => playbackstatus::playing,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => playbackstatus::paused,
            _ => playbackstatus::closed,
        }
    }
}

pub struct mediacontroller {
    manager: GlobalSystemMediaTransportControlsSessionManager,
}

impl mediacontroller {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
        Ok(Self { manager })
    }

    fn get_session(&self) -> Result<GlobalSystemMediaTransportControlsSession, Box<dyn std::error::Error>> {
        self.manager
            .GetCurrentSession()
            .map_err(|e| format!("no active media session found: {}", e).into())
    }

    pub async fn read(&self) -> Result<mediainfo, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let props: GlobalSystemMediaTransportControlsSessionMediaProperties = 
            session.TryGetMediaPropertiesAsync()?.await?;
        
        let playback_info = session.GetPlaybackInfo()?;
        let status = playback_info.PlaybackStatus()?;

        let genres = props.Genres()?;
        let mut genre_list = Vec::new();
        for i in 0..genres.Size()? {
            if let Ok(genre) = genres.GetAt(i) {
                genre_list.push(genre.to_string());
            }
        }

        Ok(mediainfo {
            title: props.Title()?.to_string(),
            artist: props.Artist()?.to_string(),
            album: props.AlbumTitle()?.to_string(),
            album_artist: props.AlbumArtist()?.to_string(),
            track_number: props.TrackNumber()? as u32,
            genres: genre_list,
            playback_status: status.into(),
        })
    }

    pub async fn play(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TryPlayAsync()?.await?;
        Ok(result)
    }

    pub async fn pause(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TryPauseAsync()?.await?;
        Ok(result)
    }

    pub async fn toggle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TryTogglePlayPauseAsync()?.await?;
        Ok(result)
    }

    pub async fn stop(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TryStopAsync()?.await?;
        Ok(result)
    }

    pub async fn next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TrySkipNextAsync()?.await?;
        Ok(result)
    }

    pub async fn previous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let result = session.TrySkipPreviousAsync()?.await?;
        Ok(result)
    }

    pub fn status(&self) -> Result<playbackstatus, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let playback_info = session.GetPlaybackInfo()?;
        let status = playback_info.PlaybackStatus()?;
        Ok(status.into())
    }

    pub fn is_playing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.status()? == playbackstatus::playing)
    }

    pub fn app_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let session = self.get_session()?;
        let source = session.SourceAppUserModelId()?;
        Ok(source.to_string())
    }
}

impl std::fmt::Display for playbackstatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            playbackstatus::closed => write!(f, "closed"),
            playbackstatus::opened => write!(f, "opened"),
            playbackstatus::changing => write!(f, "changing"),
            playbackstatus::stopped => write!(f, "stopped"),
            playbackstatus::playing => write!(f, "playing"),
            playbackstatus::paused => write!(f, "paused"),
        }
    }
}

impl std::fmt::Display for mediainfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} [{}]",
            self.artist,
            self.title,
            self.playback_status
        )
    }
}
