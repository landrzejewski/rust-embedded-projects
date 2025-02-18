use std::{fmt::{self, Display, Formatter}, sync::atomic::{AtomicBool, Ordering}};

use esp_idf_svc::{hal::{prelude::Peripherals, timer::{self, config::Config, TimerDriver}}, log::EspLogger, sys::link_patches};

#[derive(Default)]
struct Timer {
    seconds: u8,
    minutes: u8,
    hours: u8
}

impl Timer {

    fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds > 59 {
            self.minutes += 1;
            self.seconds = 0;
        }
        if self.minutes > 59 {
            self.hours += 1;
            self.minutes = 0;
        }
        if self.hours > 24 {
            self.seconds = 0;
            self.minutes = 0;
            self.hours = 0;
        }
    }

}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hours, self.minutes, self.seconds)
    }
}

static FLAG: AtomicBool = AtomicBool::new(false);

fn on_tick() {
    FLAG.store(true, Ordering::Relaxed);
}

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let config = timer::config::Config::new().auto_reload(true);
    let mut timer = TimerDriver::new(peripherals.timer00, &config).unwrap();
    timer.set_alarm(timer.tick_hz()).unwrap();
    unsafe { timer.subscribe(on_tick).unwrap() }
    timer.enable_interrupt().unwrap();
    timer.enable_alarm(true).unwrap();
    timer.enable(true).unwrap();

    let mut my_timer = Timer::default();
    loop {
        if FLAG.load(Ordering::Relaxed) {
            my_timer.tick();
            FLAG.store(false, Ordering::Relaxed);
            log::info!("{my_timer}");
        }
    }


}