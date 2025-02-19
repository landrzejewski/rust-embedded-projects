#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{gpio::{Level, Output}, main, timer::{timg::TimerGroup, Timer}, Config};
use esp_println::logger;

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let mut let_pin = Output::new(peripherals.GPIO0, Level::Low);
    let timer_group = TimerGroup::new(peripherals.TIMG0);
    let timer= timer_group.timer0;
    timer.start();
    let mut start = timer.now();
    loop {
        if timer.now().checked_duration_since(start).unwrap().to_secs() > 5 {
            let_pin.toggle();
            start = timer.now();
        }
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}