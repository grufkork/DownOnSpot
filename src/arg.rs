use crate::settings;
use clap::{Args as ClapArgs, Command, FromArgMatches, Parser, crate_authors, crate_version};

#[derive(Parser, Debug)]
pub struct Args {
	#[arg(
		long_help = "Track / Album / Playlist / Artist / Podcast / Episode / Show / User URL, ID or search term\nFor example, \'Ariana Grande\', \'spotify:track:0KjAxsrYSvN0xGuh3cKPxD\', or \'https://open.spotify.com/playlist/37i9dQZF1DXcxvFzl58uP7\'"
	)]
	pub input: String,
}

impl Args {
	pub fn from_cli() -> Self {
		let cli = get_command();
		Self::from_arg_matches(&cli.get_matches()).unwrap()
	}
}

fn get_command() -> Command {
	let cli = Command::new(clap::crate_name!())
		.author(crate_authors!())
		.version(crate_version!())
		.about(format!(
			"Settings file located at: {}",
			settings::get_config_settings_path().to_string_lossy()
		));

	Args::augment_args(cli)
}
