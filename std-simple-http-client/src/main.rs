use std::error::Error;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::prelude::Peripherals, http::{
        client::{Configuration as HttpConfig, EspHttpConnection},
        Method,
    }, log::EspLogger, nvs::EspDefaultNvsPartition, sys::{esp_crt_bundle_attach, link_patches}, wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi}
};

const BUFFER_SIZE: usize = 512;

fn main() -> Result<(), Box<dyn Error>> {
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

    wifi.wait_netif_up()?;
    let mut connection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_crt_bundle_attach),
        ..Default::default()
    })?;
    _ = connection.initiate_request(Method::Get, "https://httpbin.org/get", &[])?;
    _ = connection.initiate_response()?;

    let status = connection.status();
    log::info!("Status: {status}");
    let content_length = connection.header("Content-Length").unwrap_or("unknown");
    log::info!("Content length: {content_length}");

    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let bytes_read = connection.read(&mut buffer)?;
        if  bytes_read == 0 {
            break;
        }
        let chunk = &buffer[..bytes_read];
        let text = String::from_utf8_lossy(chunk);
        log::info!("{text}");
    }
    Ok(())
}
