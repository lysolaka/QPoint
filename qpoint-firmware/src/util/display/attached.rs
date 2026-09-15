use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Polyline;
use embedded_graphics::text::Text;

use qpoint_common::Parameter;

use crate::util::display::style::*;

/// Layout for the attached mode.
pub fn draw_layout<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    const BR_SYM_LEFT: [Point; 4] = [
        Point::new(94, 2),
        Point::new(91, 2),
        Point::new(91, 24),
        Point::new(94, 24),
    ];
    const BR_SYM_RIGHT: [Point; 4] = [
        Point::new(122, 2),
        Point::new(125, 2),
        Point::new(125, 24),
        Point::new(122, 24),
    ];

    const BR_VAL_LEFT: [Point; 4] = [
        Point::new(5, 28),
        Point::new(2, 28),
        Point::new(2, 61),
        Point::new(5, 61),
    ];
    const BR_VAL_RIGHT: [Point; 4] = [
        Point::new(122, 28),
        Point::new(125, 28),
        Point::new(125, 61),
        Point::new(122, 61),
    ];

    Text::with_text_style(
        "QPoint: h-mode",
        Point::new(3, 5),
        DEFAULT_STYLE,
        DEFAULT_TEXT,
    )
    .draw(display)?;
    Text::with_text_style("Results:", Point::new(3, 17), DEFAULT_STYLE, DEFAULT_TEXT)
        .draw(display)?;

    Polyline::new(&BR_SYM_LEFT)
        .into_styled(LINE_STYLE)
        .draw(display)?;
    Polyline::new(&BR_SYM_RIGHT)
        .into_styled(LINE_STYLE)
        .draw(display)?;

    Text::with_text_style("h", Point::new(94, 7), SYM_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("ie", Point::new(99, 7), SUB_STYLE, SUB_TEXT).draw(display)?;

    Text::with_text_style("h", Point::new(111, 7), SYM_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("re", Point::new(116, 7), SUB_STYLE, SUB_TEXT).draw(display)?;

    Text::with_text_style("h", Point::new(94, 17), SYM_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("fe", Point::new(99, 17), SUB_STYLE, SUB_TEXT).draw(display)?;

    Text::with_text_style("h", Point::new(111, 17), SYM_STYLE, DEFAULT_TEXT).draw(display)?;
    Text::with_text_style("oe", Point::new(116, 17), SUB_STYLE, SUB_TEXT).draw(display)?;

    Polyline::new(&BR_VAL_LEFT)
        .into_styled(LINE_STYLE)
        .draw(display)?;
    Polyline::new(&BR_VAL_RIGHT)
        .into_styled(LINE_STYLE)
        .draw(display)?;

    Ok(())
}

/// Draw the `value` of `parameter` on the screen.
pub fn draw_parameter<D>(display: &mut D, parameter: Parameter, value: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let value = format_param(value);

    Text::with_text_style(&value, parameter.draw_pos().into(), DEFAULT_STYLE, NUM_TEXT)
        .draw(display)?;

    Ok(())
}

/// Format the value of a parameter in a special way.
fn format_param(x: f32) -> heapless::String<9> {
    for precision in (0..=7).rev() {
        if let Ok(s) = heapless::format!(9; "{:+.*}", precision, x) {
            return s;
        }
    }
    defmt::panic!("Couldn't format x = {} into 9 characters", x);
}
