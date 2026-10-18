use embedded_graphics::{
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
};

pub const LINE: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(BinaryColor::On)
    .build();

pub const FILL: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .fill_color(BinaryColor::On)
    .build();

pub const CLEAR: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .fill_color(BinaryColor::Off)
    .build();

pub mod mono_font {
    use embedded_graphics::{
        mono_font::ascii::FONT_5X8,
        mono_font::ascii::FONT_6X12,
        mono_font::{MonoTextStyle, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
    };

    pub const NORMAL: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X12)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    pub const INVERTED: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X12)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

    pub const SUBSCRIPT: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
}

pub mod text {
    use embedded_graphics::text::{Alignment, Baseline, TextStyle, TextStyleBuilder};

    pub const LEFT: TextStyle = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();

    pub const SUB_LEFT: TextStyle = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Bottom)
        .build();

    pub const CENTER: TextStyle = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Top)
        .build();
}
