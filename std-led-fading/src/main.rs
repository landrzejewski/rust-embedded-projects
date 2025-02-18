use esp_idf_svc::{
    hal::{
        delay::{Ets, FreeRtos},
        ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution::Bits14},
        prelude::Peripherals,
        units::FromValueType,
    }, log::EspLogger, sys::link_patches
};

fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(1000_u32.Hz())
            .resolution(Bits14),
    )
        .unwrap();
    let mut pwm = LedcDriver::new(
        peripherals.ledc.channel0,
        &timer_driver,
        peripherals.pins.gpio1,
    )
        .unwrap();

    let min_duty = 0;
    let max_duty = pwm.get_max_duty();

    pwm.set_duty(min_duty).unwrap();
    pwm.enable().unwrap();
    loop {
        for duty in min_duty..max_duty {
            pwm.set_duty(duty).unwrap();
            Ets::delay_us(10);
        }
        FreeRtos::delay_ms(1_000);
        for duty in (min_duty..max_duty).rev() {
            pwm.set_duty(duty).unwrap();
            Ets::delay_us(10);
        }
        FreeRtos::delay_ms(1_000);
    }
}
