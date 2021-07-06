#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
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
    let mut config: hal::adc::AdcConfig = Default::default();
    config.clock_divisor = hal::adc::ClockDivisor::_2;
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
    let mut ptb3 = ptb3.into_analog();
    let mut temp = adc.onchip_channels.tempsense().unwrap();
    let mut bandgap = adc.onchip_channels.bandgap().unwrap();

    let _bg_val = adc.read(&mut bandgap).unwrap_or(0);
    let _temp_val = adc.read(&mut temp).unwrap_or(0);
    let _pot_val = adc.read(&mut pot);
    let _ptc3_val = adc.read(&mut ptc3);
    let _ptb3_val = adc.read(&mut ptb3);

    // // Verify this can't happen
    // let _pot_adc_test = pot.into_analog();

    // // Verify this can't happen
    // let val = pot.is_high();

    // return to normal pins
    let pot = pot.outof_analog();
    let ptc3 = ptc3.outof_analog();
    let mut ptb3 = ptb3.outof_analog();
    adc.onchip_channels.return_tempsense(temp);

    // Verify this can happen
    let _pot_bin_val = pot.is_high();
    let _ptc3_bin_val = ptc3.is_high();
    let _ptb3_bin_val = ptb3.is_set_high();
    ptb3.set_low().unwrap();

    let _state = pin_state();

    // verify we can grab the temp again
    let mut new_temp = adc.onchip_channels.tempsense().unwrap();
    let _new_temp_val = adc.read(&mut new_temp);

    loop {
        cortex_m::asm::nop();
    }
}
