use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;

use qpoint_common::Parameter;

pub mod attached;
pub mod normal;
pub mod style;

#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum DisplayUpdate {
    SetAttached,
    SetNormal,
    SetSelection(bool),
    SetDrop(f32),
    SetGain(f32),
    SetParameter { value: f32, param: Parameter },
}

impl DisplayUpdate {
    pub fn is_normal_mode(&self) -> bool {
        match self {
            DisplayUpdate::SetParameter { .. } => false,
            _ => true,
        }
    }
}

/// Clear the display.
pub fn clear<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    display
        .bounding_box()
        .offset(1)
        .into_styled(style::LINE_STYLE)
        .draw(display)
}
