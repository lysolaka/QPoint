#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod measurement;

use serde::{Deserialize, Serialize};

use crate::measurement::{BaseSource, CollectorSource, EmitterSource, MeasurementResult};

/// Command type to control the behaviour of the firmware from a PC.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    /// Setup the device for control by the USB interface.
    Attach,
    /// Return control to the basic UI.
    Detach,
    /// Display the `value` of parameter `param` on the screen.
    DisplayParam { param: Parameter, value: f32 },
    /// Set an RGB LED colour.
    ///
    /// The format is `(r, g, b)`
    LedSet(u8, u8, u8),
    /// Select a base source.
    BaseSelect(BaseSource),
    /// Set a `value` on the base terminal.
    ///
    /// The allowed ranges depend on the currently selected source:
    /// - [`BaseSource::HighZ`]: irrelevant (forced to 0).
    /// - [`BaseSource::ISource`]: `0..=150` uA,
    /// - [`BaseSource::ISink`]: `0..=150` uA,
    /// - [`BaseSource::VSource`]: `0..=5` V,
    ///
    /// The unit of `value` is:
    /// - uA for [`BaseSource::ISource`] and [`BaseSource::ISink`],
    /// - V for [`BaseSource::VSource`],
    /// - *ignored* for [`BaseSource::HighZ`] (`value` forced to 0).
    ///
    /// If `measure` is `true`, the firmware will respond with a measurement taken after setting the
    /// new value.
    BaseSet { value: f32, measure: bool },
    /// Select a collector source.
    CollectorSelect(CollectorSource),
    /// Set a `value` on the collector terminal.
    ///
    /// The allowed ranges depend on the currently selected source:
    /// - [`CollectorSource::VSource`]: `0..=5` V,
    /// - *others*: value forced to 0.
    ///
    /// The unit of `value` is:
    /// - V for [`CollectorSource::VSource`],
    /// - *ignored* for others (`value` is forced to 0).
    ///
    /// If `measure` is `true`, the firmware will respond with a measurement taken after setting the
    /// new value.
    CollectorSet { value: f32, measure: bool },
    /// Select an emitter source.
    EmitterSelect(EmitterSource),
}

impl Command {
    /// Is the command supposed to be run by the measurement runner.
    pub fn is_measurement(&self) -> bool {
        match self {
            Command::BaseSelect(_) => true,
            Command::BaseSet { .. } => true,
            Command::CollectorSelect(_) => true,
            Command::CollectorSet { .. } => true,
            Command::EmitterSelect(_) => true,
            _ => false,
        }
    }
}

/// Responses to commands sent back by the firmware.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Response {
    /// Command acknowledged.
    Ack,
    /// Measurement result sent after a set command.
    Measurement(MeasurementResult),
}

/// Hybrid model parameters names.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Parameter {
    Hie,
    Hre,
    Hfe,
    Hoe,
}

impl Parameter {
    /// Position of the parameter on the screen.
    pub fn draw_pos(&self) -> (i32, i32) {
        match self {
            Parameter::Hie => (32, 34),
            Parameter::Hre => (95, 34),
            Parameter::Hfe => (32, 53),
            Parameter::Hoe => (95, 53),
        }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Parameter::Hie => "Hie",
            Parameter::Hre => "Hre",
            Parameter::Hfe => "Hfe",
            Parameter::Hoe => "Hoe",
        })
    }

}
