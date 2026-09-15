use embedded_graphics::{
    mono_font::ascii::{FONT_4X6, FONT_5X8, FONT_6X12},
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::{Alignment, Baseline, TextStyle, TextStyleBuilder},
};

pub const LINE_STYLE: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(BinaryColor::On)
    .fill_color(BinaryColor::Off)
    .build();

pub const FILL_STYLE: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(BinaryColor::On)
    .fill_color(BinaryColor::On)
    .build();

pub const DEFAULT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X12)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

pub const INVERTED_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X12)
    .text_color(BinaryColor::Off)
    .background_color(BinaryColor::On)
    .build();

pub const SUB_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_4X6)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

pub const SYM_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_5X8)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

pub const DEFAULT_TEXT: TextStyle = TextStyleBuilder::new()
    .alignment(Alignment::Left)
    .baseline(Baseline::Middle)
    .build();

pub const NUM_TEXT: TextStyle = TextStyleBuilder::new()
    .alignment(Alignment::Center)
    .baseline(Baseline::Middle)
    .build();

pub const SUB_TEXT: TextStyle = TextStyleBuilder::new()
    .alignment(Alignment::Left)
    .baseline(Baseline::Top)
    .build();
