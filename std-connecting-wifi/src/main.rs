use std::error::Error;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::prelude::Peripherals, log::EspLogger, nvs::EspDefaultNvsPartition, sys::link_patches, wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi}
};

fn main() -> Result<(), Box<dyn Error>> {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let system_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().ok();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, system_loop.clone(), nvs)?,system_loop.clone()
    )?;
    let wifi_config: ClientConfiguration = ClientConfiguration {
        ssid: "Wokwi-GUEST".try_into().unwrap(),
        password: "".try_into().unwrap(),
        auth_method: esp_idf_svc::wifi::AuthMethod::None,
        ..Default::default()
    };
    wifi.set_configuration(&Configuration::Client(wifi_config))?;
    wifi.start()?;
    log::info!("Is connected: {}", wifi.is_connected()?);
    wifi.connect()?;
    while !wifi.is_connected()? {
        log::info!("Connecting...");
    }
    log::info!("Connected");
    Ok(())
}
