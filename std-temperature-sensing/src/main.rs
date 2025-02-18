use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{config::{AdcChannelConfig, Calibration}, AdcChannelDriver, AdcDriver},
            Resolution,
        },
        delay::FreeRtos,
        prelude::Peripherals,
    }, log::EspLogger, sys::link_patches
};
use libm::log;

const B: f64 = 3950.0;
const V_MAX: f64 = 4095.0;

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();
    let pin = peripherals.pins.gpio4;
    let channel_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit,
    };
    let mut channel = AdcChannelDriver::new(&adc1, pin, &channel_config).unwrap();
    loop {
        let raw_value = channel.read_raw().unwrap();
        let temperature =
            1. / (log(1. / (V_MAX / raw_value as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;
        log::info!("Temperature: {:.2} celcius", temperature);
        FreeRtos::delay_ms(1_000);
    }
}
