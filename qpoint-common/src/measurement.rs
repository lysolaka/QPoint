use serde::{Serialize, Deserialize};

/// 1 LSB for the ADC/DAC given in milivolts (mV) for mapping to 3,3V.
pub const U3V3_1LSB: f32 = 0.805664;
/// 1 LSB for the ADC/DAC given in milivolts (mV) for mapping to 5V.
pub const U5V_1LSB: f32 = 1.220703;

/// Circuit used to drive the base terminal.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseSource {
    /// High impedance.
    ///
    /// The base terminal is effectively disconnected (floating).
    HighZ,
    /// Current source.
    ///
    /// Creates a positive voltage at the base terminal, such that the current flowing **out** of this
    /// circuit is proportional to the setting voltage. Current range: [0, 150] uA.
    ISource,
    /// Current sink.
    ///
    /// Creates a positive voltage at the base terminal, such that the current flowing **into** this
    /// circuit is proportional to the setting voltage. Current range: [0, 150] uA.
    ISink,
    /// Voltage source.
    ///
    /// Connects the base terminal to a variable voltage source ([0, 5] V) through a 15k resistor.
    VSource,
}

impl BaseSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    pub fn selection(&self) -> (bool, bool) {
        match self {
            BaseSource::HighZ => (false, false),
            BaseSource::ISource => (false, true),
            BaseSource::ISink => (true, false),
            BaseSource::VSource => (true, true),
        }
    }

    /// DAC setting to drive the base terminal with the given value.
    ///
    /// The unit of `value` is:
    /// - uA for `ISource` and `ISink`,
    /// - V for `VSource`,
    /// - *ignored* for `HighZ` (returns 0).
    // - ISource | ISink: I [A] = U [V] / 22k => U [V] = I [A] * 22k => U [mV] = I [uA] / 22
    pub fn dac_value(&self, value: f32) -> u16 {
        match self {
            BaseSource::HighZ => 0,
            BaseSource::ISource | BaseSource::ISink => {
                libm::roundf((value * 22.0) / U3V3_1LSB) as u16
            }
            BaseSource::VSource => libm::roundf((value * 1000.0) / U5V_1LSB) as u16,
        }
    }
}

/// Circuit used to drive the collector terminal.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectorSource {
    /// High impedance.
    ///
    /// The collector terminal is effectively disconnected (floating).
    HighZ,
    /// +5 V.
    ///
    /// Connects the collector directly to the 5 V power supply.
    VCC,
    /// Voltage source.
    ///
    /// Connects the collector terminal to a variable voltage source ([0, 5] V).
    VSource,
    /// GND.
    ///
    /// Connects the collector directly to ground.
    GND,
}

impl CollectorSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    pub fn selection(&self) -> (bool, bool) {
        match self {
            CollectorSource::HighZ => (false, false),
            CollectorSource::VCC => (false, true),
            CollectorSource::VSource => (true, false),
            CollectorSource::GND => (true, true),
        }
    }

    /// DAC setting to drive the base terminal with the given value.
    ///
    /// The unit of the value is:
    /// - V for `VSource`,
    /// - *ignored* for `HighZ`, `VCC`, `GND` (returns 0).
    pub fn dac_value(&self, value: f32) -> u16 {
        match self {
            CollectorSource::VSource => libm::roundf((value * 1000.0) / U5V_1LSB) as u16,
            _ => 0,
        }
    }
}

/// Circuit used to drive the emitter terminal.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmitterSource {
    /// High impedance.
    ///
    /// The emitter terminal is effectively disconnected (floating).
    HighZ,
    /// +5 V.
    ///
    /// Connects the emitter directly to the 5 V power supply.
    VCC,
    /// GND.
    ///
    /// Connects the emitter directly to ground.
    GND,
}

impl EmitterSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    pub fn selection(&self) -> (bool, bool) {
        match self {
            EmitterSource::HighZ => (false, false),
            EmitterSource::VCC => (false, true),
            EmitterSource::GND => (true, false),
        }
    }
}

/// ADC driver measurement results, converted to real world units.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MeasurementResult {
    /// Base voltage in milivolts [mV].
    pub base_voltage: f32,
    /// Collector current in miliamperes [mA].
    pub collector_current: f32,
}

#[cfg(feature = "std")]
impl std::fmt::Display for BaseSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BaseSource::HighZ => "High impedance",
            BaseSource::ISource => "Current source",
            BaseSource::ISink => "Current sink",
            BaseSource::VSource => "Voltage source",
        })
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CollectorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CollectorSource::HighZ => "High impedance",
            CollectorSource::VCC => "+5 V",
            CollectorSource::VSource => "Voltage source",
            CollectorSource::GND => "Ground",
        })
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for EmitterSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EmitterSource::HighZ => "High impedance",
            EmitterSource::VCC => "+5 V",
            EmitterSource::GND => "Ground",
        })
    }
}
