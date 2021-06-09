//! This example will toggle the Red LED everytime the watchdog resets (must be
//! run in release mode).
//!
//! The watchdog will only function properly when flashed (no debugger
//! connected). Flash the MCU, power cycle the board (pull the plug), note the
//! LED turns on green to indicate that the reset was due to a power cycle.
//! Wait approximately 30 seconds and the LED will go to a dim red and green
//! (the red is much stronger than the green).
//!
//! The example also provides some sim values to be read with a debugger (note
//! watchdog will just reset whenever it's touched with the debugger attached).
//!
//! @TODO incorporate UART as option to print out what goes on here.

#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use hal::{pac, prelude::*, system};
use panic_halt as _;

#[entry]
fn main() -> ! {
    //println!("Hello, world!");
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let sim = dp.SIM.split();
    let watchdog = dp.WDOG.split();
    let gpiob = dp.GPIOB.split();

    // Sim values to read with the debugger
    // @TODO read these out through USART or semihosted environment
    let id: system::sim::Id = sim.id;
    let _ids = (
        id.family(),
        id.subfamily(),
        id.revision(),
        id.pinout() as u8,
    );
    let _statii = (sim.status.debugger_reset(), sim.status.lv_reset());
    let _uuid = sim.uuid.uuid();

    let mut config = watchdog.configuration();
    config.period = 0x3FFF;
    config.clock = system::watchdog::WDogClock::LpoClock;

    let watchdog = watchdog.configure(config);

    let mut led_red = gpiob.pth0.into_push_pull_output();
    led_red.set_high().unwrap();
    let mut led_green = gpiob.pth1.into_push_pull_output();
    led_green.set_high().unwrap();

    let counts = watchdog.counts();
    if counts > 0xFFFE {
        // it won't be
        cortex_m::asm::nop();
    }

    // If reset was because of the watchdog
    if sim.status.watchdog_reset() {
        // Turn on the LED
        led_red.set_low().unwrap();

        // Reconfigure the watchdog for a short timer on a faster clock.
        // reset every 8mSec
        // (makes the LED somewhat more DIM than full on RED)
        let mut config = watchdog.configuration();
        config.period = 0x00FF;
        config.clock = system::watchdog::WDogClock::IntRefClock;
        watchdog.configure(config);

        // turning on the green proves that unlocking didn't trigger a reset
        led_green.set_low().unwrap();
    }

    if sim.status.power_on_reset() {
        led_green.set_low().unwrap();
    }
    loop {
        cortex_m::asm::nop();
    }
}
