use esp_idf_svc::{
    hal::{
        delay::{FreeRtos, BLOCK},
        i2c::{I2cConfig, I2cDriver},
        prelude::Peripherals,
        units::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};
use nobcd::BcdNumber;

const DS_ADDRESS: u8 = 0x68;

enum TimeElements {
    Seconds = 0,
    Minutes,
    Hours,
    Day,
    Month,
    Year,
    DayOfWeek,
}

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio4;
    let config = I2cConfig::new().baudrate(100_u32.kHz().into());
    let mut ds = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let start_time: [u8; 7] = [0, 15, 12, 19, 2, 25, 3];

    for (idx, element) in start_time.iter().enumerate() {
        let bytes: [u8; 1] = BcdNumber::new(*element).unwrap().bcd_bytes();
        ds.write(DS_ADDRESS, &[idx as u8, bytes[0]], BLOCK).unwrap();
    }

    loop {
        let mut buffer: [u8; 7] = [0_u8; 7];
        ds.write(DS_ADDRESS, &[0_u8], BLOCK).unwrap();
        ds.read(DS_ADDRESS, &mut buffer, BLOCK).unwrap();
        let seconds: u8 = BcdNumber::from_bcd_bytes([buffer[0]]).unwrap().value();
        let minutes: u8 = BcdNumber::from_bcd_bytes([buffer[1]]).unwrap().value();
        let hour: u8 = BcdNumber::from_bcd_bytes([buffer[2]]).unwrap().value();
        log::info!("{hour}:{minutes}:{seconds}");
        FreeRtos::delay_ms(1_000);
    }
}
