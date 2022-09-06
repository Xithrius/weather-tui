# Weather in the terminal

[![CI](https://github.com/Xithrius/weather-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/Xithrius/weather-tui/actions/workflows/ci.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/Xithrius/weather-tui/main.svg)](https://results.pre-commit.ci/latest/github/Xithrius/weather-tui/main)
[![Lines of code](https://tokei.rs/b1/github/Xithrius/weather-tui?category=code)](https://github.com/Xithrius/weather-tui)
[![Crate Status](https://img.shields.io/crates/v/weather-tui.svg)](https://crates.io/crates/weather-tui)

![image](https://user-images.githubusercontent.com/15021300/188537621-bd22eb66-0239-4af1-a3db-8147423983df.png)

## Setup

1. Make sure [rustup](https://www.rust-lang.org/tools/install) is installed, and `~/.cargo/bin` is in your `PATH` environment variable.
2. Install `weather-tui` through `cargo install weather-tui`.
3. Generate an API key from [openweathermap](https://home.openweathermap.org/api_keys). What you name the key will not affect this application.
4.  Run `wt`, and the application will run first-time setup, generating a config at the following locations across different operating systems:
        - `Linux/MacOS`: `~/.config/twt/config.toml`
        - `Windows`: `%appdata%\twt\config.toml`
5. At said config locations, fill in `api_key` with your api key from step (3), and `area` with the location you'd like to grab data from, such as London.

If you have any problems, do not hesitate to [submit an issue](https://github.com/Xithrius/weather-tui/issues/new/choose).
