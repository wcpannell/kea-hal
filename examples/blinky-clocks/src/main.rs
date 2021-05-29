#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
use hal::{clocks, pac, port, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    //println!("Hello, world!");
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.ICS.constrain();
    let sysclock = clocks.system_clock.into_fbi(); // 31.25kHz system clockrate

    // Pot is on PTC2
    // let pot = ADC not yet implemented, will control blink rate
    // (timer not implemented either)
    let sw2 = gpioa.pta0.into_floating_input(); // button to VDD, has 10K pulldown installed.
    let sw3 = gpioa.pta1.into_floating_input(); // button to VDD, has 10K pulldown installed.
    let mut led_red = gpiob.pth0.into_push_pull_output();
    let mut led_green = gpiob.pth1.into_high_drive_output();
    let mut led_blue = gpiob.pte7.into_push_pull_output();
    led_green.toggle().unwrap();
    let mut led_green = led_green.into_push_pull_output();
    let filter = port::Filter::get();
    filter.div3.set_divisor(2);
    filter.porta.into_div3();

    let mut i: u32 = 1;
    loop {
        led_red.set_high().unwrap();
        if sw2.is_high().unwrap() {
            led_green.toggle().unwrap();
        }
        if sw3.is_high().unwrap() {
            led_blue.set_high().unwrap();
        } else {
            led_blue.set_low().unwrap();
        }
        // wait
        led_red.set_low().unwrap();

        if i == 0x7F {
            let sysclock = sysclock.into_fei(); // 32MHz system clock
        } else if i == 0xFFFF {
            let sysclock = sysclock.into_fbi(); // 31.25kHz system clock
            i = 0;
        }

        i += 1;
    }
}
