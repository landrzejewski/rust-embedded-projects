use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::{delay::FreeRtos, prelude::Peripherals}, http::{server::{Configuration as HttpServerConfig, EspHttpServer}, Method}, log::EspLogger, nvs::EspDefaultNvsPartition, sys::link_patches, wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi}
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

    wifi.wait_netif_up()?;

    let mut server = EspHttpServer::new(&HttpServerConfig::default())?;
    server.fn_handler("/api", Method::Get, |request| {
        let mut response = request.into_ok_response()?;
        response.write(index_page().as_bytes())?;
        Ok::<(), anyhow::Error>(())
    })?;

    loop {
        FreeRtos::delay_ms(1_000);
    }
}


fn index_page() -> String {
    format!(r#"
        <html>
            <head><title>Rust</title></head>
            <body>
                Hello!
            </body>
        </html>
    "#)
}
