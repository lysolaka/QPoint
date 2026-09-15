use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, Rectangle},
    text::Text,
};

use crate::util::display::style::*;

/// Draw the layout for normal mode.
pub fn draw_layout<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Text::with_text_style(
        "QPoint: basic mode",
        Point::new(3, 5),
        DEFAULT_STYLE,
        DEFAULT_TEXT,
    )
    .draw(display)?;

    Text::with_text_style("U", Point::new(3, 34), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("BE", Point::new(10, 34), SYM_STYLE, SUB_TEXT).draw(display)?;
    Text::with_text_style(":", Point::new(21, 34), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("V", Point::new(65, 34), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;

    Text::with_text_style("h", Point::new(3, 51), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("FE", Point::new(10, 51), SYM_STYLE, SUB_TEXT).draw(display)?;
    Text::with_text_style(":", Point::new(21, 51), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;

    Ok(())
}

/// Draw the NPN/PNP mode selection.
pub fn draw_selection<D>(display: &mut D, selection: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let (npn_style, pnp_style) = if selection == false {
        // NPN selected
        Rectangle::with_corners(Point::new(38, 14), Point::new(60, 26))
            .into_styled(FILL_STYLE)
            .draw(display)?;

        Rectangle::with_corners(Point::new(68, 14), Point::new(90, 26))
            .into_styled(LINE_STYLE)
            .draw(display)?;

        (INVERTED_STYLE, DEFAULT_STYLE)
    } else {
        // PNP selected
        Rectangle::with_corners(Point::new(38, 14), Point::new(60, 26))
            .into_styled(LINE_STYLE)
            .draw(display)?;

        Rectangle::with_corners(Point::new(68, 14), Point::new(90, 26))
            .into_styled(FILL_STYLE)
            .draw(display)?;

        (DEFAULT_STYLE, INVERTED_STYLE)
    };

    Text::with_text_style("NPN", Point::new(49, 19), npn_style, NUM_TEXT).draw(display)?;
    Text::with_text_style("PNP", Point::new(79, 19), pnp_style, NUM_TEXT).draw(display)?;

    if selection == false {
        // NPN selected
        Line::new(Point::new(68, 14), Point::new(90, 14))
            .into_styled(LINE_STYLE)
            .draw(display)?;
    } else {
        // PNP selected
        Line::new(Point::new(38, 14), Point::new(60, 14))
            .into_styled(LINE_STYLE)
            .draw(display)?;
    }

    Ok(())
}

/// Draw the value of base-emitter voltage drop
pub fn draw_drop<D>(display: &mut D, drop: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let drop = format_param(drop);
    Text::with_text_style(&drop, Point::new(27, 34), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;

    Ok(())
}

/// Draw the value of gain.
pub fn draw_gain<D>(display: &mut D, gain: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let drop = format_param(gain);
    Text::with_text_style(&drop, Point::new(27, 51), DEFAULT_STYLE, DEFAULT_TEXT).draw(display)?;

    Ok(())
}

/// Format the value of a parameter in a special way.
fn format_param(x: f32) -> heapless::String<6> {
    for precision in (0..=4).rev() {
        if let Ok(s) = heapless::format!(6; "{:.*}", precision, x) {
            return s;
        }
    }
    defmt::panic!("Couldn't format x = {} into 6 characters", x);
}
