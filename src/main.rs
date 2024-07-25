#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut a0_value = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut a0_value);
    loop {
        let value = a0.analog_read(&mut a0_value);
        ufmt::uwriteln!(&mut serial, "Potenciometer {}", value).unwrap_infallible();
        arduino_hal::delay_ms(500);
    }
}
