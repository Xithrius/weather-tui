# weather-tui
Weather in the terminal.


## Setup

1. Make sure [rustup](https://www.rust-lang.org/tools/install) is installed, and `~/.cargo/bin` is in your `PATH` environment variable.
2. Install `weather-tui` through `cargo install weather-tui`.
3. Generate an API key from [openweathermap](https://home.openweathermap.org/api_keys). What you name the key will not affect this application.
4. You now have two options for running `weather-tui`:
    - Run `wt --no-config --key aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` to run. **NOTE:** Be aware that the key will appear in your [history](https://www.man7.org/linux/man-pages/man3/history.3.html) (unless disabled).
    - Run `wt`, and the application will run first-time setup, generating a config at the following locations across different operating systems:
        - `Linux/MacOS`: `~/.config/twt/config.toml`
        - `Windows`: `%appdata%\twt\config.toml`

If you have any problems, do not hesitate to [submit an issue](https://github.com/Xithrius/weather-tui/issues/new/choose).
