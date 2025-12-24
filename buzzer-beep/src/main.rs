#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
use embassy_time::Timer;

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

const fn get_top(freq: f64, div_int: u8) -> u16 {
    assert!(div_int != 0, "Divider must not be 0");

    let result = 150_000_000. / (freq * div_int as f64);

    assert!(result >= 1.0, "Frequency too high");
    assert!(
        result <= 65535.0,
        "Frequency too low: TOP exceeds 65534 max"
    );

    result as u16 - 1
}

const PWM_DIV_INT: u8 = 64;
const PWM_TOP: u16 = get_top(440., PWM_DIV_INT);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut pwm_config = PwmConfig::default();
    pwm_config.top = PWM_TOP;
    pwm_config.divider = PWM_DIV_INT.into();

    let mut buzzer = Pwm::new_output_b(p.PWM_SLICE7, p.PIN_15, pwm_config);

    loop {
        buzzer
            .set_duty_cycle_percent(50)
            .expect("50 is valid duty percentage");
        Timer::after_millis(1000).await;

        buzzer
            .set_duty_cycle_percent(0)
            .expect("0 is valid duty percentage");
        Timer::after_millis(1000).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"buzzer-beep"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
