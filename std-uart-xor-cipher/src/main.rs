use std::str::from_utf8;

use esp_idf_svc::{
    hal::{
        delay::BLOCK,
        gpio::{Gpio0, Gpio1},
        prelude::Peripherals,
        uart::{config::Config as UartConfig, UartDriver},
        units::Hertz,
    },
    log::EspLogger,
    sys::link_patches,
};

const MESSAGE: &str = "Embedded Rust";
const KEY: u8 = 200;

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio10;
    let rx = peripherals.pins.gpio11;

    let config = UartConfig::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<Gpio0>::None,
        Option::<Gpio1>::None,
        &config,
    )
        .unwrap();

    log::info!("Message: {MESSAGE}");
    log::info!("Message bytes: {:?}", MESSAGE.as_bytes());

    let encrypted_message_bytes: Vec<u8> =
        MESSAGE.as_bytes().iter().map(|byte| byte ^ KEY).collect();

    log::info!("Encrypted message bytes: {:?}", encrypted_message_bytes);

    let mut received_bytes = Vec::new();

    for byte in encrypted_message_bytes.iter() {
        uart.write(&[*byte]).unwrap();
        let mut buffer = [0_u8; 1];
        uart.read(&mut buffer, BLOCK).unwrap();
        received_bytes.extend_from_slice(&buffer);
    }

    let decrypted_message_bytes: Vec<u8> = received_bytes.iter().map(|byte| byte ^ KEY).collect();

    if let Ok(message) = from_utf8(&decrypted_message_bytes) {
        log::info!("Received message: {message}");
    }
}
