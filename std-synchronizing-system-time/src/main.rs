use std::time::SystemTime;

use chrono::{DateTime, FixedOffset, Local, Utc};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::{delay::FreeRtos, prelude::Peripherals}, log::EspLogger, nvs::EspDefaultNvsPartition, sntp::{EspSntp, SyncStatus}, sys::link_patches, wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi}
};

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let system_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take().ok();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, system_loop.clone(), nvs)?,
        system_loop.clone(),
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

    let sntp = EspSntp::new_default()?;
    while sntp.get_sync_status() != SyncStatus::Completed {
        log::info!("Synchronizing...");
        FreeRtos::delay_ms(1_000);
    }
    loop {
        let system_time = SystemTime::now();
        let local_time: DateTime<Local> = system_time.clone().into();
        let utc_time: DateTime<Utc> = system_time.clone().into();
        let offset = FixedOffset::east_opt(3600).unwrap();
        log::info!("Local: {}", local_time.with_timezone(&offest));
        log::info!("Utc: {utc_time}");
        FreeRtos::delay_ms(5_000);
    }
}
