#![no_std]
#![no_main]

use core::{cell::Cell, panic::PanicInfo, sync::atomic::{AtomicBool, Ordering}};

use critical_section::Mutex;
use esp_hal::{delay::Delay, gpio::{Event, Input, Io, Level, Output, Pull}, handler, interrupt::InterruptConfigurable, main, Config};
use esp_println::logger;

const INITIAL_DELAY_IN_MILLS: u32 = 200;
const DELAY_CHANGE_IN_MILLS: u32 = 1_000;

static FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static SIMPLE_FLAG: AtomicBool = AtomicBool::new(false);

#[handler]
fn on_press() {
    SIMPLE_FLAG.store(true, Ordering::Relaxed);
    //critical_section::with(|cs| FLAG.borrow(cs).set(true));
}

#[main]
fn main() -> ! {
    logger::init_logger_from_env();

    let peripherals = esp_hal::init(Config::default());
    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(on_press);

    let mut button_pin = Input::new(peripherals.GPIO0, Pull::Up);
    button_pin.listen(Event::AnyEdge);

    let mut led_pin = Output::new(peripherals.GPIO4, Level::Low);

    let delay = Delay::new();
    let mut delay_value = INITIAL_DELAY_IN_MILLS;

    loop {
        // critical_section::with(|cs| {
        //     if FLAG.borrow(cs).get() {
        //         delay_value += DELAY_CHANGE_IN_MILLS;
        //         FLAG.borrow(cs).set(false);
        //     }
        // });
        if SIMPLE_FLAG.load(Ordering::Relaxed) {
            delay_value += DELAY_CHANGE_IN_MILLS;
            SIMPLE_FLAG.store(false, Ordering::Relaxed);
        }

        led_pin.set_high();
        delay.delay_millis(delay_value);
        led_pin.set_low();
        delay.delay_millis(INITIAL_DELAY_IN_MILLS);
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}