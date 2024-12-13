
use std::{thread, time::Duration};

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

pub struct WwvbAmplitudeShiftKeyingModulator {
    signal: Signal,
}

impl WwvbAmplitudeShiftKeyingModulator {
    pub fn new(signal: Signal) -> Self {        
        Self {
            signal,
        }
    }

    pub fn write_low(&self) {
        self.signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(SHORT_DURATION);
        self.signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(LONG_DURATION);
    }

    pub fn write_high(&self) {
        self.signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(MID_DURATION);
        self.signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(MID_DURATION);
    }


    pub fn write_mark(&self) {
        self.signal.set_amplitude(LOW_AMPLITUDE);
        thread::sleep(LONG_DURATION);
        self.signal.set_amplitude(HIGH_AMPLITUDE);
        thread::sleep(SHORT_DURATION);
    }
}
