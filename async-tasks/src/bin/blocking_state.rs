#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Delay, Duration, Instant};
use embedded_hal::delay::DelayNs;

// mod async_tasks;
// mod blocking;

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

    let mut led = Output::new(p.PIN_25, Level::Low);
    let button = Input::new(p.PIN_15, Pull::Up);

    let mut blink_enabled = true;
    let mut delay = Delay;

    let mut last_toggle_time = Instant::now();
    let mut last_button_state = false;

    let blink_interval = Duration::from_millis(300);

    loop {
        let now = Instant::now();

        let button_pressed = button.is_low();
        if button_pressed && !last_button_state {
            blink_enabled = !blink_enabled;
            if !blink_enabled {
                led.set_low();
            }
        }
        last_button_state = button_pressed;

        if blink_enabled && now.duration_since(last_toggle_time) >= blink_interval {
            led.toggle();
            last_toggle_time = now;
        }

        delay.delay_ms(10);
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"async-tasks"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
