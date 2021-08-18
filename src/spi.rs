//! # SPI Peripheral
//!
//! ## Pins
//!
//! The SPI0 peripheral has two choices for its I/O pins (SCK, MOSI, MISO, CS),
//! PTB2:5 or PTE0:3. SPI1 only uses one set of pins (PTD0:3).
//!
//! At this time there is no method implemented to return Pins from the SPI
//! peripherals. Keep in mind that once the pins given to the SPI they can not
//! be returned.
//!
//! ### High Drive Current GPIO
//!
//! The SPI peripheral, like other peripherals in this family of MCUs will
//! reconfigure the GPIO ports as needed when the peripheral is activated.
//! However, the SPI peripherals do not modify the High Drive Current
//! peripheral's settings. This provides stronger drive to the SPI bus lines in
//! order to increase the slew rate for the output signal.
//!
//! Note that only the MOSI (sdo in controller mode) pin for SPI0 and SPI1 with
//! the default pins and MISO (sdo in peripheral mode) for SPI0 with alternate
//! pins have the high current drive peripheral implemented.
//!
//! See the spi-talking-to-myself example in the source repo.
//!
//! ## Interrupts
//!
//! SPI peripherals have one interrupt vector, 4 flags, and 3 masks each.
//! Once in the interrupt you will have to check the flag bits to determine
//! which flag triggered the interrupt in order to respond appropriately.

use crate::hal::spi;
use crate::{pac::SPI0, pac::SPI1, HALExt};
use core::marker::PhantomData;
use embedded_time::rate::*;

/// Spi Peripheral Interface
// @TODO - Work out how to store pins so we can give them back
pub struct Spi<SPI, Disabled, Pins> {
    peripheral: SPI,
    _state: PhantomData<Disabled>,
    _pins: PhantomData<Pins>,
}

/// Peripheral disabled
pub struct Disabled;
/// Peripheral enabled
pub struct Enabled<T> {
    _state: PhantomData<T>,
}
/// Peripheral ignores this sub-state at this time
pub struct DontCare;

/// SPI Peripheral in Controller mode
pub struct Controller;
///SPI Peripheral in Peripheral mode
pub struct Peripheral;

/// Peripheral uses default pins (or only has one set of pins available)
pub struct DefaultPins;
/// Peripheral is using alternate pins
pub struct AltPins;

// // Roughing work on Pin Storage so we can return the pins one day
// struct DefaultPinsSpi0<T2, T3, T4, T5> {
//     clock: PTB2<T2>,
//     copi: PTB3<T3>,
//     cipo: PTB4<T4>,
//     cs: Option<PTB5<T5>>,
// }
//
// impl<T2, T3, T4, T5> Spi0Pins for DefaultPinsSpi0<T2, T3, T4, T5> {}

// impl<T2, T3, T4, T5> Default for DefaultPinsSpi0<T2, T3, T4, T5> {
//     fn default() -> Self {
//         Self {
//             clock: None,
//             copi: None,
//             cipo: None,
//             cs: None,
//         }
//     }
// }
//
// struct AltPinsSpi0<T0, T1, T2, T3> {
//     clock: Option<PTE0<T0>>,
//     copi: Option<PTE1<T1>>,
//     cipo: Option<PTE2<T2>>,
//     cs: Option<PTE3<T3>>,
// }
//
// impl<T0, T1, T2, T3> Spi0Pins for AltPinsSpi0<T0, T1, T2, T3> {}
//
// impl<T0, T1, T2, T3> Default for AltPinsSpi0<T0, T1, T2, T3> {
//     fn default() -> Self {
//         Self {
//             clock: None,
//             copi: None,
//             cipo: None,
//             cs: None,
//         }
//     }
// }

use crate::gpio::gpioa::{PTB2, PTB3, PTB4, PTB5};
use crate::gpio::gpiob::{PTE0, PTE1, PTE2, PTE3};
impl HALExt for SPI0 {
    type T = Spi<SPI0, Disabled, DefaultPins>;
    fn split(self) -> Self::T {
        Spi {
            peripheral: self,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }
}

use crate::gpio::gpioa::{PTD0, PTD1, PTD2, PTD3};
impl HALExt for SPI1 {
    type T = Spi<SPI1, Disabled, DefaultPins>;
    fn split(self) -> Spi<SPI1, Disabled, DefaultPins> {
        Spi {
            peripheral: self,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }
}

impl Spi<SPI0, Disabled, DefaultPins> {
    /// Change to alternate pins
    pub fn into_alt_pins(self) -> Spi<SPI0, Disabled, AltPins> {
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }

