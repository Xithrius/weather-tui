use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    handlers::{
        app::App,
        config::CompleteConfig,
        event::{Config, Event, Events, Key},
    },
    ui::draw_ui,
};

fn reset_terminal() {
    disable_raw_mode().unwrap();

    execute!(stdout(), LeaveAlternateScreen).unwrap();
}

fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend).unwrap()
}

fn quit_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) {
    disable_raw_mode().unwrap();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();
}

pub async fn ui_driver(config: CompleteConfig, mut app: App) {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal();
        original_hook(panic);
    }));

    let mut events = Events::with_config(Config {
        exit_key: Key::Null,
        tick_rate: Duration::from_millis(config.terminal.tick_delay),
    });

    let mut terminal = init_terminal();

    terminal.clear().unwrap();

    app.weather_data = Some(app.api.get_5_day_forecast().await);

    loop {
        terminal.draw(|f| draw_ui(f, &app, &config)).unwrap();

        if let Some(Event::Input(key)) = events.next().await {
            match key {
                // Quit the application
                Key::Esc | Key::Char('q') => {
                    quit_terminal(terminal);
                    break;
                }

                // Refresh the data, doing another API call
                Key::Ctrl('r') => {
                    app.weather_data = Some(app.api.get_5_day_forecast().await);
                }

                // Manual panic
                Key::Ctrl('p') => {
                    panic!("Triggered on-purpose panic successfully.");
                }
                _ => {}
            }
        }
    }

    reset_terminal();
}
