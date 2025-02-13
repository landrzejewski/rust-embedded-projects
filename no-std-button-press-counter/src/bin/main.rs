#![no_std]
#![no_main]

use core::{cell::{Cell, RefCell}, panic::PanicInfo};

use critical_section::Mutex;
use esp_hal::{gpio::{Event, Input, Io, Pull}, handler, interrupt::InterruptConfigurable, main, Config};
use esp_println::{logger, println};

static FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false)); 
static PIN: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

#[handler]
fn on_press() {
    critical_section::with(|cs| {
        PIN.borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        FLAG.borrow(cs).set(true);
    });
}

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let peripherals = esp_hal::init(Config::default());

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(on_press);
    
    let mut button_pin = Input::new(peripherals.GPIO4, Pull::Up);
    button_pin.listen(Event::FallingEdge);
    critical_section::with(|cs| PIN.borrow_ref_mut(cs).replace(button_pin));
    let mut counter = 0;
    loop {
      critical_section::with(|cs| {
        if FLAG.borrow(cs).get() {
            counter += 1;
            println!("Counter: {counter}\r");
            FLAG.borrow(cs).set(false);
        }
      })
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}