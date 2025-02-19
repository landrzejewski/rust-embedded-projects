#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{delay::Delay, gpio::{Level, Output}, main, Config};
use esp_println::logger;

const DELAY: u32 = 1_000;

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());
    let mut led_pin = Output::new(peripherals.GPIO1, Level::Low);
    let delay = Delay::new();
    loop {
        led_pin.set_high();
        delay.delay_millis(DELAY);
        led_pin.set_low();
        delay.delay_millis(DELAY);
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}