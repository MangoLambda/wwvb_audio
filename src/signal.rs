use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

const MINIMUM_SOUND_BUFFER_SIZE: usize = 2;
const CARRIER_FILE_NAME: &str = "./400Hz_2s.ogg";

pub struct Signal {
    sink: Sink,                                // Needed to keep the sink alive
    _stream: OutputStream,                     // Needed to keep the stream alive
    _stream_handle: rodio::OutputStreamHandle, // Needed to keep the stream alive
}

impl Signal {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = File::open(CARRIER_FILE_NAME).unwrap();
        let file_reader = BufReader::new(file);
        let source = Decoder::new(file_reader).unwrap();
        println!("Sample rate: {}", source.sample_rate());
        sink.append(source);

        Self {
            sink,
            _stream: _stream,
            _stream_handle: stream_handle,
        }
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.sink.set_volume(amplitude);
    }

    pub fn restart(&mut self) {
        println!("Restart");
        let file = File::open(CARRIER_FILE_NAME).unwrap();
        let file_reader = BufReader::new(file);
        let source = Decoder::new(file_reader).unwrap();
        self.sink.append(source);
    }

    // TODO: make async?
    pub fn restart_if_is_ending_soon_or_if_ended(&mut self) {
        if self.is_ending_soon_or_ended() {
            self.restart();
        }
    }

    fn is_ending_soon_or_ended(&self) -> bool {
        self.sink.len() < MINIMUM_SOUND_BUFFER_SIZE
    }
}
