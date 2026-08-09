<div align="center">

# DownOnSpot

<br>
</div>

## ⚒️ Building

1. Clone the repository using git and change to the local repository directory:

   ```bash
   git clone https://github.com/oSumAtrIX/DownOnSpot.git
   cd DownOnSpot
   ```

2. Install dependencies

   If you are on Linux, make sure you have the `libasound2-dev` package installed.  
   Additionally, you need the [libmp3lame](https://www.rarewares.org/mp3-lame-libraries.php#libmp3lame) library.  
   On Mac OS, run `brew install lame`, provided you have [Homebrew](https://brew.sh/) installed.

3. Build

   ```bash
   cargo build --release
   ```

## 🕹️ Usage

2. Run DownOnSpot

   ```bash
   $ ./down_on_spot
   Settings could not be loaded because of the following error: IO: NotFound No such file or directory. (os error 2)...
   ..but default settings have been created successfully. Edit them and run the program again.
   ```

3. Edit the `settings.json` file

   The `settings.json` file is located in the following directories:

   - Windows: `C:\Users\<user>\AppData\Roaming\down_on_spot\settings.json`
   - Unix: `~/.config/down_on_spot/settings.json`

   The `client_id` and `client_secret` can be obtained by creating an app at [spotify for developers](https://developer.spotify.com).

   You can obtain your `access_token` by following [this guide](https://developer.spotify.com/documentation/web-api/tutorials/getting-started#request-an-access-token).
   Or [the webplayer](https://open.spotify.com) also uses an access token (with a longer ttl), which you can view by inspecting the network traffic via your browser dev tools, when logged in.

🎉 Now you can use DownOnSpot

   ```bash
   $ ./down_on_spot
   Usage:
   down_on_spot.exe <search_term> | <track_url> | <album_url> | <playlist_url> | <artist_url>
   ```

### ⚙️ Template variables

You can use the following template variables for `path` and `filename_template` in the `settings.json` file:

- %0disc%
- %0track%
- %album%
- %albumArtist%
- %albumArtists%
- %artist%
- %disc%
- %id%
- %title%
- %track%

## 🧭 Additional scripts

- [Userscript to download titles from YouTube](https://gist.github.com/oSumAtrIX/6abf46e2ea25d32f4e6608c3c3cf837e)

## 🐞 Known issues

- Slow MP3 downloads due to libmp3lame
- Sporadic `channel error` when downloading tracks

## 🔑 License

DownOnSpot is licensed under the GPLv3 licence. Please see the [licence file](LICENSE) for more information.
[tl;dr](https://www.tldrlegal.com/license/gnu-general-public-license-v3-gpl-3) you may copy, distribute and modify DownOnSpot as long as you track changes/dates in source files.
Any modifications to DownOnSpot must also be made available under the GPL, along with build & install instructions.
