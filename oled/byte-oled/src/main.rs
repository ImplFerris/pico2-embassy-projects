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

// Interrupt Binding
use embassy_rp::peripherals::I2C0;
use embassy_rp::{bind_interrupts, i2c};

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c};

// OLED
use ssd1306::{I2CDisplayInterface, Ssd1306Async, prelude::*};

// Embedded Graphics
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
};

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

// 8x5 pixels
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    0b00111000,
    0b01000100,
    0b01000100,
    0b00101000,
    0b11101110,
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_16;
    let scl = p.PIN_17;

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000; // 400kHz

    let i2c_bus = I2c::new_async(p.I2C0, scl, sda, Irqs, i2c_config);

    let i2c_interface = I2CDisplayInterface::new(i2c_bus);

    let mut display = Ssd1306Async::new(i2c_interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .await
        .expect("failed to initialize the display");

    let raw_image = ImageRaw::<BinaryColor>::new(IMG_DATA, 8);

    let image = Image::new(&raw_image, Point::zero());

    image
        .draw(&mut display)
        .expect("failed to draw text to display");

    display
        .flush()
        .await
        .expect("failed to flush data to display");

    loop {
        Timer::after_millis(100).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"byte-oled"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
