use std::sync::{Arc, Mutex};
use rodio::{OutputStream, Sink, Source};

use crate::adjustable_sine_wave::AdjustableSineWave;

pub struct Signal {
    amplitude: Arc<Mutex<f32>>,
    _sink : Arc<Mutex<Sink>>,                    // Needed to keep the sink alive
    _stream : OutputStream,                      // Needed to keep the stream alive
    _stream_handle : rodio::OutputStreamHandle,  // Needed to keep the stream alive
}

impl Signal {
    pub fn new(frequency: f32, amplitude: f32) -> Self {

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Shared amplitude value
        let amplitude = Arc::new(Mutex::new(amplitude));

        // Create a sine wave source
        let source = AdjustableSineWave::new(frequency, Arc::clone(&amplitude));
        println!("Sample rate: {}", source.sample_rate());
        sink.append(source);
        
        Self {
            amplitude,
            _sink : Arc::new(Mutex::new(sink)),
            _stream : _stream,
            _stream_handle : stream_handle,
        }
    }

    pub fn set_amplitude(&self, amplitude: f32) {
        let mut amp = self.amplitude.lock().unwrap();
        *amp = amplitude;
        println!("Amplitude changed to: {}", *amp);
    }
}
