use lewton::inside_ogg::OggStreamReader;
use std::io::{Error, ErrorKind, Read, Seek};

use crate::downloader::{AudioFormat, Quality};
use crate::error::SpotifyError;
use crate::error::SpotifyError::{InvalidFormat, LameConverterError};

/// Converts audio to MP3
pub enum AudioConverter {
	Ogg {
		decoder: OggStreamReader<ReadWrap>,
		lame: lame::Lame,
		lame_end: bool,
	},
}

unsafe impl Send for AudioConverter {}

impl AudioConverter {
	/// Wrap reader
	pub fn new(
		read: Box<dyn Read + Send + 'static>,
		format: AudioFormat,
		quality: Quality,
	) -> Result<AudioConverter, SpotifyError> {
		// Create encoder
		let bitrate = match quality {
			Quality::Q320 => 320,
			Quality::Q256 => 256,
			Quality::Q160 => 160,
			Quality::Q96 => 96,
		};

		let mut lame = lame::Lame::new().unwrap();

		match lame.set_channels(2) {
			Ok(_) => {}
			Err(_) => return Err(LameConverterError("Channels".to_string())),
		};

		match lame.set_quality(0) {
			Ok(_) => {}
			Err(_) => return Err(LameConverterError("Quality".to_string())),
		};
		match lame.set_kilobitrate(bitrate) {
			Ok(_) => {}
			Err(_) => return Err(LameConverterError("Bitrate".to_string())),
		};

		match format {
			AudioFormat::Aac => todo!(),
			// Lewton decoder
			AudioFormat::Ogg => {
				let decoder = OggStreamReader::new(ReadWrap::new(Box::new(read)))?;
				let sample_rate = decoder.ident_hdr.audio_sample_rate;
				// Init lame
				match lame.set_sample_rate(sample_rate) {
					Ok(_) => {}
					Err(_) => return Err(LameConverterError("Sample rate".to_string())),
				};
				match lame.init_params() {
					Ok(_) => {}
					Err(_) => return Err(LameConverterError("Init".to_string())),
				};

				Ok(AudioConverter::Ogg {
					lame,
					decoder,
					lame_end: false,
				})
			}
			AudioFormat::Mp3 => panic!("No reencoding allowd!"),
			_ => Err(InvalidFormat),
		}
	}
}

impl Read for AudioConverter {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match self {
			AudioConverter::Ogg {
				decoder,
				lame,
				lame_end,
			} => {
				match decoder.read_dec_packet() {
					Ok(packet) => match packet {
						Some(data) => {
							// 0 sized packets aren't EOF
							if data[0].is_empty() {
								return self.read(buf);
							}

							let result = match lame.encode(&data[0], &data[1], buf) {
								Ok(size) => {
									if size == 0 {
										return self.read(buf);
									}
									size
								}
								Err(e) => {
									return Err(Error::new(
										ErrorKind::InvalidData,
										format!("Lame error: {e:?}"),
									));
								}
							};
							Ok(result as usize)
						}
						None => {
							if *lame_end {
								return Ok(0);
							}
							*lame_end = true;
							Ok(0)
						}
					},
					Err(e) => {
						// Close lame
						if !*lame_end {
							*lame_end = true;
						}
						warn!("Lawton error: {e}, calling EOF");
						Ok(0)
					}
				}
			}
		}
	}
}

pub struct ReadWrap {
	source: Box<dyn Read + Send + 'static>,
}

impl ReadWrap {
	pub fn new(read: Box<dyn Read + Send + 'static>) -> ReadWrap {
		ReadWrap {
			source: Box::new(read),
		}
	}
}

impl Read for ReadWrap {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.source.read(buf)
	}
}

/// Fake seek for Rodio
impl Seek for ReadWrap {
	fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
		Ok(0)
	}
}
