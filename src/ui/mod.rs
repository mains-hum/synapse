pub mod app;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(6),
        ])
        .split(f.size());

    let list_height = chunks[1].height.saturating_sub(2) as usize;
    app.update_scroll(list_height);

    let current_song = app.songs.get(app.current_track).map(|s| s.as_str()).unwrap_or("No track");
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" synapse", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  |  "),
        Span::styled(current_song, Style::default().fg(Color::White)),
    ])).block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let visible_songs: Vec<ListItem> = app.songs.iter()
        .enumerate()
        .skip(app.scroll_offset)
        .take(list_height)
        .map(|(i, song)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if i == app.current_track {
                Style::default().fg(Color::Green)
            } else { Style::default() };
            ListItem::new(format!("{:>3}. {}", i + 1, song)).style(style)
        }).collect();

    let list = List::new(visible_songs)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("󰲸 library ({}/{}) ", app.selected_index + 1, app.songs.len())));
    f.render_widget(list, chunks[1]);

    let elapsed = format!("{}:{:02}", app.elapsed_time.as_secs() / 60, app.elapsed_time.as_secs() % 60);
    let total = format!("{}:{:02}", app.current_song_duration.as_secs() / 60, app.current_song_duration.as_secs() % 60);
    
    let vol_color = if app.volume > 100 { Color::Red } else { Color::Blue };
    
    let volume_text = format!("volume: {}%", app.volume);

    let text = vec![
        Line::from(vec![
            Span::styled(if app.is_playing { " playing" } else { " paused" }, Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::styled(format!("{}", app.play_mode.icon()), Style::default().fg(Color::Magenta)),
            Span::raw(" | "),
            Span::styled(volume_text, Style::default().fg(vol_color)),
        ]),
        Line::from(vec![Span::styled(format!("time: {} / {}", elapsed, total), Style::default().fg(Color::White))]),
        Line::from(vec![Span::styled("j/k: navigation | enter: select/play | space: play/stop | n/b: next/back", Style::default().fg(Color::Gray))]),
    Line::from(vec![Span::styled("h/l: ±5 sec | m: change mode | ↑/↓: volume", Style::default().fg(Color::Gray))]),

    ];
    f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" 󰌳 player ")), chunks[2]);
}
