
use std::{collections::VecDeque, sync::{Arc, Mutex}, thread, time::Duration};

use crate::signal::Signal;

// Faster bit time to prevent time drift elongation due to program execution time.
// We can wait to re-sync for a few ms each minute rather than skip a whole minute due to desync.
const TIME_VARIATION:   f32 = 0.995; 
const TIME_MS_TO_US:    f32 = 1000.0;

const SHORT_DURATION:    Duration = Duration::from_micros((200.0 * TIME_MS_TO_US * TIME_VARIATION) as u64);
const MID_DURATION:      Duration = Duration::from_micros((500.0 * TIME_MS_TO_US * TIME_VARIATION) as u64);
const LONG_DURATION:     Duration = Duration::from_micros((800.0 * TIME_MS_TO_US * TIME_VARIATION) as u64);

const LOW_AMPLITUDE:    f32 = 0.1;
const HIGH_AMPLITUDE:   f32 = 1.0;

const LOW_BIT:  char = 'L';
const HIGH_BIT: char = 'H';
const MARK_BIT: char = 'M';

pub struct WwvbAmplitudeShiftKeyingModulator {
    frequency: f32,
    bit_queue: Arc<Mutex<VecDeque<char>>>,
}

impl WwvbAmplitudeShiftKeyingModulator {
    pub fn new(frequency: f32, bit_queue: Arc<Mutex<VecDeque<char>>>) -> Self {       

        Self {
            frequency: frequency,
            bit_queue: bit_queue
        }
    }

    pub fn start(self) {
        let bit_queue_clone = Arc::clone(&self.bit_queue); // Clone Arc to share between threads

        thread::spawn(move || {
            let signal = Signal::new(self.frequency, 1.0);
            Self::command_executor_thread(signal, bit_queue_clone);
        });

    }

    pub fn command_executor_thread(signal: Signal, bit_queue: Arc<Mutex<VecDeque<char>>>) {
        loop {
            let bit = {
                let mut deque = bit_queue.lock().unwrap();
                deque.pop_front()
            };

            if let Some(bit) = bit {
                match bit {
                    LOW_BIT => Self::write_low(&signal),
                    HIGH_BIT => Self::write_high(&signal),
                    MARK_BIT => Self::write_high(&signal),
                    _ => (),
                }
            }

            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn write_low(signal: &Signal) {
        signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(SHORT_DURATION);
        signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(LONG_DURATION);
    }

    pub fn write_high(signal: &Signal) {
        signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(MID_DURATION);
        signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(MID_DURATION);
    }


    pub fn write_mark(signal: &Signal) {
        signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(LONG_DURATION);
        signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(SHORT_DURATION);
    }
}
