use embedded_graphics::{
    mono_font::{ascii::FONT_8X12, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let fill_style = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_8X12, BinaryColor::On);

    // Draw battery indicator
    Rectangle::new(Point::new(2, 2), Size::new(24, 50))
        .into_styled(thin_stroke)
        .draw(&mut display)?;

    // Draw individual battery level indicators instead of a bar
    for i in 0..3 {
        let y_pos = 3 + i * 12;
        Rectangle::new(Point::new(6, y_pos), Size::new(18, 8))
            .into_styled(fill_style)
            .draw(&mut display)?;

        // Add a checkmark to the filled battery indicators
        if i < 2 {
            Rectangle::new(Point::new(9, y_pos + 2), Size::new(4, 6))
                .into_styled(thin_stroke)
                .draw(&mut display)?;
        }
    }

    // Draw speed indicator with numerical value
    Rectangle::new(Point::new(40, 50), Size::new(30, 10))
        .into_styled(thin_stroke)
        .draw(&mut display)?;
    Text::new("42 km/h", Point::new(40, 42), character_style) // Replace 42 with actual speed value
        .draw(&mut display)?;

    // Draw power indicator
    Rectangle::new(Point::new(90, 50), Size::new(30, 10))
        .into_styled(thin_stroke)
        .draw(&mut display)?;
    Text::new("80%", Point::new(90, 42), character_style) // Replace 80 with actual power level
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::BlackWhite)
        .build();
    Window::new("Status Display", &output_settings).show_static(&display);

    Ok(())
}