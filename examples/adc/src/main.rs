#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use hal::{pac, prelude::*};
use panic_halt as _;

pub struct State {
    is_output: bool,
    is_input: bool,
    is_pullup: bool,
}

pub fn pin_state() -> State {
    let gpioa = unsafe { &(*pac::GPIOA::ptr()) };
    let port = unsafe { &(*pac::PORT::ptr()) };
    State {
        is_output: (gpioa.pddr.read().pdd().bits() & (1 << 18)) != 0,
        is_input: (gpioa.pidr.read().pid().bits() & (1 << 18)) == 0,
        is_pullup: port.puel.read().ptcpe2().bit_is_set(),
    }
}

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let adc = dp.ADC.split();

    // setup adc clock (bus clock is 16MHz by default).
    // Adc has to be in Enabled State (specifically, SIM.scgc[adc] == 1) before
    // touching the peripheral, including calling into_analog on a pin.
    // Failing to do so causes hardfault.
    let config: hal::adc::AdcConfig = Default::default(); // the default is okay for this
    let mut adc = adc.configure(config);

    // set pot to something so we can verify its return type after
    // releasing from adc.
    let _state = pin_state();
    let pot = gpioa.ptc2.into_pull_up_input(); // Should measure value
    let ptc3 = gpioa.ptc3.into_pull_up_input(); // should be in this state after releaseing
    let mut ptb3 = gpioa.ptb3.into_push_pull_output(); // should be in this state after releasing

    ptb3.set_high().unwrap();

    let _state = pin_state();

    let _init_binval = pot.is_high();

    // turn pot (PTC2/AD10) and ptc3 into adc channels
    let mut pot = pot.into_analog();
    let mut ptc3 = ptc3.into_analog();

    let _pot_val = adc.read(&mut pot);
    let _pc3_val = adc.read(&mut ptc3);

    // return to normal pins
    let _pot = pot.outof_analog();
    let _ptc3 = ptc3.outof_analog();

    let _state = pin_state();

    loop {
        cortex_m::asm::nop();
    }
}
