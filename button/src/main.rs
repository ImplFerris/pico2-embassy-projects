#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::Pull;
use embassy_rp::{
    self as hal,
    gpio::{Input, Level, Output},
};
use embassy_time::Timer;

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

    let button = Input::new(p.PIN_15, Pull::Up);
    // let mut led = Output::new(p.PIN_16, Level::Low);
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        if button.is_low() {
            defmt::info!("Button pressed");
            led.set_high();
            Timer::after_secs(3).await;
        } else {
            led.set_low();
        }
        Timer::after_millis(5).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"button"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
