use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos, gpio::{IOPin, InterruptType, PinDriver, Pull}, prelude::Peripherals
    },
    log::EspLogger,
    sys::link_patches,
};

const INITIAL_DELAY_IN_MILLS: u32 = 100;
const DELAY_CHANGE_IN_MILLIS: u32 = 200;

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led_pins = [
        PinDriver::output(peripherals.pins.gpio4.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio5.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio6.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio7.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio15.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio16.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio17.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio18.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio8.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio3.downgrade()).unwrap(),
    ];

    let mut button_pin = PinDriver::input(peripherals.pins.gpio1).unwrap();
    button_pin.set_pull(Pull::Up).unwrap();
    button_pin
        .set_interrupt_type(InterruptType::NegEdge)
        .unwrap();
    unsafe { button_pin.subscribe(on_press).unwrap() }
    button_pin.enable_interrupt().unwrap();

    let mut delay = INITIAL_DELAY_IN_MILLS;

    loop {
        for led_pin in &mut led_pins {
            if FLAG.load(Ordering::Relaxed) {
                delay += DELAY_CHANGE_IN_MILLIS;
                FLAG.store(false, Ordering::Relaxed);
                button_pin.enable_interrupt().unwrap();
            }
            led_pin.set_high().unwrap();
            FreeRtos::delay_ms(delay);
            led_pin.set_low().unwrap();
            FreeRtos::delay_ms(INITIAL_DELAY_IN_MILLS);
        }
    }

}

fn on_press() {
    FLAG.store(true, Ordering::Relaxed);
}
