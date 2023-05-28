use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::config::{Config, DriverConfig};
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_sys::{self as _, EspError};

use display_interface_spi::SPIInterfaceNoCS;
use log::*;
use mipidsi::Builder;

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb666;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

fn main() -> Result<(), EspError> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sclk = peripherals.pins.gpio14;
    let sdo = peripherals.pins.gpio13;
    let sdi = peripherals.pins.gpio12;
    let cs = peripherals.pins.gpio15;
    let dc = peripherals.pins.gpio2;
    let rst = peripherals.pins.gpio19;

    let config = Config::default();
    let bus_config = DriverConfig::default();
    let spi_driver = SpiDeviceDriver::new_single(
        peripherals.spi2,
        sclk,
        sdo,
        Some(sdi),
        Some(cs),
        &bus_config,
        &config,
    )?;

    let dc_io = PinDriver::input_output(dc)?;
    let rst_io = PinDriver::input_output_od(rst)?;
    let spi = SPIInterfaceNoCS::new(spi_driver, dc_io);
    let mut display = Builder::ili9486_rgb666(spi)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut Delay, Some(rst_io))
        .unwrap();

    loop {
        display.clear(Rgb666::RED).unwrap();
        info!("Hello World");

        Text::new(
            "Hello World!",
            Point::new(100, 50),
            MonoTextStyle::new(&FONT_6X10, Rgb666::BLACK),
        )
        .draw(&mut display)
        .unwrap();

        Delay::delay_ms(5000);
    }
}
