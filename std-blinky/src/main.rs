use esp_idf_svc::{hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals}, log::EspLogger, sys::link_patches};

const DELAY: u32 = 3_000;

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin = peripherals.pins.gpio1;
    let mut  led_pin = PinDriver::output(pin).unwrap();
    loop {
        // led_pin.set_high().unwrap();
        // FreeRtos::delay_ms(DELAY);
        // led_pin.set_low().unwrap();
        // FreeRtos::delay_ms(DELAY);

        led_pin.toggle().unwrap();
        FreeRtos::delay_ms(DELAY);
        log::info!("Toggle");
    }

}