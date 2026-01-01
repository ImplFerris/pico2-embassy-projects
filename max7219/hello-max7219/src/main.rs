#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// For MAX7219
use embedded_hal_bus::spi::ExclusiveDevice;

// For Drawing shapes
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

// For SPI
use embassy_rp::spi::{Config as SpiConfig, Spi};

// For CS Pin
use embassy_rp::gpio::{Level, Output};

use max7219_display::led_matrix::display::SingleMatrix;
//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let cs_pin = Output::new(p.PIN_13, Level::High);

    let clk = p.PIN_14;
    let mosi = p.PIN_15;

    let spi_bus = Spi::new_blocking_txonly(p.SPI1, clk, mosi, SpiConfig::default());
    let spi_dev =
        ExclusiveDevice::new_no_delay(spi_bus, cs_pin).expect("Failed to get exclusive device");

    // Create a display instance for a single 8x8 LED matrix (not daisy-chained)
    let mut display = SingleMatrix::from_spi(spi_dev).expect("display count 1 should not panic");

    // Set brightness (intensity level) of the only device at index 0
    display
        .driver()
        .set_intensity(0, 1)
        .expect("failed to set intensity");

    // ---- Draw Rectangle ----
    // let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(
    //     embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
    // );
    let hollow_rect_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On) // Only draw the border
        .stroke_width(1) // Border thickness of 1 pixel
        .build();
    let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(hollow_rect_style);
    rect.draw(&mut display).expect("failed to draw the shape");

    display.flush().expect("failed to send to the device");

    loop {
        Timer::after_millis(100).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"hello-max7219"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
