use crate::downloader::DownloaderConfig;
use crate::error::SpotifyError;
use aspotify::CountryCode;
use serde::{Deserialize, Serialize};

use tokio::{
	fs::File,
	fs::create_dir_all,
	io::{AsyncReadExt, AsyncWriteExt},
};

use std::{
	env,
	path::{Path, PathBuf},
};

// Structure for holding all the settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub access_token: String,
	pub client_id: String,
	pub client_secret: String,
	pub refresh_ui_seconds: u64,
	pub downloader: DownloaderConfig,
	pub market_country_code: Option<CountryCode>,
}

// On UNIX systems (eg. Linux, *BSD, even macOS), follow the
// XDG Base Directory Specification for storing config files
#[cfg(target_family = "unix")]
fn get_config_folder_path() -> PathBuf {
	match env::var("XDG_CONFIG_HOME") {
		Ok(v) => Path::new(&v).join("down_on_spot").to_path_buf(),
		Err(_) => Path::new(&env::var("HOME").unwrap()).join(".config/down_on_spot"),
	}
}

// On Windows, follow whatever windows does for AppData
#[cfg(target_family = "windows")]
fn get_config_folder_path() -> PathBuf {
	Path::new(&env::var("APPDATA").unwrap()).join("down_on_spot")
}

/// Returns the full path to the Settings json
///
/// Windows: `%APPDATA%\down_on_spot\settings.json`
///
/// Unix-like: `~/.config/down_on_spot/settings.json`
pub fn get_config_settings_path() -> PathBuf {
	get_config_folder_path().join("settings.json")
}

impl Settings {
	// Create new instance
	pub fn new(access_token: &str, client_id: &str, client_secret: &str) -> Settings {
		Settings {
			access_token: access_token.to_string(),
			client_id: client_id.to_string(),
			client_secret: client_secret.to_string(),
			refresh_ui_seconds: 1,
			downloader: DownloaderConfig::new(),
			market_country_code: None,
		}
	}

	/// Save config
	///
	/// Returns the path of the written file if successful
	pub async fn save(&self) -> Result<PathBuf, SpotifyError> {
		// Get and create config folder path, generate config file path
		let config_file_path = get_config_settings_path();
		let config_folder_path = config_file_path
			.parent()
			.expect("Configuration file path should have a parent component");
		create_dir_all(&config_folder_path).await?;

		// Serialize the settings to a json file
		let data = serde_json::to_string_pretty(self)?;
		let mut file = File::create(&config_file_path).await?;
		file.write_all(data.as_bytes()).await?;
		Ok(config_file_path)
	}

	// Load config
	pub async fn load() -> Result<Settings, SpotifyError> {
		// Get config folder path, generate config file path
		let config_file_path = get_config_settings_path();

		// Deserialize the settings from a json file
		let mut file = File::open(config_file_path).await?;
		let mut buf = String::new();
		file.read_to_string(&mut buf).await?;
		Ok(serde_json::from_str(&buf)?)
	}
}
