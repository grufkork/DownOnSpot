use base64::Engine;
use base64::engine::general_purpose;
use chrono::{Datelike, NaiveDate};
use oggvorbismeta::{CommentHeader, VorbisComments, read_comment_header, replace_comment_header};
use std::fs::File;
use std::path::{Path, PathBuf};

use super::Field;
use crate::error::SpotifyError;

pub struct OggTag {
	path: PathBuf,
	tag: CommentHeader,
}

impl OggTag {
	/// Load tag from file
	pub fn open(path: impl AsRef<Path>) -> Result<OggTag, SpotifyError> {
		let mut file = File::open(&path)?;
		let tag = read_comment_header(&mut file).map_err(|e| SpotifyError::Error(e.to_string()))?;
		Ok(OggTag {
			path: path.as_ref().to_owned(),
			tag,
		})
	}
}

impl super::Tag for OggTag {
	fn set_separator(&mut self, _separator: &str) {}

	fn set_field(&mut self, field: Field, value: Vec<String>) {
		let tag = match field {
			Field::Title => "TITLE",
			Field::Artist => "ARTIST",
			Field::Album => "ALBUM",
			Field::TrackNumber => "TRACKNUMBER",
			Field::DiscNumber => "DISCNUMBER",
			Field::Genre => "GENRE",
			Field::Label => "LABEL",
			Field::AlbumArtist => "ALBUMARTIST",
		};
		self.set_raw(tag, value);
	}

	fn add_cover(&mut self, mime: &str, data: Vec<u8>) {
		let mut picture: Vec<u8> = Vec::new();

		// MIME type
		picture.extend(3u32.to_be_bytes().iter());
		picture.extend((mime.len() as u32).to_be_bytes().iter());
		picture.extend(mime.as_bytes());

		// Description
		picture.extend(0u32.to_be_bytes().iter());

		// Width, height, depth, and number of colors
		picture.extend(0u32.to_be_bytes().iter());
		picture.extend(0u32.to_be_bytes().iter());
		picture.extend(0u32.to_be_bytes().iter());
		picture.extend(0u32.to_be_bytes().iter());

		// Image data
		picture.extend((data.len() as u32).to_be_bytes().iter());
		picture.extend(data);

		self.tag.add_tag_single(
			"METADATA_BLOCK_PICTURE",
			general_purpose::STANDARD.encode(picture),
		);
	}

	fn set_raw(&mut self, tag: &str, value: Vec<String>) {
		self.tag.add_tag_multi(
			tag,
			&value.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
		);
	}

	fn save(&mut self) -> Result<(), SpotifyError> {
		let file = File::open(&self.path)?;
		let mut out = replace_comment_header(file, &self.tag.clone())
			.map_err(|e| SpotifyError::Error(e.to_string()))?;
		let mut file = File::create(&self.path)?;
		std::io::copy(&mut out, &mut file)?;
		Ok(())
	}

	fn set_release_date(&mut self, date: NaiveDate) {
		self.tag.add_tag_single(
			"DATE",
			format!("{}-{:02}-{:02}", date.year(), date.month(), date.day()),
		)
	}

	fn add_unique_file_identifier(&mut self, track_id: &str) {
		self.tag.add_tag_single("SPOTIFY_TRACKID", track_id);
	}
}
