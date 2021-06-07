//! This example will toggle the Red LED everytime the watchdog resets (must be
//! run in release mode). The example also provides some sim values to be read
//! with a debugger (must be run in debug mode)

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

    //let mut config = watchdog.configuration();
    //config.period = 0xFFFE;
    //config.clock = system::watchdog::WDogClock::LpoClock;
    let config = system::watchdog::WDogConfig {
        interrupt: false,
        debug_mode: true,
        wait_mode: false,
        stop_mode: false,
        windowed: false,
        prescale: false,
        period: 0xA55A,
        window: 0,
        clock: system::watchdog::WDogClock::IntRefClock,
    };
    let temp = watchdog.configuration();

    watchdog.configure(temp).configure(config);
    //config = watchdog.configuration();

    //    let id: system::sim::Id = sim.id;
    //
    //    let _ids = (
    //        id.family(),
    //        id.subfamily(),
    //        id.revision(),
    //        id.pinout() as u8,
    //    );
    //
    //    let _statii = (sim.status.debugger_reset(), sim.status.lv_reset());
    //
    //    let _uuid = sim.uuid.uuid();
    //
    let mut led_red = gpiob.pth0.into_push_pull_output();
    led_red.set_high().unwrap();
    if sim.status.watchdog_reset() {
        led_red.set_low().unwrap();
    }

    let mut led_green = gpiob.pth1.into_push_pull_output();
    led_green.set_high().unwrap();
    if sim.status.power_on_reset() {
        led_green.set_low().unwrap();
    }
    loop {
        cortex_m::asm::nop();
    }
}
