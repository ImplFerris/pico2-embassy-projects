#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

// I2C
use embassy_rp::i2c::{self, Config};

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_18;
    let scl = p.PIN_19;

    let mut i2c_config = Config::default();
    i2c_config.frequency = 400_000; //400kHz

    let i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, i2c_config);

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    defmt::info!("sending text to display");
    Text::with_baseline("Hello, Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        Timer::after_millis(100).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"hello-oled"),
    embassy_rp::binary_info::rp_program_description!(c"Hello OLED"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
