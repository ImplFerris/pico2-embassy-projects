#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;

use embassy_futures::select::{Either, select};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

// Shared signal to communicate between tasks
static BLINK_SIGNAL: Signal<ThreadModeRawMutex, bool> = Signal::new();

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static>) {
    let mut is_blinking = true;

    loop {
        match select(BLINK_SIGNAL.wait(), Timer::after_millis(300)).await {
            Either::First(new_state) => {
                is_blinking = new_state;
                if !is_blinking {
                    led.set_low();
                }
            }
            Either::Second(_) => {
                if is_blinking {
                    led.toggle();
                }
            }
        }
    }
}

#[embassy_executor::task]
async fn handle_button(button: Input<'static>) {
    let mut is_blinking = true;

    loop {
        if button.is_low() {
            is_blinking = !is_blinking;
            BLINK_SIGNAL.signal(is_blinking);
            Timer::after_millis(200).await;
        }
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let led = Output::new(p.PIN_25, Level::Low);
    let button = Input::new(p.PIN_15, Pull::Up);

    spawner
        .spawn(blink_led(led))
        .expect("failed to spawn led task");
    spawner
        .spawn(handle_button(button))
        .expect("failed to spawn button task");

    loop {
        Timer::after_millis(100).await;
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
