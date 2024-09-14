use std::path::Path;

use crate::downloader::AudioFormat;
use crate::error::SpotifyError;

use self::id3::ID3Tag;
use ogg::OggTag;

mod id3;
mod ogg;

pub enum TagWrap {
	Ogg(OggTag),
	Id3(ID3Tag),
}

impl TagWrap {
	/// Load from file
	pub fn new(path: impl AsRef<Path>, format: AudioFormat) -> Result<TagWrap, SpotifyError> {
		match format {
			AudioFormat::Ogg => Ok(TagWrap::Ogg(OggTag::open(path)?)),
			AudioFormat::Mp3 => Ok(TagWrap::Id3(ID3Tag::open(path)?)),
			_ => Err(SpotifyError::Error("Invalid format!".into())),
		}
	}

	/// Get Tag trait
	pub fn get_tag(&mut self) -> &mut dyn Tag {
		match self {
			TagWrap::Ogg(tag) => tag,
			TagWrap::Id3(tag) => tag,
		}
	}
}

pub trait Tag {
	/// Set tag values separator
	fn set_separator(&mut self, separator: &str);
	fn set_raw(&mut self, tag: &str, value: Vec<String>);
	fn set_field(&mut self, field: Field, value: Vec<String>);
	fn set_release_date(&mut self, date: String);
	fn add_cover(&mut self, mime: &str, data: Vec<u8>);
	/// Adds the file identifier of the track
	fn add_unique_file_identifier(&mut self, track_id: &str);
	fn save(&mut self) -> Result<(), SpotifyError>;
}

#[derive(Debug, Clone)]
pub enum Field {
	Title,
	Artist,
	Album,
	TrackNumber,
	DiscNumber,
	AlbumArtist,
	Genre,
	Label,
}
