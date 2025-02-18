use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{hal::{gpio::{InterruptType, PinDriver, Pull}, prelude::Peripherals}, log::EspLogger, sys::link_patches};

static FLAG: AtomicBool = AtomicBool::new(false);

fn on_press() {
    FLAG.store(true, Ordering::Relaxed);
}

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut button_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();
    button_pin.set_pull(Pull::Up).unwrap();
    button_pin.set_interrupt_type(InterruptType::NegEdge).unwrap();
    unsafe { button_pin.subscribe(on_press).unwrap() }
    button_pin.enable_interrupt().unwrap();

    let mut counter = 0_u32;

    loop {
        if FLAG.load(Ordering::Relaxed) {
            counter = counter + 1;
            log::info!("Press count: {counter}");
            FLAG.store(false, Ordering::Relaxed);
            button_pin.enable_interrupt().unwrap();
        }
    }
}



