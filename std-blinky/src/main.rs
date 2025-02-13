use esp_idf_svc::{hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals}, sys::link_patches};

const DELAY: u32 = 3_000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    // EspLogger::initialize_default();
    // log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pin1 = peripherals.pins.gpio1;
    let mut led_pin = PinDriver::output(pin1).unwrap();
    loop {
        led_pin.set_high().unwrap();
        FreeRtos::delay_ms(DELAY);
        led_pin.set_low().unwrap();
        FreeRtos::delay_ms(DELAY);
    }
}
