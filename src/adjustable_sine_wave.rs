use std::sync::{Arc, Mutex};
use rodio::Source;
use rodio::source::SineWave;

pub struct AdjustableSineWave {
    amplitude: Arc<Mutex<f32>>,
    sine_wave: SineWave,
}

impl AdjustableSineWave {
    pub fn new(frequency: f32, amplitude: Arc<Mutex<f32>>) -> Self {
        Self {
            amplitude,
            sine_wave: SineWave::new(frequency),
        }
    }
}

impl Source for AdjustableSineWave {
    fn current_frame_len(&self) -> Option<usize> {
        self.sine_wave.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.sine_wave.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sine_wave.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.sine_wave.total_duration()
    }
}

impl Iterator for AdjustableSineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let amplitude = *self.amplitude.lock().unwrap();
        self.sine_wave.next().map(|sample| sample * amplitude)
    }
}