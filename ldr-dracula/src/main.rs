#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// Interrupt Binding
use embassy_rp::adc::InterruptHandler;
use embassy_rp::bind_interrupts;

// ADC
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig};

// For LED
use embassy_rp::gpio::{Level, Output, Pull};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

const LDR_THRESHOLD: u16 = 200;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());

    let mut adc_pin = Channel::new_pin(p.PIN_28, Pull::None);
    let mut led = Output::new(p.PIN_15, Level::Low);

    loop {
        let adc_reading = adc
            .read(&mut adc_pin)
            .await
            .expect("Unable to read the adc value");
        defmt::info!("ADC value: {}", adc_reading);
        if adc_reading < LDR_THRESHOLD {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after_secs(1).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ldr-dracula"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
