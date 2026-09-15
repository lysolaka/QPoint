use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;

pub mod attached;
pub mod normal;
pub mod style;

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
