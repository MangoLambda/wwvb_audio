mod adjustable_sine_wave;
mod signal;
use signal::Signal;
mod wwvb_amplitude_shift_keying_modulator;
use wwvb_amplitude_shift_keying_modulator::WwvbAmplitudeShiftKeyingModulator;

fn main() {
    let signal = Signal::new(500.0, 1.0);
    let wwvb_modulator = WwvbAmplitudeShiftKeyingModulator::new(signal);

    loop {
        wwvb_modulator.write_low();
        wwvb_modulator.write_high();
        wwvb_modulator.write_mark();
    }
}