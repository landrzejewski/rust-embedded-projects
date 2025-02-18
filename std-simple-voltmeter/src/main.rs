use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{
                config::{AdcChannelConfig, Calibration},
                AdcChannelDriver, AdcDriver,
            },
            Resolution,
        },
        delay::FreeRtos,
        prelude::Peripherals,
    },
    log::EspLogger,
    sys::link_patches,
};

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
        let value = channel.read().unwrap();
        log::info!("Raw value: {raw_value}, value: {value} mV");
        FreeRtos::delay_ms(1_000);
    }
}
