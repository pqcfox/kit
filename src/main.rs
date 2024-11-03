#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::{
    epd1in54_v2::{Display1in54, Epd1in54},
    prelude::*,
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
    spi::{master::Spi, SpiMode},
};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new();

    let cs = Output::new(io.pins.gpio33, Level::High);
    let dc = Output::new(io.pins.gpio34, Level::Low);
    let rst = Output::new(io.pins.gpio35, Level::Low);
    let busy_in = Input::new(io.pins.gpio36, Pull::None);
    let spi_bus = Spi::new(peripherals.SPI2, 20.MHz(), SpiMode::Mode0)
        .with_sck(io.pins.gpio47)
        .with_mosi(io.pins.gpio48)
        .with_miso(io.pins.gpio46);
    let mut spi = ExclusiveDevice::new(spi_bus, cs, delay).unwrap();
    let mut epd = Epd1in54::new(&mut spi, busy_in, dc, rst, &mut delay, None).unwrap();

    let mut display = Display1in54::default();
    display.set_rotation(DisplayRotation::Rotate0);

    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let _ = Text::with_text_style("Hello, world!", Point::new(5, 50), style, text_style)
        .draw(&mut display);
    let _ = epd.update_frame(&mut spi, display.buffer(), &mut delay);
    let _ = epd.display_frame(&mut spi, &mut delay);

    loop {}
}
