use std::fs;
use crate::player::Player;
use std::time::{Duration, Instant};
use lofty::{AudioFile, Probe};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayMode {
    Normal, Shuffle, Repeat, RepeatOne,
}

impl PlayMode {
    pub fn next(&self) -> Self {
        match self {
            PlayMode::Normal => PlayMode::Shuffle,
            PlayMode::Shuffle => PlayMode::Repeat,
            PlayMode::Repeat => PlayMode::RepeatOne,
            PlayMode::RepeatOne => PlayMode::Normal,
        }
    }
    pub fn icon(&self) -> &str {
        match self {
            PlayMode::Normal => "→",
            PlayMode::Shuffle => "🔀",
            PlayMode::Repeat => "🔁",
            PlayMode::RepeatOne => "🔁1",
        }
    }
}

pub struct App {
    pub music_path: String,
    pub songs: Vec<String>,
    pub playlist: Vec<usize>,
    pub current_track: usize,
    pub playlist_position: usize,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub is_playing: bool,
    pub play_mode: PlayMode,
    pub current_song_duration: Duration,
    pub elapsed_time: Duration,
    pub last_update: Instant,
    pub volume: u32,
    player: Option<Player>,
}

impl App {
    pub fn new(music_path: String) -> Self {
        let songs = Self::scan_music_directory(&music_path);
        let player = Player::new().ok();
        let song_count = songs.len();
        
        App {
            music_path,
            songs,
            playlist: (0..song_count).collect(),
            current_track: 0,
            playlist_position: 0,
            selected_index: 0,
            scroll_offset: 0,
            is_playing: false,
            play_mode: PlayMode::Normal,
            current_song_duration: Duration::from_secs(0),
            elapsed_time: Duration::from_secs(0),
            last_update: Instant::now(),
            volume: 100,
            player,
        }
    }

    fn scan_music_directory(path: &str) -> Vec<String> {
        let mut songs = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let s_ext = ext.to_string_lossy().to_lowercase();
                    if ["mp3", "flac", "ogg", "wav"].contains(&s_ext.as_str()) {
                        if let Some(name) = entry.file_name().to_str() {
                            songs.push(name.to_string());
                        }
                    }
                }
            }
        }
        songs.sort();
        songs
    }

    pub fn update_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 { return; }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    pub fn toggle_play_mode(&mut self) {
        self.play_mode = self.play_mode.next();
        match self.play_mode {
            PlayMode::Shuffle => {
                let mut rng = thread_rng();
                self.playlist.shuffle(&mut rng);
                if let Some(pos) = self.playlist.iter().position(|&idx| idx == self.current_track) {
                    self.playlist_position = pos;
                }
            },
            _ => {
                self.playlist.sort();
                if let Some(pos) = self.playlist.iter().position(|&idx| idx == self.current_track) {
                    self.playlist_position = pos;
                }
            }
        }
    }

    pub fn volume_up(&mut self) {
        self.volume = (self.volume + 5).min(200);
        if let Some(ref player) = self.player { player.set_volume(self.volume as f32 / 100.0); }
    }

    pub fn volume_down(&mut self) {
        self.volume = self.volume.saturating_sub(5);
        if let Some(ref player) = self.player { player.set_volume(self.volume as f32 / 100.0); }
    }

    pub fn toggle_playback(&mut self) {
        if let Some(ref player) = self.player {
            if self.is_playing {
                player.pause();
                self.is_playing = false;
            } else {
                if player.has_track() { player.resume(); } else { self.play_current(); }
                self.is_playing = true;
                self.last_update = Instant::now();
            }
        }
    }

    pub fn play_current(&mut self) {
        if let Some(ref player) = self.player {
            if let Some(name) = self.songs.get(self.current_track) {
                let path = format!("{}/{}", self.music_path, name);
                if let Ok(probed) = Probe::open(&path).and_then(|p| p.read()) {
                    self.current_song_duration = probed.properties().duration();
                }
                if player.play(&path, self.volume as f32 / 100.0).is_ok() {
                    self.elapsed_time = Duration::from_secs(0);
                    self.last_update = Instant::now();
                    self.is_playing = true;
                }
            }
        }
    }

    pub fn next_track(&mut self) {
        if self.play_mode == PlayMode::RepeatOne { 
            self.play_current(); 
            return; 
        }
        if self.playlist_position < self.playlist.len() - 1 {
            self.playlist_position += 1;
        } else {
            self.playlist_position = 0;
        }
        self.current_track = self.playlist[self.playlist_position];
        self.selected_index = self.current_track; 
        self.play_current();
    }

    pub fn previous_track(&mut self) {
        if self.playlist_position > 0 {
            self.playlist_position -= 1;
        } else {
            self.playlist_position = self.playlist.len() - 1;
        }
        self.current_track = self.playlist[self.playlist_position];
        self.selected_index = self.current_track; 
        self.play_current();
    }

    pub fn seek_forward(&mut self) {
        if let Some(ref player) = self.player {
            let new_time = self.elapsed_time + Duration::from_secs(5);
            if new_time < self.current_song_duration {
                self.elapsed_time = new_time;
                player.seek(self.elapsed_time);
            } else { self.next_track(); }
        }
    }

    pub fn seek_backward(&mut self) {
        if let Some(ref player) = self.player {
            self.elapsed_time = self.elapsed_time.saturating_sub(Duration::from_secs(5));
            player.seek(self.elapsed_time);
        }
    }

    pub fn update_time(&mut self) {
        if self.is_playing {
            let now = Instant::now();
            self.elapsed_time += now.duration_since(self.last_update);
            self.last_update = now;
        }
    }

    pub fn next_song_in_list(&mut self) { if self.selected_index < self.songs.len() - 1 { self.selected_index += 1; } }
    pub fn previous_song_in_list(&mut self) { if self.selected_index > 0 { self.selected_index -= 1; } }
    pub fn play_selected(&mut self) { 
        self.current_track = self.selected_index;
        if let Some(pos) = self.playlist.iter().position(|&idx| idx == self.current_track) {
            self.playlist_position = pos;
        }
        self.play_current(); 
    }
    pub fn check_track_finished(&mut self) {
        if self.is_playing && self.elapsed_time >= self.current_song_duration { self.next_track(); }
    }
}