    /// Enable SPI0 with default pins
    pub fn enable_as_controller<T2, T3, T4, T5>(
        self,
        clock: PTB2<T2>,
        sdo: PTB3<T3>,
        // sdi: Option<PTB4<T4>>,  // Bidirectional mode needs own Mode type
        sdi: PTB4<T4>,
        cs: Option<PTB5<T5>>,
        manage_cs: bool,
        mode: spi::Mode,
    ) -> Spi<SPI0, Controller, DefaultPins> {
        // Select PTB2:5 for SPI0
        let sim = unsafe { &(*pac::SIM::ptr()) };
        // Select PTE0:3 for SPI0
        sim.pinsel.modify(|_, w| w.spi0ps()._0());
        // Enable busclock to SPI0 peripheral before touching it
        sim.scgc.modify(|_, w| w.spi0()._1());
        self.enable_spi(true, false, cs.is_some(), manage_cs, mode);
        let _ = (clock, sdo, sdi, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }

    /// Enable SPI0 in peripheral mode with Alternate Pins
    pub fn enable_as_peripheral<T2, T3, T4, T5>(
        self,
        clock: PTB2<T2>,
        sdi: PTB3<T3>,
        sdo: PTB4<T4>,
        cs: PTB5<T5>,
        mode: spi::Mode,
    ) -> Spi<SPI0, Enabled<Peripheral>, DefaultPins> {
        let sim = unsafe { &(*pac::SIM::ptr()) };
        // Select PTE0:3 for SPI0
        sim.pinsel.modify(|_, w| w.spi0ps()._0());
        // Enable busclock to SPI0 peripheral before touching it
        sim.scgc.modify(|_, w| w.spi0()._1());

        // Peripheral mode always uses cs, and manage_cs has no effect
        self.enable_spi(false, false, true, true, mode);

        let _ = (clock, sdo, sdi, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }
}

impl Spi<SPI0, Disabled, AltPins> {
    /// Enable SPI0 in controller mode with Alternate Pins
    pub fn enable_as_controller<T0, T1, T2, T3>(
        self,
        clock: PTE0<T0>,
        sdo: PTE1<T1>,
        // sdi: Option<PTE2<T2>>,  // Bidirectional mode needs own Mode type
        sdi: PTE2<T2>,
        cs: Option<PTE3<T3>>,
        manage_cs: bool,
        mode: spi::Mode,
    ) -> Spi<SPI0, Controller, AltPins> {
        let sim = unsafe { &(*pac::SIM::ptr()) };
        // Select PTE0:3 for SPI0
        sim.pinsel.modify(|_, w| w.spi0ps()._1());
        // Enable busclock to SPI0 peripheral before touching it
        sim.scgc.modify(|_, w| w.spi0()._1());

        // bidirectional controller with mode fault enabled will auto switch to
        // peripheral mode when modefault occurs. Current impl does not handle
        // that
        self.enable_spi(true, false, cs.is_some(), manage_cs, mode);
        let _ = (clock, sdo, sdi, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }

    /// Enable SPI0 in peripheral mode with Alternate Pins
    pub fn enable_as_peripheral<T0, T1, T2, T3>(
        self,
        clock: PTE0<T0>,
        // sdi: Option<PTE1<T1>>,  // Bidirectional mode needs own mode type
        sdi: PTE1<T1>,
        sdo: PTE2<T2>,
        cs: PTE3<T3>,
        mode: spi::Mode,
    ) -> Spi<SPI0, Enabled<Peripheral>, AltPins> {
        let sim = unsafe { &(*pac::SIM::ptr()) };
        // Select PTE0:3 for SPI0
        sim.pinsel.modify(|_, w| w.spi0ps()._1());
        // Enable busclock to SPI0 peripheral before touching it
        sim.scgc.modify(|_, w| w.spi0()._1());

        // Peripheral mode always uses cs, and manage_cs has no effect
        self.enable_spi(false, false, true, true, mode);

        let _ = (clock, sdi, sdo, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }
}

impl Spi<SPI1, Disabled, DefaultPins> {
    /// Enable SPI1 as the controller with default pins
    ///
    /// When manage_cs is set (and the cs pin has been provided) SPI1 will
    /// automatically drive the CS pin when doing SPI transfers. If manage_cs
    /// is not set, then this pin is used to detect if there is another
    /// Controller on the bus, which throws the Master Mode Fault flag
    pub fn enable_as_controller<T0, T1, T2, T3>(
        self,
        clock: PTD0<T0>,
        sdo: PTD1<T1>,
        // sdi: Option<PTD2<T2>>,  // Bidirectional mode needs own Mode type
        sdi: PTD2<T2>,
        cs: Option<PTD3<T3>>,
        manage_cs: bool,
        mode: spi::Mode,
    ) -> Spi<SPI1, Enabled<Controller>, DefaultPins> {
        // Enable bus clock to SPI1 (needed before writing anything to the SPI
        // peripheral
        unsafe { (*pac::SIM::ptr()).scgc.modify(|_, w| w.spi1()._1()) };

        // bidirectional controller with mode fault enabled will auto switch to
        // peripheral mode when modefault occurs. Current impl does not handle
        // that

        self.enable_spi(true, false, cs.is_some(), manage_cs, mode);

        let _ = (clock, sdo, sdi, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }

    /// Enable SPI1 as peripheral with default pins
    pub fn enable_as_peripheral<T0, T1, T2, T3>(
        self,
        clock: PTD0<T0>,
        //sdi: Option<PTD1<T1>>,  // Bidirectional mode needs own Mode type
        sdi: PTD1<T1>,
        sdo: PTD2<T2>,
        cs: PTD3<T3>,
        mode: spi::Mode,
    ) -> Spi<SPI1, Enabled<Peripheral>, DefaultPins> {
        // Enable bus clock to SPI1 (needed before writing anything to the SPI
        // peripheral
        unsafe { (*pac::SIM::ptr()).scgc.modify(|_, w| w.spi1()._1()) };

        // Peripheral mode always uses cs, and manage_cs has no effect
        self.enable_spi(false, false, true, true, mode);
        let _ = (clock, sdi, sdo, cs);
        Spi {
            peripheral: self.peripheral,
            _state: PhantomData,
            _pins: PhantomData,
        }
    }
}

macro_rules! spi_builder {
    ( $($SpiRegister:ident,)+ ) => {
        $(
            impl<Pins> Spi<$SpiRegister, Disabled, Pins> {
                /// Do the low level work of enabling the SPI. try for DRY.
                fn enable_spi(
                    &self,
                    is_controller: bool,
                    is_bidirectional: bool,
                    use_cs: bool,
                    manage_cs: bool,
                    mode: spi::Mode
                ) {
                    self.peripheral.c1.write(|w| {
                        w.lsbfe()._0()  // MSB is transfered first
                            .ssoe()
                            .bit(use_cs && manage_cs)
                            .cpha()
                            // was using variant, but using bit works for both SPI0/1
                            .bit(match mode.phase {
                                spi::Phase::CaptureOnFirstTransition => false,
                                spi::Phase::CaptureOnSecondTransition => true,
                            })
                            .cpol()
                            // was using variant, but using bit works for both SPI0/1
                            .bit(match mode.polarity {
                                spi::Polarity::IdleLow => false,
                                spi::Polarity::IdleHigh => true,
                            })
                            .mstr()
                            .bit(is_controller)
                            .sptie()._0() // No interrupts implemented yet
                            .spe()
                            ._1()
                            .spie()._0() // No interrupts implemented yet
                    });

                    // Cannot just write to C2 because of reserve bits
                    // using bidirectional mode trashes FullDuplex?
                    self.peripheral.c2.modify(|_, w| {
                        w.spc0().bit(is_bidirectional)
                            .spiswai()._0() // default Spi active in WAIT.
                            .bidiroe().bit(is_controller) // if bidir, start in right mode
                            .modfen().bit(use_cs)
                            .spmie()._0()  // No interrupts implemented yet
                    });
                }
            }

            impl<Pins, Mode> Spi<$SpiRegister, Enabled<Mode>, Pins> {
                /// Set the baud rate of transmission
                ///
                /// This is only used when the MCU is the bus Controller. This relies on
                /// accurately inputting the bus frequency until a way to share the current
                /// bus frequency is worked out.
                pub fn set_baudrate(&self, baudrate: Hertz, bus_freq: Hertz) {
                    let divisor = bus_to_baudrate_divisor(bus_freq.integer(), baudrate.integer());
                    self.set_baudrate_divisor(&divisor);
                }

                /// Set the baud rate by directly setting the divisor
                pub fn set_baudrate_divisor(&self, divisor: &BaudrateDivisor) {
                    self.peripheral
                        .br
                        .write(|w| unsafe { w.sppr().bits(divisor.scale).spr().bits(divisor.power) });
                }

                /// Get the current baudrate divisor
                pub fn baudrate_divisor(&self) -> BaudrateDivisor {
                    let reader = self.peripheral.br.read();
                    BaudrateDivisor {
                        scale: reader.sppr().bits(),
                        power: reader.spr().bits(),
                    }
                }

                /// Check if mode fault occured
                pub fn mode_fault(&self) -> bool {
                    self.peripheral.s.read().modf().bit()
                }
            }

            impl<Mode, Pins> Spi<$SpiRegister, Mode, Pins> {
                /// Check if read buffer full (ready to read)
                pub fn read_ready(&self) -> bool {
                    self.peripheral.s.read().sprf().bit()
                }
                /// Check if read matches value in match register
                pub fn read_matches(&self) -> bool {
                    self.peripheral.s.read().spmf().bit()
                }
                /// Check if transmit buffer empty (ready to send)
                pub fn send_ready(&self) -> bool {
                    self.peripheral.s.read().sptef().bit()
                }
                /// Set the value for hardware read match
                pub fn set_hw_match(&self, value: u8) {
                    self.peripheral.m.write(|w| unsafe { w.bits(value) });
                }
            }

            impl<Pins, Mode> spi::FullDuplex<u8> for Spi<$SpiRegister, Enabled<Mode>, Pins> {
                type Error = core::convert::Infallible;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    if !self.read_ready() {
                        return Err(nb::Error::WouldBlock);
                    }

                    // Bidirectional mode not implemented
                    // // Set direction for bidirectional mode (no effect on normal mode)
                    // self.peripheral.c2.modify(|_, w| w.bidiroe()._0());

                    Ok(self.peripheral.d.read().bits())
                }

                fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
                    if !self.send_ready() {
                        return Err(nb::Error::WouldBlock);
                    }

                    // Bidirectional mode not implemented
                    // // Set direction for bidirectional mode (no effect on normal mode)
                    // self.peripheral.c2.modify(|_, w| w.bidiroe()._1());

                    self.peripheral.d.write(|w| unsafe { w.bits(word) });
                    Ok(())
                }
            }

        )+
    };
}
spi_builder!(SPI0, SPI1,);

/// Holds the parameters used to calculate the divisor used to derive the SPI
/// baudrate from the bus clock
pub struct BaudrateDivisor {
    /// Linearly scale the bus clock, value must be <= 7
    pub scale: u8,
    /// Scale the bus clock divisor by a power of 2, value must be <= 8
    pub power: u8,
}
impl BaudrateDivisor {
    /// The transfer function for computing the baud rate divisor.
    /// was throwing errors about converting between time rates and u32, so now in
    /// u32
    pub const fn divisor(&self) -> Result<u32, ()> {
        if (self.scale > 7) || (self.power > 8) {
            return Err(());
        }
        Ok((self.scale as u32 + 1) << (self.power + 1))
        //let result = bus_freq / divisor;
    }
}
const fn bus_to_baudrate_divisor(bus_freq: u32, baudrate: u32) -> BaudrateDivisor {
    // yolo on rounding errors
    let target: u32 = bus_freq / baudrate;
    divisor_to_baudrate_divisor(target)
}

// Replace with something other than stupid brute force eventually
// In the mean-time it's not that bad. 72 loops done on the user's host
// computer is practically instantaneous. Even on target it isn't the end of
// the world.
// @TODO check NXP's examples to see if they do better so I don't have to think
const fn divisor_to_baudrate_divisor(divisor: u32) -> BaudrateDivisor {
    let mut best: BaudrateDivisor = BaudrateDivisor { scale: 7, power: 9 };
    let mut scale: u8 = 0;
    let mut power: u8 = 0;
    let mut old_error: u32 = u32::max_value();
    while scale <= 7 {
        while power <= 8 {
            let new = BaudrateDivisor { scale, power };
            let new_div = match new.divisor() {
                Ok(f) => f,
                Err(_) => 8 << 9,
            };
            let error: u32 = (new_div as i32 - divisor as i32).unsigned_abs();
            if error <= old_error {
                old_error = error;
                best.scale = scale;
                best.power = power;
            }
            power += 1;
        }
        power = 0;
        scale += 1;
    }
    best
}

/// Errors used in Result types in this module
pub enum Errors {
    /// The baudrate divisor requested is out of range
    DivsorOutOfRange,
    /// One of the inputs is out of range
    BadInput,
}
