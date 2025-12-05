#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::{Duration, Timer};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

// For GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};

// For PWM
use embassy_rp::pwm::{Pwm, SetDutyCycle};

// For time calculation
use embassy_time::Instant;

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

    let max_duty = led.max_duty_cycle();
    // defmt::info!("Max duty cycle {}", max_duty);

    loop {
        Timer::after_millis(10).await;

        let distance = match measure_distance(&mut trigger, &echo).await {
            Some(d) => d,
            None => {
                Timer::after_secs(5).await;
                continue; // Skip to next iteration
            }
        };

        let duty_cycle = calculate_duty_cycle(distance, max_duty);
        led.set_duty_cycle(duty_cycle)
            .expect("duty cycle is within valid range");
    }
}

const ECHO_TIMEOUT: Duration = Duration::from_millis(100);

async fn measure_distance(trigger: &mut Output<'_>, echo: &Input<'_>) -> Option<f64> {
    // Send trigger pulse
    trigger.set_low();
    Timer::after_micros(2).await;
    trigger.set_high();
    Timer::after_micros(10).await;
    trigger.set_low();

    // Wait for echo HIGH (sensor responding)
    let timeout = Instant::now();
    while echo.is_low() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for HIGH");
            return None; // Return early on timeout
        }
    }

    let start = Instant::now();

    // Wait for echo LOW (pulse complete)
    let timeout = Instant::now();
    while echo.is_high() {
        if timeout.elapsed() > ECHO_TIMEOUT {
            defmt::warn!("Timeout waiting for LOW");
            return None; // Return early on timeout
        }
    }

    let end = Instant::now();

    // Calculate distance
    let time_elapsed = end.checked_duration_since(start)?.as_micros();
    let distance = time_elapsed as f64 * 0.0343 / 2.0;

    Some(distance)
}

const MAX_DISTANCE_CM: f64 = 30.0;

fn calculate_duty_cycle(distance: f64, max_duty: u16) -> u16 {
    if distance < MAX_DISTANCE_CM && distance >= 2.0 {
        let normalized = (MAX_DISTANCE_CM - distance) / MAX_DISTANCE_CM;
        // defmt::info!("duty cycle :{}", (normalized * max_duty as f64) as u16);
        (normalized * max_duty as f64) as u16
    } else {
        0
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ultrasonic"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
