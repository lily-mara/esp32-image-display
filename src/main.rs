use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{image::Image, prelude::*};
use esp_idf_hal::{
    delay::Ets,
    gpio::{AnyIOPin, Gpio16, Gpio18, Gpio19, Gpio23, Gpio4, Gpio5, PinDriver},
    spi::{config::Config, Dma, SpiDeviceDriver, SpiDriver, SPI2},
};
use mipidsi::Builder;
use std::error::Error;
use tinybmp::Bmp;

use esp_idf_sys as _;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_sys::link_patches();

    let rst = PinDriver::input_output_od(unsafe { Gpio23::new() })?;
    let dc = PinDriver::input_output_od(unsafe { Gpio16::new() })?;
    let mut delay = Ets;

    let sclk = unsafe { Gpio18::new() };
    let spi = unsafe { SPI2::new() };
    let sdo = unsafe { Gpio19::new() };

    let spi = SpiDriver::new(
        spi,
        sclk,
        sdo,
        None::<AnyIOPin>,
        Dma::Channel1(240 * 135 * 2 + 8),
    )?;

    let mut bl = PinDriver::input_output_od(unsafe { Gpio4::new() })?;
    bl.set_high()?;

    let cs = unsafe { Gpio5::new() };

    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;

    let di = SPIInterfaceNoCS::new(spi, dc);
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|_| Box::<dyn Error>::from("display init"))?;

    let bmp_data = include_bytes!("IMG_2446.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();

    Image::new(&bmp, Point::new(0, 0))
        .draw(&mut display)
        .map_err(|_| Box::<dyn Error>::from("draw image"))?;

    Ok(())
}
