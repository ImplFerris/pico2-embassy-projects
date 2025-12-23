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

// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

const PWM_DIV_INT: u8 = 64;
const PWM_TOP: u16 = 46_874;

// Alternative method:
// const TOP: u16 = PWM_TOP + 1;
// const MIN_DUTY: u16 = (TOP as f64 * (2.5 / 100.)) as u16;
// const HALF_DUTY: u16 = (TOP as f64 * (7.5 / 100.)) as u16;
// const MAX_DUTY: u16 = (TOP as f64 * (12. / 100.)) as u16;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut servo_config: PwmConfig = Default::default();
    servo_config.top = PWM_TOP;
    servo_config.divider = PWM_DIV_INT.into();

    let mut servo = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, servo_config);

    loop {
        servo
            .set_duty_cycle_fraction(25, 1000)
            .expect("invalid min duty cycle");
        Timer::after_millis(1000).await;

        servo
            .set_duty_cycle_fraction(75, 1000)
            .expect("invalid half duty cycle");
        Timer::after_millis(1000).await;

        servo
            .set_duty_cycle_fraction(120, 1000)
            .expect("invalid max duty cycle");
        Timer::after_millis(1000).await;
    }

    // Alternative method
    // loop {
    //     servo
    //         .set_duty_cycle(MIN_DUTY)
    //         .expect("invalid min duty cycle");
    //     Timer::after_millis(1000).await;

    //     servo
    //         .set_duty_cycle(HALF_DUTY)
    //         .expect("invalid half duty cycle");
    //     Timer::after_millis(1000).await;

    //     servo
    //         .set_duty_cycle(MAX_DUTY)
    //         .expect("invalid max duty cycle");
    //     Timer::after_millis(1000).await;
    // }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"servo-motor"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
