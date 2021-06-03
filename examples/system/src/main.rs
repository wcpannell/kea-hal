#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
use hal::{pac, prelude::*, system};
use panic_halt as _;

#[entry]
fn main() -> ! {
    //println!("Hello, world!");
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let sim = dp.SIM.split();

    let id: system::sim::Id = sim.id;

    let _ids = (
        id.family(),
        id.subfamily(),
        id.revision(),
        id.pinout() as u8,
    );

    let _statii = (sim.status.debugger_reset(), sim.status.lv_reset());

    let _uuid = sim.uuid.uuid();

    loop {
        cortex_m::asm::nop();
    }
}
