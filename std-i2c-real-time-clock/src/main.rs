use esp_idf_svc::{hal::{delay::{FreeRtos, BLOCK}, i2c::*, prelude::Peripherals, units::FromValueType}, log::EspLogger, sys::link_patches};
use nobcd::BcdNumber;

const DS_ADDR: u8 = 0x68;

struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    month: u8,
    year: u8,
    day_of_week: u8
}

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherials = Peripherals::take().unwrap();
    let i2c = peripherials.i2c0;
    let sda = peripherials.pins.gpio5;
    let scl = peripherials.pins.gpio4;
    let config = I2cConfig::new().baudrate(100_u32.kHz().into());
    let mut ds = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let start_time = DateTime {
        seconds: 0,
        minutes: 15,
        hours: 12,
        day: 1,
        month: 1,
        year: 4,
        day_of_week: 4
    };

    let seconds_bytes: [u8; 1] = BcdNumber::new(start_time.seconds).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[0, seconds_bytes[0]], BLOCK).unwrap();

    let minutes_bytes: [u8; 1] = BcdNumber::new(start_time.minutes).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[1, minutes_bytes[0]], BLOCK).unwrap();

    let hours_bytes: [u8; 1] = BcdNumber::new(start_time.hours).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[2, hours_bytes[0]], BLOCK).unwrap();

    let day_bytes: [u8; 1] = BcdNumber::new(start_time.day).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[3, day_bytes[0]], BLOCK).unwrap();

    let week_day_bytes: [u8; 1] = BcdNumber::new(start_time.day_of_week).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[4, week_day_bytes[0]], BLOCK).unwrap();

    let month_bytes: [u8; 1] = BcdNumber::new(start_time.month).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[5, month_bytes[0]], BLOCK).unwrap();

    let year_bytes: [u8; 1] = BcdNumber::new(start_time.year).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[6, year_bytes[0]], BLOCK).unwrap();

    loop {
        let mut buffer: [u8; 7] = [0_u8; 7];
        ds.write(DS_ADDR, &[0_u8], BLOCK).unwrap();
        ds.read(DS_ADDR, &mut buffer, BLOCK).unwrap();
        let seconds = BcdNumber::from_bcd_bytes([buffer[0]]).unwrap().value::<u8>();
        let minutes = BcdNumber::from_bcd_bytes([buffer[1]]).unwrap().value::<u8>();
        let hours = BcdNumber::from_bcd_bytes([buffer[2]]).unwrap().value::<u8>();
        log::info!("{hours}:{minutes}:{seconds}");
        FreeRtos::delay_ms(1_000);
    }

}
