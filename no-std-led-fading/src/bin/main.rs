#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp_hal::{
    delay::Delay,
    ledc::{
        channel::{self, ChannelIFace, Number},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::RateExtU32,
    Config,
};
use esp_println::logger;

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let led_pin = peripherals.GPIO7;

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);

    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 1u32.kHz(),
        })
        .unwrap();

    let mut channel = ledc.channel(Number::Channel0, led_pin);
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let min_duty = 0_u8;
    let max_duty = 100_u8;

    let delay = Delay::new();
    loop {
        for duty in min_duty..max_duty {
            channel.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }

        for duty in (min_duty..max_duty).rev() {
            channel.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}