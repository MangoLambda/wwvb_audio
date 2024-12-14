mod signal;
mod adjustable_sine_wave;
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread, time::Duration};

mod wwvb_amplitude_shift_keying_modulator;
use chrono::Local;
use wwvb_amplitude_shift_keying_modulator::WwvbAmplitudeShiftKeyingModulator;

mod wwvb_encoder;
use wwvb_encoder::WwvbEncoder;

fn main() {
    let frequency = 440.0;
    let queue: Arc<Mutex<VecDeque<char>>> = Arc::new(Mutex::new(VecDeque::new()));
    let wwvb_modulator = WwvbAmplitudeShiftKeyingModulator::new(
        frequency, Arc::clone(&queue));
    wwvb_modulator.start();

    let new_wwvb_time = WwvbEncoder::encode(Local::now());
    let mut new_wwvb_time_queue = VecDeque::from(new_wwvb_time);


    {
        let mut deque = queue.lock().unwrap();
        deque.push_back('H');
        deque.push_back('L');
        deque.push_back('M');
    }

    loop {
        thread::sleep(Duration::from_millis(1000));
        {
            let mut deque = queue.lock().unwrap();
            deque.append(&mut new_wwvb_time_queue);
        }
    }
}