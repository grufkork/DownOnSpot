use futures::{pin_mut, TryStreamExt};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use rspotify::clients::BaseClient;
use rspotify::model::{
    AlbumId, ArtistId, Country, FullAlbum, FullArtist, FullPlaylist, FullTrack, IncludeExternal,
    Market, PlayableItem, PlaylistId, SearchResult, SearchType, SimplifiedAlbum, SimplifiedTrack,
    TrackId,
};
use rspotify::{ClientCredsSpotify, Credentials as ClientCredentials};
use std::fmt;
use std::path::Path;
use url::Url;

use crate::error::SpotifyError;

pub struct Spotify {
    // librespot session
    pub session: Session,
    pub spotify: ClientCredsSpotify,
    pub market: Option<Market>,
}

impl Spotify {
    /// Create new instance
    pub async fn new(
        username: &str,
        password: &str,
        client_id: &str,
        client_secret: &str,
        market_country_code: Option<Country>,
    ) -> Result<Spotify, SpotifyError> {
        // librespot
        let cache = Cache::new(Some(Path::new("credentials_cache")), None, None, None).unwrap();
        let credentials = match cache.credentials() {
            Some(creds) => creds,
            None => Credentials::with_password(username, password),
        };

        let session = Session::new(SessionConfig::default(), Some(cache));
        session.connect(credentials, true).await?;

        // rspotify
        let credentials = ClientCredentials {
            id: client_id.to_string(),
            secret: Some(client_secret.to_string()),
        };
        let spotify = ClientCredsSpotify::new(credentials);
        spotify.request_token().await?;

        Ok(Spotify {
            session,
            spotify,
            market: market_country_code.map(Market::Country),
        })
    }

    /// Parse URI or URL into URI
    pub fn parse_uri(uri: &str) -> Result<String, SpotifyError> {
        // Already URI
        if uri.starts_with("spotify:") {
            if uri.split(':').count() < 3 {
                return Err(SpotifyError::InvalidUri);
            }
            return Ok(uri.to_string());
        }

        // Parse URL
        let url = Url::parse(uri)?;
        // Spotify Web Player URL
        if url.host_str() == Some("open.spotify.com") {
            let path = url
                .path_segments()
                .ok_or_else(|| SpotifyError::Error("Missing URL path".into()))?
                .collect::<Vec<&str>>();
            if path.len() < 2 {
                return Err(SpotifyError::InvalidUri);
            }
            return Ok(format!("spotify:{}:{}", path[0], path[1]));
        }
        Err(SpotifyError::InvalidUri)
    }

    /// Fetch data for URI
    pub async fn resolve_uri(&self, uri: &str) -> Result<SpotifyItem, SpotifyError> {
        let parts = uri.split(':').skip(1).collect::<Vec<&str>>();
        let id = parts[1];
        match parts[0] {
            "track" => {
                let track = self
                    .spotify
                    .track(TrackId::from_id(id)?, self.market)
                    .await?;
                Ok(SpotifyItem::Track(track))
            }
            "playlist" => {
                let playlist = self
                    .spotify
                    .playlist(PlaylistId::from_id(id)?, None, self.market)
                    .await?;
                Ok(SpotifyItem::Playlist(playlist))
            }
            "album" => {
                let album = self
                    .spotify
                    .album(AlbumId::from_id(id)?, self.market)
                    .await?;
                Ok(SpotifyItem::Album(album))
            }
            "artist" => {
                let artist = self.spotify.artist(ArtistId::from_id(id)?).await?;
                Ok(SpotifyItem::Artist(artist))
            }
            // Unsupported / Unimplemented
            _ => Ok(SpotifyItem::Other(uri.to_string())),
        }
    }

    /// Get search results for query
    pub async fn search(&self, query: &str) -> Result<Vec<FullTrack>, SpotifyError> {
        Ok(self
            .spotify
            .search(
                query,
                SearchType::Track,
                None,
                Some(IncludeExternal::Audio),
                Some(50),
                Some(0),
            )
            .await
            .map(|result| match result {
                SearchResult::Tracks(page) => page.items,
                _ => Vec::new(),
            })
            .unwrap())
    }

    /// Get all tracks from playlist
    pub async fn full_playlist(&self, id: PlaylistId<'_>) -> Result<Vec<FullTrack>, SpotifyError> {
        let mut tracks: Vec<FullTrack> = Vec::new();
        let stream = self.spotify.playlist_items(id, None, self.market);

        pin_mut!(stream);
        while let Some(item) = stream.try_next().await.unwrap() {
            match item.track.unwrap() {
                PlayableItem::Track(t) => tracks.push(t),
                _ => continue,
            }
        }

        Ok(tracks)
    }

    /// Get all tracks from album
    pub async fn full_album(&self, id: AlbumId<'_>) -> Result<Vec<SimplifiedTrack>, SpotifyError> {
        let mut tracks: Vec<SimplifiedTrack> = Vec::new();
        let stream = self.spotify.album_track(id, self.market);

        pin_mut!(stream);
        while let Some(item) = stream.try_next().await.unwrap() {
            tracks.push(item)
        }

        Ok(tracks)
    }

    /// Get all tracks from artist
    pub async fn full_artist(
        &self,
        id: ArtistId<'_>,
    ) -> Result<Vec<SimplifiedTrack>, SpotifyError> {
        let mut albums: Vec<SimplifiedAlbum> = Vec::new();
        let mut tracks: Vec<SimplifiedTrack> = Vec::new();
        let stream = self.spotify.artist_albums(id, None, self.market);

        pin_mut!(stream);
        while let Some(item) = stream.try_next().await.unwrap() {
            albums.push(item)
        }

        for album in albums {
            tracks.append(&mut self.full_album(album.id.unwrap()).await.unwrap())
        }

        Ok(tracks)
    }
}

impl Clone for Spotify {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            spotify: ClientCredsSpotify::new(self.spotify.creds.clone()),
            market: self.market,
        }
    }
}

/// Basic debug implementation so can be used in other structs
impl fmt::Debug for Spotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Spotify Instance>")
    }
}

#[derive(Debug, Clone)]
pub enum SpotifyItem {
    Track(FullTrack),
    Album(FullAlbum),
    Playlist(FullPlaylist),
    Artist(FullArtist),
    /// Unimplemented
    Other(String),
}
