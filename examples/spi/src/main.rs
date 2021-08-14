//! SPI example to communicate with the System Basis Chip on the FRDM-KEAxXXX
//! dev boards.
//!
//! Some things to know about the setup on the devboard:
//!  - The SBC is connected to SPI1 default pins PTD0:PTD3
//!  - SBC IO-O is connected to SW4 which has a 12k pullup to Vsup and debounce
//!  - SBC MuxOut is connected to PA6

#![no_main]
#![no_std]

use kea_hal as hal;

use core::convert::TryInto;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::spi;
use hal::embedded_time::rate::*;
use hal::{pac, prelude::*};
use panic_halt as _;

fn send_16<SPI: spi::FullDuplex<u8>>(spi: &mut SPI, word: u16) -> u16 {
    let mut returned: [u8; 2] = [0, 0];
    for (count, byte) in word.to_be_bytes().iter().enumerate() {
        let _ = hal::nb::block!(spi.send(*byte));
        returned[count] = hal::nb::block!(spi.read()).ok().unwrap();
    }
    u16::from_be_bytes(returned)
}

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let sbc_spi = dp.SPI1.split();

    let sw4 = gpioa.pta6.into_pull_up_input();
    let mut led_green = gpiob.pth1.into_push_pull_output(); // low is on
    led_green.set_high().unwrap();

    // Set for default spi phase and polarity
    let spi_mode = spi::Mode {
        phase: spi::Phase::CaptureOnSecondTransition,
        polarity: spi::Polarity::IdleLow,
    };

    let mut cs_pin = gpioa.ptd3.into_push_pull_output();
    cs_pin.set_high().unwrap();

    // Pass in the pins used by SPI1. SPI1 has no alternate pins, so we can't
    // use the into_alt_pins method to switch them out.
    let mut sbc_spi = sbc_spi.enable_as_controller::<hal::gpio::DefaultMode, hal::gpio::DefaultMode, hal::gpio::DefaultMode, hal::gpio::DefaultMode>(
        gpioa.ptd0, // clock
        gpioa.ptd1, // sdo (mosi)
        gpioa.ptd2, // sdi (miso)
        //Some(gpioa.ptd3), // cs; pin will be toggled by the peripheral
        None, // cs; pin will be toggled by the user
        true, // The peripheral will set the mode_fault flag (and interrupt if enabled) if some other device tries to be master and sets this line low
        spi_mode,
    );

    // bus clock is 16MHz by default. SBC max rate is 1/250ns = 4MHz, let's use
    // 1MHz just for fun.
    // Calculate and set the bus divisor settings to achieve 1Mbps baudrate.
    sbc_spi.set_baudrate(1_000_000_u32.Hz(), 16_u32.MHz().try_into().unwrap());

    // read Mode
    const SBC_READ_MODE: u16 = 0xDD80;

    //const SBC_ONLY_READ: u16 = 0x2580;
    const SBC_PET_WATCHDOG: u16 = 0x5A00;

    // 01 = write, 00000 = Mux addr, Parity is high if count_ones() is even
    // 100 = Mux to IO-0, 0 = disable internal 2k res, 1 = divide voltage by 4
    const SBC_SET_MUX_IO0: u16 = 0b_01_00000_0_100_0_1_000;

    // 01 = write, 01111 = mux addr, parity high if count_ones() is even
    // 00 = no Vaux (not on chip), 10 = 5VCAN on with monitoring after 1ms, 1 =
    //    enable external Vdd transistor, 1 = automatically use the external
    //    when > 60mA current, 0 = disable turning off Vdd in LP mode
    const SBC_SET_5VCAN: u16 = 0b01_01111_1_00_0_10_110;

    // Init SBC
    // get mode
    cs_pin.set_low().unwrap();
    let val = send_16(&mut sbc_spi, SBC_READ_MODE);
    cs_pin.set_high().unwrap();

    // Pet watchdog to enter normal mode
    cs_pin.set_low().unwrap();
    let val0 = send_16(&mut sbc_spi, SBC_PET_WATCHDOG);
    cs_pin.set_high().unwrap();

    // 5VCAN supply must be on for MUX to function
    cs_pin.set_low().unwrap();
    let val1 = send_16(&mut sbc_spi, SBC_SET_5VCAN);
    cs_pin.set_high().unwrap();

    // Set SBC mux to IO-0
    cs_pin.set_low().unwrap();
    let val2 = send_16(&mut sbc_spi, SBC_SET_MUX_IO0);
    cs_pin.set_high().unwrap();

    // Pet SBC watchdog
    cs_pin.set_low().unwrap();
    let mut val3 = send_16(&mut sbc_spi, SBC_PET_WATCHDOG);
    cs_pin.set_high().unwrap();

    loop {
        if sw4.is_low().unwrap() {
            led_green.set_low().unwrap();
        } else {
            led_green.set_high().unwrap();
        }

        // Pet SBC watchdog
        cs_pin.set_low().unwrap();
        val3 = send_16(&mut sbc_spi, SBC_PET_WATCHDOG);
        cs_pin.set_high().unwrap();

        //cortex_m::asm::nop();
    }
}
