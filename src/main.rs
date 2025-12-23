mod ui;
mod player;

use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let music_path = args.get(1).cloned().unwrap_or_else(|| ".".to_string());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new(music_path);
    
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;
        app.update_time();
        app.check_track_finished();

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => app.next_song_in_list(),
                    KeyCode::Char('k') => app.previous_song_in_list(),
                    KeyCode::Char('h') => app.seek_backward(),
                    KeyCode::Char('l') => app.seek_forward(),
                    KeyCode::Char(' ') => app.toggle_playback(),
                    KeyCode::Up => app.volume_up(),
                    KeyCode::Down => app.volume_down(),
                    KeyCode::Char('m') => app.toggle_play_mode(),
                    KeyCode::Char('n') => app.next_track(),     // Переход на следующий трек
                    KeyCode::Char('b') => app.previous_track(), // Переход на предыдущий трек
                    KeyCode::Enter => app.play_selected(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
