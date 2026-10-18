use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, primitives::Rectangle, text::Text};

pub mod style;

#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum DisplayUpdate {
    Lock,
    Unlock,
    ClearValues,
    SetSelection(bool),
    SetUbe(f32),
    SetHfe(f32),
    SetUceSat(Option<f32>),
}

/// Clear the display.
pub fn clear<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    display
        .bounding_box()
        .offset(1)
        .into_styled(style::CLEAR)
        .draw(display)
}

/// Clear the numeric values.
pub fn clear_values<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Rectangle::new(Point::new(33, 17), Size::new(108, 12))
        .into_styled(style::CLEAR)
        .draw(display)?;
    Rectangle::new(Point::new(33, 34), Size::new(72, 12))
        .into_styled(style::CLEAR)
        .draw(display)?;
    Rectangle::new(Point::new(57, 51), Size::new(108, 12))
        .into_styled(style::CLEAR)
        .draw(display)?;

    Ok(())
}

/// Draw the static layout of the display.
pub fn draw_layout<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    // LOGO
    Rectangle::new(Point::new(87, 0), Size::new(41, 13))
        .into_styled(style::FILL)
        .draw(display)?;
    Text::with_text_style(
        "QPoint",
        Point::new(90, 0),
        style::mono_font::INVERTED,
        style::text::LEFT,
    )
    .draw(display)?;

    // SELECTION
    Text::with_text_style(
        "SEL",
        Point::new(3, 0),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        ":",
        Point::new(21, 0),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;

    // U_BE
    Text::with_text_style(
        "U",
        Point::new(3, 17),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        "BE",
        Point::new(10, 30),
        style::mono_font::SUBSCRIPT,
        style::text::SUB_LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        ":",
        Point::new(21, 17),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;

    // h_FE
    Text::with_text_style(
        "h",
        Point::new(3, 34),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        "FE",
        Point::new(10, 47),
        style::mono_font::SUBSCRIPT,
        style::text::SUB_LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        ":",
        Point::new(21, 34),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;

    // U_CE(sat)
    Text::with_text_style(
        "U",
        Point::new(3, 51),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        "CE(sat)",
        Point::new(10, 63),
        style::mono_font::SUBSCRIPT,
        style::text::SUB_LEFT,
    )
    .draw(display)?;
    Text::with_text_style(
        ":",
        Point::new(45, 51),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;

    Ok(())
}

/// Draw the currently selected type. `true` is PNP, `false` is NPN.
pub fn draw_selection<D>(display: &mut D, selection: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text = if selection { "PNP" } else { "NPN" };

    Text::with_text_style(
        text,
        Point::new(33, 0),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Ok(())
}

pub fn draw_ube<D>(display: &mut D, ube: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text = format_ube(ube);

    Text::with_text_style(
        text.as_str(),
        Point::new(33, 17),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Ok(())
}

pub fn draw_hfe<D>(display: &mut D, hfe: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text = format_hfe(hfe);

    Text::with_text_style(
        text.as_str(),
        Point::new(33, 34),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Ok(())
}

pub fn draw_uce_sat<D>(display: &mut D, uce_sat: Option<f32>) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let text = if let Some(uce_sat) = uce_sat {
        format_uce_sat(uce_sat)
    } else {
        // types must be the same thus the string of length 7
        defmt::unwrap!(heapless::format!(7; "???"))
    };

    Text::with_text_style(
        text.as_str(),
        Point::new(57, 51),
        style::mono_font::NORMAL,
        style::text::LEFT,
    )
    .draw(display)?;
    Ok(())
}

/// Draw a cover informing the user that a measurement from the PC application is in progress.
pub fn draw_cover<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    clear(display)?;

    // LOGO
    Rectangle::new(Point::new(43, 0), Size::new(41, 13))
        .into_styled(style::FILL)
        .draw(display)?;
    Text::with_text_style(
        "QPoint",
        Point::new(63, 0),
        style::mono_font::INVERTED,
        style::text::CENTER,
    )
    .draw(display)?;

    // MESSAGE
    Text::with_text_style(
        "Measurement",
        Point::new(63, 17),
        style::mono_font::NORMAL,
        style::text::CENTER,
    )
    .draw(display)?;
    Text::with_text_style(
        "in",
        Point::new(63, 34),
        style::mono_font::NORMAL,
        style::text::CENTER,
    )
    .draw(display)?;
    Text::with_text_style(
        "progress...",
        Point::new(63, 51),
        style::mono_font::NORMAL,
        style::text::CENTER,
    )
    .draw(display)?;

    Ok(())
}

/// Format the base-emitter voltage value in a special way.
#[inline]
fn format_ube(x: f32) -> heapless::String<9> {
    for precision in (0..=4).rev() {
        if let Ok(s) = heapless::format!(9; "{:+.*} V", precision, x) {
            return s;
        }
    }

    defmt::panic!("Couldn't format U_BE = {} into 9 characters", x);
}

/// Format the gain value in a special way.
#[inline]
fn format_hfe(x: f32) -> heapless::String<6> {
    for precision in (0..=4).rev() {
        if let Ok(s) = heapless::format!(6; "{:.*}", precision, x) {
            return s;
        }
    }

    defmt::panic!("Couldn't format h_FE = {} into 6 characters", x);
}

/// Format the base-emitter voltage value in a special way.
#[inline]
fn format_uce_sat(x: f32) -> heapless::String<7> {
    for precision in (0..=2).rev() {
        if let Ok(s) = heapless::format!(7; "{:+.*} V", precision, x) {
            return s;
        }
    }

    defmt::panic!("Couldn't format U_CE(sat) = {} into 7 characters", x);
}
