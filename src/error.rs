use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotifyError {
    Error(String),
    IoError(std::io::ErrorKind, String),
    MercuryError,
    AuthenticationError,
    Unavailable,
    SpotifyIdError,
    ChannelError,
    AudioKeyError,
    LameConverterError(String),
    JoinError,
    RSpotify(String),
    Serde(String, usize, usize),
    InvalidUri,
    ParseError(url::ParseError),
    ID3Error(String, String),
    Reqwest(String),
    InvalidFormat,
    NotConnected,
    UnknownPacket(u8),
    AlreadyDownloaded,
}

impl std::error::Error for SpotifyError {}
impl fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpotifyError::Error(e) => write!(f, "Error: {}", e),
            SpotifyError::MercuryError => write!(f, "Mercury Error"),
            SpotifyError::IoError(kind, err) => write!(f, "IO: {:?} {}", kind, err),
            SpotifyError::AuthenticationError => write!(f, "Authentication Error"),
            SpotifyError::Unavailable => write!(f, "Unavailable!"),
            SpotifyError::SpotifyIdError => write!(f, "Invalid Spotify ID"),
            SpotifyError::ChannelError => write!(f, "Channel Error"),
            SpotifyError::AudioKeyError => write!(f, "Audio Key Error"),
            SpotifyError::LameConverterError(e) => write!(f, "Lame error: {}", e),
            SpotifyError::JoinError => write!(f, "Tokio Join Error"),
            SpotifyError::RSpotify(e) => write!(f, "Spotify Error: {}", e),
            SpotifyError::Serde(e, l, c) => write!(f, "Serde Error @{}:{} {}", l, c, e),
            SpotifyError::InvalidUri => write!(f, "Invalid URI"),
            SpotifyError::ParseError(e) => write!(f, "Parse Error: {}", e),
            SpotifyError::ID3Error(k, e) => write!(f, "ID3 Error: {} {}", k, e),
            SpotifyError::Reqwest(e) => write!(f, "Reqwest Error: {}", e),
            SpotifyError::InvalidFormat => write!(f, "Invalid Format!"),
            SpotifyError::NotConnected => write!(f, "Not Connected"),
            SpotifyError::UnknownPacket(e) => write!(f, "Unknown Packet: {}", e),
            SpotifyError::AlreadyDownloaded => write!(f, "Already Downloaded"),
        }
    }
}
impl From<std::io::Error> for SpotifyError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.kind(), e.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for SpotifyError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::Error(e.to_string())
    }
}

impl From<librespot::core::mercury::MercuryError> for SpotifyError {
    fn from(_: librespot::core::mercury::MercuryError) -> Self {
        Self::MercuryError
    }
}

impl From<librespot::core::error::Error> for SpotifyError {
    fn from(e: librespot::core::error::Error) -> Self {
        SpotifyError::Error(e.to_string())
    }
}

impl From<librespot::core::session::SessionError> for SpotifyError {
    fn from(e: librespot::core::session::SessionError) -> Self {
        match e {
            librespot::core::session::SessionError::IoError(e) => e.into(),
            librespot::core::session::SessionError::AuthenticationError(_) => {
                SpotifyError::AuthenticationError
            }
            librespot::core::session::SessionError::NotConnected => SpotifyError::NotConnected,
            librespot::core::session::SessionError::Packet(e) => SpotifyError::UnknownPacket(e),
        }
    }
}

impl From<librespot::core::spotify_id::SpotifyIdError> for SpotifyError {
    fn from(_: librespot::core::spotify_id::SpotifyIdError) -> Self {
        Self::SpotifyIdError
    }
}

impl From<librespot::core::channel::ChannelError> for SpotifyError {
    fn from(_: librespot::core::channel::ChannelError) -> Self {
        Self::ChannelError
    }
}

impl From<librespot::core::audio_key::AudioKeyError> for SpotifyError {
    fn from(_: librespot::core::audio_key::AudioKeyError) -> Self {
        Self::AudioKeyError
    }
}

impl From<tokio::task::JoinError> for SpotifyError {
    fn from(_: tokio::task::JoinError) -> Self {
        Self::JoinError
    }
}

impl From<rspotify::ClientError> for SpotifyError {
    fn from(e: rspotify::ClientError) -> Self {
        Self::RSpotify(e.to_string())
    }
}

impl From<serde_json::Error> for SpotifyError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e.to_string(), e.line(), e.column())
    }
}

impl From<url::ParseError> for SpotifyError {
    fn from(e: url::ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<id3::Error> for SpotifyError {
    fn from(e: id3::Error) -> Self {
        Self::ID3Error(e.kind.to_string(), e.description.to_string())
    }
}

impl From<reqwest::Error> for SpotifyError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}

impl From<lewton::VorbisError> for SpotifyError {
    fn from(e: lewton::VorbisError) -> Self {
        SpotifyError::Error(format!("Lewton: {}", e))
    }
}

impl From<rspotify::model::IdError> for SpotifyError {
    fn from(e: rspotify::model::IdError) -> Self {
        Self::Error(format!("{}", e))
    }
}
