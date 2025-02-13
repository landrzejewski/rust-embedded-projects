use std::str::from_utf8;

use esp_idf_svc::{hal::{delay::BLOCK, gpio, prelude::Peripherals, uart::{config::Config, UartDriver}, units::Hertz}, log::EspLogger, sys::link_patches};

const MESSAGE: &str = "Rust embedded";
const KEY: u8 = 200;

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherials = Peripherals::take().unwrap();
    let tx = peripherials.pins.gpio10;
    let rx = peripherials.pins.gpio11;
    let config = Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherials.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config
    ).unwrap();

    let mut received_message = Vec::new();
    log::info!("Message: {MESSAGE}");
    log::info!("Message bytes: {:?}", MESSAGE.as_bytes());
    let encrypted_message: Vec<u8> = MESSAGE.as_bytes()
        .iter()
        .map(|b| b ^ KEY)
        .collect();
    log::info!("Encrypted message: {:?}", encrypted_message);
    for byte in encrypted_message.iter() {
        uart.write(&[*byte]).unwrap();
        let mut buffer = [0_u8; 1];
        uart.read(&mut buffer, BLOCK).unwrap();
        received_message.extend_from_slice(&buffer);
    }
    let decrypted_message: Vec<u8> = received_message
        .iter()
        .map(|b| b ^ KEY)
        .collect();
    if let Ok(msg) = from_utf8(&decrypted_message) {
        log::info!("Recived message: {msg}");
    }
}
