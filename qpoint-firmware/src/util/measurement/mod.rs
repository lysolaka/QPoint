mod adc_control;
mod base_control;
mod collector_control;
mod emitter_control;

pub use adc_control::AdcControl;
pub use base_control::BaseControl;
pub use collector_control::CollectorControl;
pub use emitter_control::EmitterControl;

/// Clamp a value to an interval [`min`, `max`].
pub fn clamp<T: PartialOrd + Copy>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

/// Produce an interval of length `N` from a start value and step value.
pub fn interval<const N: usize>(start: f32, step: f32) -> [f32; N] {
    let mut interval = [0.0; N];

    for i in 0..N {
        interval[i] = start + (i as f32) * step;
    }

    interval
}

/// Estimate the value of collector-emitter saturation voltage from the measurement of collector
/// currents every 0.02V of collector-emitter voltage. In other words, the function assumes that
/// `currents[0]` is the current at 0.00V, `currents[1]` at 0.02V and so on.
///
/// Returns the estimated value of U_CE(sat) if `Some`, or `None` if the estimation could not find a
/// value.
pub fn estimate_uce_sat<const N: usize>(currents: [f32; N]) -> Option<f32> {
    // waste 4 bytes of space for an additional float, but make the function generic over N
    // `let mut diffs = [0.0; N - 1];` is an error
    let mut diffs = [0.0; N];

    for i in 0..(N - 1) {
        diffs[i] = currents[i + 1] - currents[i];
    }

    let mut peak_index = 0;
    let mut peak_slope = diffs[0] / 0.02;

    for i in 1..(N - 1) {
        let slope = diffs[i] / 0.02;
        if slope > peak_slope {
            peak_slope = slope;
            peak_index = i;
        }
    }

    if peak_slope <= 0.0 {
        return None;
    }

    let threshold = 0.2 * peak_slope;

    for i in peak_index..(N - 1) {
        let slope = diffs[i] / 0.02;
        if slope <= threshold {
            return Some((i as f32) * 0.02);
        }
    }

    return None;
}
