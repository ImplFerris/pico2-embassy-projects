#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::{Instant, Timer};

// For GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};

// For PWM
use embassy_rp::pwm::{Pwm, SetDutyCycle};

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

    // For Onboard LED
    // let mut led = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());

    // For external LED connected on GPIO 3
    let mut led = Pwm::new_output_b(p.PWM_SLICE1, p.PIN_3, Default::default());

    let mut trigger = Output::new(p.PIN_17, Level::Low);
    let echo = Input::new(p.PIN_16, Pull::Down);

    led.set_duty_cycle(0)
        .expect("duty cycle is within valid range");

    loop {
        Timer::after_millis(5).await;

        trigger.set_low();
        Timer::after_micros(2).await;
        trigger.set_high();
        Timer::after_micros(10).await;
        trigger.set_low();

        while echo.is_low() {}
        let start = Instant::now();
        while echo.is_high() {}
        let end = Instant::now();

        let time_elapsed = end.checked_duration_since(start).unwrap().as_micros();

        let distance = time_elapsed as f64 * 0.0343 / 2.0;

        let duty_cycle = if distance < 30.0 {
            let step = 30.0 - distance;
            (step * 1500.) as u16 + 1000
        } else {
            0
        };
        led.set_duty_cycle(duty_cycle).unwrap();
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ultrsonic"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
