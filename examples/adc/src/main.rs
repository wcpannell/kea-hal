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

pub fn adc_read(chan: u8) -> u16 {
    // Don't need to change anythign in sc2
    // start conversion in sc1
    let periph = unsafe { &(*pac::ADC::ptr()) };
    periph.sc1.modify(|_, w| unsafe { w.adch().bits(chan) });

    // wait for conversion
    while periph.sc2.read().adact().bit_is_set() {
        cortex_m::asm::nop();
    }

    periph.r.read().adr().bits()
}

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    // set pot to something so we can verify its return type after
    // releasing from adc.
    let state = pin_state();
    let pot = gpioa.ptc2.into_pull_up_input(); // Should measure value
    let ptc3 = gpioa.ptc3.into_pull_up_input(); // should be in this state after releaseing
    let mut ptb3 = gpioa.ptb3.into_push_pull_output(); // should be in this state after releasing

    ptb3.set_high().unwrap();

    let state = pin_state();

    let init_binval = pot.is_high();

    // Turn on ADC peripheral
    dp.SIM.scgc.modify(|_, w| w.adc()._1());

    // turn pot (PTC2/AD10) into an adc channel
    dp.ADC
        .apctl1
        .write(|w| unsafe { w.adpc().bits((1 << 11) | (1 << 10) | (1 << 7)) });

    // setup adc clock (bus clock is 16MHz by default)
    dp.ADC.sc3.write(
        |w| {
            w.adiclk()
                ._01() // bus clock/2
                .mode()
                ._10()
        }, // 12-bit measurement
    );

    let pot_val = adc_read(10);
    let pc3_val = adc_read(11);

    // return to normal pin
    dp.ADC.apctl1.reset();

    let state = pin_state();

    loop {
        cortex_m::asm::nop();
    }
}
