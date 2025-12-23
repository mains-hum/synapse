use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Player {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Player {
            sink: Arc::new(Mutex::new(Some(sink))),
            _stream: stream,
            stream_handle,
        })
    }

    pub fn play(&self, file_path: &str, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        if let Ok(mut sink_guard) = self.sink.lock() {
            let new_sink = Sink::try_new(&self.stream_handle)?;
            new_sink.set_volume(volume);
            new_sink.append(source);
            new_sink.play();
            *sink_guard = Some(new_sink);
        }
        
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.set_volume(volume);
            }
        }
    }

    pub fn seek(&self, position: Duration) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                let _ = sink.try_seek(position);
            }
        }
    }

    pub fn pause(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.pause();
            }
        }
    }

    pub fn resume(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.play();
            }
        }
    }

    pub fn has_track(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                return !sink.empty();
            }
        }
        false
    }

    pub fn is_playing(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                return !sink.is_paused() && !sink.empty();
            }
        }
        false
    }
}
