#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config};
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_10X20}, pixelcolor::{BinaryColor}, prelude::*, text::Text};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_2;
    let scl = p.PIN_3;

    let i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());

    let interface = I2CDisplayInterface::new(i2c);
    info!("Hello world");

    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().unwrap();


    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    Text::new("Embed albo ", Point::new(0,18), style).draw(&mut display).unwrap();
    Text::new("frajer", Point::new(0,30), style).draw(&mut display).unwrap();
    display.flush().unwrap();

    info!("OLED flushed");
}
