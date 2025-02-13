#![no_std]
#![no_main]

use core::default::Default;
use core::fmt::{self, Display};
use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    delay::MicrosDurationU64,
    peripherals::TIMG0,
    prelude::*,
    timer::timg::{Timer, Timer0, TimerGroup},
    Config,
};
use esp_println::println;

static TIMER: Mutex<RefCell<Option<Timer<Timer0<TIMG0>, esp_hal::Blocking>>>> = Mutex::new(RefCell::new(None));
static FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[derive(Default)]
struct Time {
    seconds: u8,
    minutes: u8,
    hours: u8,
}

impl Time {
    fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds > 59 {
            self.minutes += 1;
        }
        if self.minutes > 59 {
            self.hours += 1;
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

#[handler]
fn on_tick() {
    critical_section::with(|cs| {
        TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .set_alarm_active(true);
        FLAG.borrow(cs).set(true);
    });
}

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(Config::default());
    let timer_group = TimerGroup::new(peripherals.TIMG0);
    let timer = timer_group.timer0;

    timer
        .load_value(MicrosDurationU64::micros(1_000_000))
        .unwrap();
    timer.set_alarm_active(true);
    timer.set_counter_active(true);
    timer.set_interrupt_handler(on_tick);
    timer.listen();

    critical_section::with(|cs| {
        TIMER.borrow_ref_mut(cs).replace(timer)
    });


    let mut time = Time::default();

    loop {
        critical_section::with(|cs|{
            if FLAG.borrow(cs).get() {
                time.tick();
                println!("{time}\r");
                FLAG.borrow(cs).set(false);
            }
        });
    }
}
