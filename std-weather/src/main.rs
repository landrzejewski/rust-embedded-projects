use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        gpio::{IOPin, InterruptType, PinDriver, Pull},
        prelude::Peripherals,
    },
    http::{
        client::{Configuration as HttpConfig, EspHttpConnection},
        server::{Configuration as HttpServerConfig, EspHttpServer},
        Method,
    },
    io::EspIOError,
    log::EspLogger,
    nvs::EspDefaultNvsPartition,
    sys::{esp_crt_bundle_attach, link_patches},
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};

const BUFFER_SIZE: usize = 512;
static FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led_pins = [
        PinDriver::output(peripherals.pins.gpio4.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio5.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio6.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio7.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio15.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio16.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio17.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio18.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio8.downgrade()).unwrap(),
        PinDriver::output(peripherals.pins.gpio3.downgrade()).unwrap(),
    ];

    let mut button_pin = PinDriver::input(peripherals.pins.gpio1).unwrap();
    button_pin.set_pull(Pull::Up).unwrap();
    button_pin
        .set_interrupt_type(InterruptType::NegEdge)
        .unwrap();
    unsafe { button_pin.subscribe(on_press).unwrap() }
    button_pin.enable_interrupt().unwrap();

    let system_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().ok();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, system_loop.clone(), nvs).unwrap(),
        system_loop.clone(),
    )
        .unwrap();
    let wifi_config: ClientConfiguration = ClientConfiguration {
        ssid: "Wokwi-GUEST".try_into().unwrap(),
        password: "".try_into().unwrap(),
        auth_method: esp_idf_svc::wifi::AuthMethod::None,
        ..Default::default()
    };
    wifi.set_configuration(&Configuration::Client(wifi_config))
        .unwrap();
    wifi.start().unwrap();
    log::info!("Is connected: {}", wifi.is_connected().unwrap());
    wifi.connect().unwrap();
    while !wifi.is_connected().unwrap() {
        log::info!("Connecting...");
    }
    log::info!("Connected");

    wifi.wait_netif_up().unwrap();

    let mut connection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_crt_bundle_attach),
        ..Default::default()
    })
        .unwrap();

    let temperatures = get_temperatures(&mut connection);
    let active_led = Arc::new(Mutex::new(0));
    let active_led_http = active_led.clone();

    let mut server = EspHttpServer::new(&HttpServerConfig::default()).unwrap();
    server
        .fn_handler("/", Method::Get, move |request| {
            let led_index = *active_led_http.lock().unwrap();
            let html = temperature_info(temperatures[led_index]);
            request.into_ok_response()?.write(html.as_bytes()).unwrap();
            Ok::<(), EspIOError>(())
        })
        .unwrap();

    led_pins[0].set_high().unwrap();

    loop {
        if FLAG.load(Ordering::Relaxed) {
            let mut led_index = active_led.lock().unwrap();
            led_pins[*led_index].set_low().unwrap();
            *led_index = (*led_index + 1) % led_pins.len();
            led_pins[*led_index].set_high().unwrap();
            FLAG.store(false, Ordering::Relaxed);
            button_pin.enable_interrupt().unwrap();
        }
    }
}

fn on_press() {
    FLAG.store(true, Ordering::Relaxed);
}

fn get_temperatures(connection: &mut EspHttpConnection) -> Vec<f64> {
    _ = connection
        .initiate_request(Method::Get, "https://api.openweathermap.org/data/2.5/forecast/daily?cnt=14&units=metric&APPID=b933866e6489f58987b2898c89f542b8&q=warsaw", &[])
        .unwrap();
    _ = connection.initiate_response().unwrap();

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut all_bytes = Vec::new();

    loop {
        let bytes_read = connection.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        all_bytes.extend_from_slice(&buffer[..bytes_read]);
    }

    let json = String::from_utf8_lossy(&all_bytes);
    extract_day_temperatures(&json)
}

fn extract_day_temperatures(json: &str) -> Vec<f64> {
    let mut temps = Vec::new();

    // Find the start of the "list" array.
    if let Some(list_index) = json.find("\"list\":") {
        let list_part = &json[list_index..];

        let mut search_from = 0;
        while let Some(temp_index) = list_part[search_from..].find("\"temp\":{") {
            let temp_start = search_from + temp_index + "\"temp\":".len();

            // Find the end of the temp object (assumes no nested braces).
            if let Some(temp_end_offset) = list_part[temp_start..].find('}') {
                let temp_end = temp_start + temp_end_offset;
                let temp_object = &list_part[temp_start..=temp_end];

                // Look for the "day" value within the temp object.
                if let Some(day_index) = temp_object.find("\"day\":") {
                    let day_start = day_index + "\"day\":".len();
                    // Extract until the next comma or closing brace.
                    let rest = &temp_object[day_start..];
                    let day_end = rest
                        .find(|c: char| c == ',' || c == '}')
                        .unwrap_or(rest.len());
                    let day_str = rest[..day_end].trim();

                    // Convert the extracted string to f64.
                    if let Ok(day_temp) = day_str.parse::<f64>() {
                        temps.push(day_temp);
                    }
                }
                search_from = temp_end + 1;
            } else {
                break;
            }
        }
    }
    temps
}

fn temperature_info(temperature: f64) -> String {
    format!(
        r#"
        <html>
            <head><title>Rust</title></head>
            <body>
                {temperature}
            </body>
        </html>
    "#
    )
}
