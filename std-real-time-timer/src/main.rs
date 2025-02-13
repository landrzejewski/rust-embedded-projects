use std::{fmt, ops::Add, sync::atomic::{AtomicBool, Ordering}};

use esp_idf_svc::{
    hal::{
        prelude::Peripherals,
        timer::{config::Config, TimerDriver},
    }, log::EspLogger, sys::link_patches
};
use std::fmt::Display;

static FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
struct Time {
    seconds: u8,
    minutes: u8,
    hours: u8
}

impl Time {

    fn tick(&mut self) {
        self.seconds = self.seconds.add(1);
        if self.seconds > 59 {
            self.minutes = self.minutes.add(1);
        }
        if self.minutes > 59 {
            self.hours = self.hours.add(1);
        }
        if self.hours > 24 {
            self.seconds = 0;
            self.minutes = 0;
            self.hours = 0;
        }
    }

}

impl Display for Time {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hours, self.minutes, self.seconds)
    }
    
}

fn on_tick() {
    FLAG.store(true, Ordering::Relaxed);
}

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let config = Config::new().auto_reload(true);
    let mut timer1 = TimerDriver::new(peripherals.timer00, &config).unwrap();
    timer1.set_alarm(timer1.tick_hz()).unwrap();
    unsafe { timer1.subscribe(on_tick).unwrap() }
    timer1.enable_interrupt().unwrap();
    timer1.enable_alarm(true).unwrap();
    timer1.enable(true).unwrap();

    let mut time = Time::default();

    loop {
        if FLAG.load(Ordering::Relaxed) {
            FLAG.store(false, Ordering::Relaxed);
            time.tick();
            log::info!("{time}");
        }
    }
}
