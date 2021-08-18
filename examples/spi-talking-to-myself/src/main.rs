//! SPI example to communicate between SPI0 and and SPI1 on the same board

#![no_main]
#![no_std]

use kea_hal as hal;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi;
use hal::embedded_time::rate::*;
use hal::{pac, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let periph_spi = dp.SPI0.split();
    let control_spi = dp.SPI1.split();

    // Set for default spi phase and polarity
    let spi_mode = spi::Mode {
        phase: spi::Phase::CaptureOnSecondTransition,
        polarity: spi::Polarity::IdleLow,
    };

    // Can't use PTD3 since that's the cs for the onboard SBC
    let mut cs_pin = gpioa.ptd4.into_push_pull_output();
    cs_pin.set_high().unwrap();

    // Pass in the pins used by SPI1. SPI1 has no alternate pins, so we can't
    // use the into_alt_pins method to switch them out.
    let mut control_spi = control_spi.enable_as_controller::<hal::gpio::DefaultMode, hal::gpio::DefaultMode, hal::gpio::DefaultMode, hal::gpio::DefaultMode>(
        gpioa.ptd0, // clock
        gpioa.ptd1, // sdo (mosi)
        gpioa.ptd2, // sdi (miso)
        None, // cs; pin will be toggled by the user
        true, // The peripheral will set the mode_fault flag (and interrupt if enabled) if some other device tries to be master and sets this line low
        spi_mode,
    );

    // Setting the PTB4 (peripheral sdo / miso) drive strength to high.
    // Using only the default push/pull results in only driving to 2.2ish volts
    // on miso line. High drive mode has the power to swing it rail to rail.
    let ptb4 = gpioa.ptb4.into_high_drive_output();

    // Use normal pins
    let mut periph_spi =
        periph_spi.enable_as_peripheral(gpioa.ptb2, gpioa.ptb3, ptb4, gpioa.ptb5, spi_mode);
    /*
        // use alternate pins.
        let mut periph_spi = periph_spi.into_alt_pins().enable_as_peripheral(
            gpiob.pte0,
            gpiob.pte1,
            gpiob.pte2,
            gpiob.pte3,
            true,
            spi_mode,
        );
    */

    // bus clock is 16MHz by default. SBC max rate is 1/250ns = 4MHz, let's use
    // 1MHz just for fun.
    // Calculate and set the bus divisor settings to achieve 1Mbps baudrate.
    control_spi.set_baudrate(1_000_000_u32.Hz(), 16_000_000_u32.Hz());

    use heapless::spsc::Queue;
    let mut control_txq: Queue<u8, 32> = Queue::new();
    let mut control_rxq: Queue<u8, 32> = Queue::new();
    let mut periph_txq: Queue<u8, 32> = Queue::new();
    let mut periph_rxq: Queue<u8, 32> = Queue::new();

    for letter in "Hello There!".as_bytes() {
        control_txq.enqueue(*letter).unwrap();
    }

    for letter in "Little One. ".as_bytes() {
        periph_txq.enqueue(*letter).unwrap();
    }

    while !control_txq.is_empty() {
        // prep response first.
        hal::nb::block!(periph_spi.send(periph_txq.dequeue().unwrap())).unwrap();

        // engage peripheral
        cs_pin.set_low().unwrap();

        // Controller transfer byte
        hal::nb::block!(control_spi.send(control_txq.dequeue().unwrap())).unwrap();
        control_rxq
            .enqueue(hal::nb::block!(control_spi.read()).unwrap())
            .unwrap();

        // See what the peripheral got.
        periph_rxq
            .enqueue(hal::nb::block!(periph_spi.read()).unwrap())
            .unwrap();

        // release peripheral
        cs_pin.set_high().unwrap();
    }

    loop {
        cortex_m::asm::nop();
    }
}
