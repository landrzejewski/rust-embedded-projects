#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{delay::Delay, i2c::master::{self, I2c}, main, Config};
use esp_println::logger;
use nobcd::BcdNumber;

const DS_ADDR: u8 = 0x68;

struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    month: u8,
    year: u8,
    day_of_week: u8,
}

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let mut ds = I2c::new(peripherals.I2C0, master::Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO5)
        .with_scl(peripherals.GPIO4);

    let start_time = DateTime {
        seconds: 0,
        minutes: 15,
        hours: 12,
        day: 1,
        month: 1,
        year: 4,
        day_of_week: 4,
    };

    let seconds_bytes: [u8; 1] = BcdNumber::new(start_time.seconds).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[0, seconds_bytes[0]]).unwrap();

    let minutes_bytes: [u8; 1] = BcdNumber::new(start_time.minutes).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[1, minutes_bytes[0]]).unwrap();

    let hours_bytes: [u8; 1] = BcdNumber::new(start_time.hours).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[2, hours_bytes[0]]).unwrap();

    let day_bytes: [u8; 1] = BcdNumber::new(start_time.day).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[3, day_bytes[0]]).unwrap();

    let week_day_bytes: [u8; 1] = BcdNumber::new(start_time.day_of_week).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[4, week_day_bytes[0]]).unwrap();

    let month_bytes: [u8; 1] = BcdNumber::new(start_time.month).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[5, month_bytes[0]]).unwrap();

    let year_bytes: [u8; 1] = BcdNumber::new(start_time.year).unwrap().bcd_bytes();
    ds.write(DS_ADDR, &[6, year_bytes[0]]).unwrap();

    let delay = Delay::new();

    loop {
        let mut buffer: [u8; 7] = [0_u8; 7];
        ds.write(DS_ADDR, &[0_u8]).unwrap();
        ds.read(DS_ADDR, &mut buffer).unwrap();
        let seconds = BcdNumber::from_bcd_bytes([buffer[0]])
            .unwrap()
            .value::<u8>();
        let minutes = BcdNumber::from_bcd_bytes([buffer[1]])
            .unwrap()
            .value::<u8>();
        let hours = BcdNumber::from_bcd_bytes([buffer[2]])
            .unwrap()
            .value::<u8>();
        log::info!("{hours}:{minutes}:{seconds}\r");
        delay.delay_millis(1_000);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
