use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
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
    let character_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    // Draw battery indicator
    Rectangle::new(Point::new(2, 2), Size::new(24, 50))
        .into_styled(thin_stroke)
        .draw(&mut display)?;

    // Battery levels
    let battery_level_height: i32 = 10; // Now it's i32 to match Point::new expectations
    for i in 0..3 {
        Rectangle::new(
            Point::new(3, 3 + i * (battery_level_height + 2) as i32),
            Size::new(22, battery_level_height as u32), // Cast to u32 here for Size::new
        )
        .into_styled(fill_style)
        .draw(&mut display)?;
    }

    // Draw speed indicator
    Rectangle::new(Point::new(40, 50), Size::new(30, 10))
        .into_styled(thin_stroke)
        .draw(&mut display)?;
    Rectangle::new(Point::new(41, 51), Size::new(15, 8))
        .into_styled(fill_style)
        .draw(&mut display)?;
    Text::new("Speed", Point::new(40, 40), character_style)
        .draw(&mut display)?;

    // Draw power indicator
    Rectangle::new(Point::new(90, 50), Size::new(30, 10))
        .into_styled(thin_stroke)
        .draw(&mut display)?;
    Rectangle::new(Point::new(91, 51), Size::new(10, 8))
        .into_styled(fill_style)
        .draw(&mut display)?;
    Text::new("Power", Point::new(90, 40), character_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Status Display", &output_settings).show_static(&display);

    Ok(())
}
