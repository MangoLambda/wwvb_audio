mod adjustable_sine_wave;
mod signal;
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread, time::Duration};

use signal::Signal;
mod wwvb_amplitude_shift_keying_modulator;
use wwvb_amplitude_shift_keying_modulator::WwvbAmplitudeShiftKeyingModulator;

fn main() {
    let frequency = 440.0;
    let queue: Arc<Mutex<VecDeque<char>>> = Arc::new(Mutex::new(VecDeque::new()));
    let wwvb_modulator = WwvbAmplitudeShiftKeyingModulator::new(
        frequency, Arc::clone(&queue));
    wwvb_modulator.start();

    {
        let mut deque = queue.lock().unwrap();
        deque.push_back('H');
        deque.push_back('L');
        deque.push_back('M');
    }

    loop {

        
        thread::sleep(Duration::from_millis(1));
    }
}