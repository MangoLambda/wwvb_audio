
use std::{thread, time::Duration};

use crate::signal::Signal;

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
        self.signal.set_amplitude(0.1);
        thread::sleep(Duration::from_millis(200));
        self.signal.set_amplitude(1.0);
        thread::sleep(Duration::from_millis(800));
    }

    pub fn write_high(&self) {
        self.signal.set_amplitude(0.1);
        thread::sleep(Duration::from_millis(500));
        self.signal.set_amplitude(1.0);
        thread::sleep(Duration::from_millis(500));
    }


    pub fn write_mark(&self) {
        self.signal.set_amplitude(0.1);
        thread::sleep(Duration::from_millis(800));
        self.signal.set_amplitude(1.0);
        thread::sleep(Duration::from_millis(200));
    }
}
