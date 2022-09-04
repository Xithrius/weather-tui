use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{At, Word};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    handlers::{
        app::{App, State},
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
    })
    .await;

    let mut terminal = init_terminal();

    terminal.clear().unwrap();

    app.weather_data = Some(app.api.get_5_day_forecast().await);

    loop {
        terminal
            .draw(|frame| draw_ui(frame, &mut app, &config))
            .unwrap();

        if let Some(Event::Input(key)) = events.next().await {
            match app.state {
                State::Normal => match key {
                    Key::Char('i') => {
                        app.state = State::Insert;
                    }
                    Key::Ctrl('r') => {
                        app.weather_data = Some(app.api.get_5_day_forecast().await);
                    }
                    Key::Char('?') => {
                        app.state = State::Help;
                    }
                    Key::Ctrl('p') => {
                        panic!("Triggered on-purpose panic successfully.");
                    }
                    Key::Char('q') => {
                        quit_terminal(terminal);
                        break;
                    }
                    _ => {}
                },
                State::Insert => {
                    let input = &mut app.input_buffer;

                    match key {
                        Key::Char(c) => {
                            input.insert(c, 1);
                        }
                        Key::Ctrl('f') | Key::Right => {
                            input.move_forward(1);
                        }
                        Key::Ctrl('b') | Key::Left => {
                            input.move_backward(1);
                        }
                        Key::Ctrl('a') | Key::Home => {
                            input.move_home();
                        }
                        Key::Ctrl('e') | Key::End => {
                            input.move_end();
                        }
                        Key::Alt('f') => {
                            input.move_to_next_word(At::AfterEnd, Word::Emacs, 1);
                        }
                        Key::Alt('b') => {
                            input.move_to_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('t') => {
                            input.transpose_chars();
                        }
                        Key::Alt('t') => {
                            input.transpose_words(1);
                        }
                        Key::Ctrl('u') => {
                            input.discard_line();
                        }
                        Key::Ctrl('k') => {
                            input.kill_line();
                        }
                        Key::Ctrl('w') => {
                            input.delete_prev_word(Word::Emacs, 1);
                        }
                        Key::Ctrl('d') => {
                            input.delete(1);
                        }
                        Key::Backspace | Key::Delete => {
                            input.backspace(1);
                        }
                        Key::Esc => {
                            app.state = State::Normal;
                        }
                        _ => {}
                    }
                }
                State::Help => match key {
                    Key::Char('q') => {
                        quit_terminal(terminal);
                        break;
                    }
                    Key::Esc => {
                        app.state = State::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    reset_terminal();
}
