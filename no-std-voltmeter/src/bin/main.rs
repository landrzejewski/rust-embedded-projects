#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation}, delay::Delay, main, Config
};
use esp_println::logger;
use nb::block;

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let mut config = AdcConfig::new();
    let mut channel_config = config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut channel = Adc::new(peripherals.ADC1, config);
    let delay = Delay::new();
    loop {
        let raw_value = block!(channel.read_oneshot(&mut channel_config)).unwrap();
        log::info!("Raw value: {raw_value}\r");
        delay.delay_millis(1_000);
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}