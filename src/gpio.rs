//! GPIO Pheripheral
//!
//! Note that per 11.1 of KEA64 Ref Man, if the pin is switch to alternate
//! peripheral function, the IO functions are disabled. Peripherals have
//! priority over IO.

use core::marker::PhantomData;

/// Default mode for GPIO pins
///
/// at reset, per 11.1 of KEA64 sub-family reference manual (pg. 133)
/// PTA4:5, PTB4, and PTC4 default to SWD_DIO, SWD_CLK, NMI, & RESET function.
pub type DefaultMode = HighImpedence;

/// Trait to split the pin register into independent pins and regs
pub trait GPIOExt {
    /// holds the pins
    type Parts;

    /// splits the peripheral into pins
    fn split(self) -> Self::Parts;
}

/// High Impedence type state
///
/// This is a type state AND and a pin state. It's not technically an input
/// since the port cannot (successfully) be written to or read from.
/// The Input probably not readable in this state.
/// @TODO Verify if functional in state
pub struct HighImpedence;

/// Input Mode type state
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output Mode type state
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Analog type state.
///
/// ADC MUST BE ENABLED BEFORE CALLING into_analog ON A PIN OR THE MCU WILL
/// HARDFAULT!
///
/// This mode "gives" the pin to the ADC hardware peripheral.
/// The ADC Peripheral can take the GPIO pins in any state. The Peripheral will
/// reconfigure the pin to turn off any output drivers, disable input buffers
/// (reading the pin after configuring as analog will return a zero), and
/// disable the pullup (i.e. HighImpedence state).
///
/// Once a pin is released from the ADC, it will return to its previous state.
/// The previous state includes output enabled, input enabled, pullup enabled,
/// and level (for outputs). Note to accomplish this the pin implements the
/// outof_analog method, which is semantically different from the other type
/// states.
///
/// For example, [gpioa::PTA0] is configured to be a Output that is set high is
/// converted into the analog mode with the [gpioa::PTA0::into_analog] method.
/// Once measurements from that pin are completed it will be returned to an
/// Output that is set high by calling the [Analog::outof_analog] method.
///
/// Note: This is a hardware feature that requires effectively no clock cycles
/// to complete. "Manually" reconfiguring the pins to HighImpedence before
/// calling into_analog() is discouraged, but it would not hurt anything.
// This type needs to be in this module so we can freely re-instantiate the Pin
// type returned from outof_analog. This can be moved to Analog if the Pin types
// implement a constructor method. It seems slightly more cohesive to have the
// Analog wrapper type here compared to adding a constructor just for this.
pub struct Analog<OldMode> {
    _oldmode: PhantomData<OldMode>,
}

/// Floating input type state
pub struct Floating;

/// PullUp input type state
pub struct PullUp;

/// PushPull output type state
pub struct PushPull;

/// High Current Drive output state
///
/// For KEA, this is called High Current Drive output type state and is only
/// available on a few ports.
pub struct HighDrive;

macro_rules! gpio {
    ($GPIOx:ident, $gpiox:ident, $puex: ident,
     [ $($PTX:ident: $PTXn:expr,)+ ],
     [ $($PTXi:ident: ($ptxi:ident, $i:expr, $FLTOffset:expr),)+ ],
     [ $($HighDrivePin:ident: ($i2:expr, $HighDriveIndex:expr),)+ ],
     [ $($AnalogIndex:expr => $AnalogPin:ident,)+
     ]) => {

        /// GPIO Port Module
        pub mod $gpiox {
            use super::{Analog, PushPull, PullUp, HighDrive, HighImpedence,
            Floating, Input, Output, GPIOExt, DefaultMode
            };
            use crate::hal::digital::v2::{ToggleableOutputPin, InputPin, OutputPin, StatefulOutputPin};
            use crate::pac::{$GPIOx, PORT, ADC};
            use core::marker::PhantomData;
            use core::convert::Infallible;

            /// Port Collection
            pub struct Parts {
                $(
                    /// A pin in the port
                    pub $ptxi: $PTXi<DefaultMode>,
                )+
            }

            impl GPIOExt for $GPIOx {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $ptxi: $PTXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Partially erased pin?
                pub struct $PTX<MODE> {
                    i: u8,
                    _mode: PhantomData<MODE>,
                }


                impl<MODE> OutputPin for $PTX<Output<MODE>> {
                    type Error = Infallible;

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // Atomically set high via stateless register
                        unsafe {
                            (*$GPIOx::ptr()).psor.write(|w| {
                                w.bits(1 << self.i)
                            });
                        }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // Atomically set low via statelss register
                        unsafe {
                            (*$GPIOx::ptr()).pcor.write(|w| {
                                w.bits(1 << self.i)
                            });
                        }
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PTX<Output<MODE>> {
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_set_low()?)
                    }
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        // Atomically check truthiness of bit in port
                        Ok(unsafe {
                            (*$GPIOx::ptr()).pdor.read().bits() & (1 << self.i)
                        } == 0)
                    }
                }

                impl<MODE> ToggleableOutputPin for $PTX<Output<MODE>> {
                    type Error = Infallible;

                    fn toggle(&mut self) -> Result<(), Self::Error>{
                        Ok(unsafe {
                            (*$GPIOx::ptr()).ptor.write(|w| {
                                w.bits(1 << self.i)
                            })
                        })
                    }
                }

                impl<MODE> InputPin for $PTX<Output<MODE>> {
                    type Error = Infallible;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_low()?)
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe{
                            (*$GPIOx::ptr()).pdir.read().bits() & (1 << self.i)
                        } == 0)
                    }
                }
            )+

            $(
                /// a GPIO Port Pin
                pub struct $PTXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                // What is this Into business? I don't remember!
                // What is the interface like? Where did I see this?

                impl Into<$PTXi<Input<PullUp>>> for $PTXi<DefaultMode> {
                    fn into(self) -> $PTXi<Input<PullUp>> {
                        self.into_pull_up_input()
                    }
                }

                impl Into<$PTXi<Input<Floating>>> for $PTXi<DefaultMode> {
                    fn into(self) -> $PTXi<Input<Floating>> {
                        self.into_floating_input()
                    }
                }

                impl Into<$PTXi<Output<PushPull>>> for $PTXi<DefaultMode> {
                    fn into(self) -> $PTXi<Output<PushPull>> {
                        self.into_push_pull_output()
                    }
                }

                /// Implements the I/O type conversion methods
                impl<MODE> $PTXi<MODE> {
                    /// Configure as floating
                    pub fn into_floating_input(self) -> $PTXi<Input<Floating>> {
                        //whatever it needs to grab the right bits
                        // from pull up reg and mode reg
                        unsafe {
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up (1 = pullup)
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                            // Set to Input (0 = input)
                            gpio.pddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            // 0 = input
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        $PTXi {_mode: PhantomData}
                    }

                    /// Configure as pull up input
                    pub fn into_pull_up_input(self) -> $PTXi<Input<PullUp>> {
                        unsafe {
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Set to Input
                            // 0 = input, 1 = output
                            gpio.pddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            // 0 = input, 1 = output
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                            // Turn on Pull Up
                            // 1 = on, 0 = off
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });

                        }

                        $PTXi {_mode: PhantomData}
                    }

                    /// Configure as PushPull output
                    pub fn into_push_pull_output(self) -> $PTXi<Output<PushPull>> {
                        unsafe {
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                            // Disable input (temporarily hiZ)
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });

                            // set to output
                            gpio.pddr.modify(|r,w| {
                                w.bits(r.bits() | (1 << $i))
                            });

                        }

                        $PTXi {_mode: PhantomData}
                    }

                    /// Configure into High Impedence output
                    pub fn into_high_impedence(self) -> $PTXi<HighImpedence> {
                        unsafe {
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });


                            // Make HiZ (disable input)
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() & (1 << $i))
                            });

                            // Set to Input
                            gpio.pddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                        }
                        $PTXi {_mode: PhantomData}
                    }
                }

                impl<MODE> OutputPin for $PTXi<Output<MODE>> {
                    type Error = Infallible;

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // Atomically set high via stateless register
                        unsafe {
                            (*$GPIOx::ptr()).psor.write(|w| {
                                w.bits(1 << $i)
                            });
                        }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // Atomically set low via statelss register
                        unsafe {
                            (*$GPIOx::ptr()).pcor.write(|w| {
                                w.bits(1 << $i)
                            });
                        }
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PTXi<Output<MODE>> {
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_set_low()?)
                    }
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        // Atomically check truthiness of bit in port
                        Ok(unsafe {
                            (*$GPIOx::ptr()).pdor.read().bits() & (1 << $i)
                        } == 0)
                    }
                }

                impl<MODE> ToggleableOutputPin for $PTXi<Output<MODE>> {
                    type Error = Infallible;

                    fn toggle(&mut self) -> Result<(), Self::Error>{
                        Ok(unsafe {
                            (*$GPIOx::ptr()).ptor.write(|w| {
                                w.bits(1 << $i)
                            })
                        })
                    }
                }

                impl<MODE> InputPin for $PTXi<Input<MODE>> {
                    type Error = Infallible;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_low()?)
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe{
                            (*$GPIOx::ptr()).pdir.read().bits() & (1 << $i)
                        } == 0)
                    }

                }
            )+

            $(
                impl Into<$HighDrivePin<Output<HighDrive>>> for $HighDrivePin<DefaultMode> {
                    fn into(self) -> $HighDrivePin<Output<HighDrive>> {
                        self.into_high_drive_output()
                    }
                }

                impl<MODE> $HighDrivePin<MODE> {
                    /// Configure into push pull output
                    ///
                    /// The KEA series calls this high current drive
                    /// This is only implemented for PTB4:5, PTD0:1, PTE0:1,
                    /// and PTH0:1.
                    pub fn into_high_drive_output(self) -> $HighDrivePin<Output<HighDrive>> {
                        unsafe {
                            let gpio = &(*$GPIOx::ptr());
                            let port = &(*PORT::ptr());

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i2))
                            });


                            // Disable input (temporarily hiZ)
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i2))
                            });

                            // Set to Output
                            gpio.pddr.modify(|r,w| {
                                w.bits(r.bits() | (1 << $i2))
                            });

                            // Enable high current drivers
                            port.hdrve.modify(|r,w| {
                                w.bits(r.bits() | (1 << $HighDriveIndex))
                            });
                        }

                        $HighDrivePin {_mode: PhantomData}
                    }
                }
            )+
            $(
                impl<MODE> $AnalogPin<MODE> {
                    /// Convert Pin into the [Analog] state for use by the ADC.
                    ///
                    /// Note, this is a restrictive mode for use only by the
                    /// ADC. The [$gpiox::$AnalogPin::outof_analog] method must
                    /// be used to return the pin to a normal Input/Output
                    /// typestate.
                    pub fn into_analog(self) -> Analog<$AnalogPin<MODE>> {
                        unsafe {
                            (*ADC::ptr())
                                .apctl1
                                .modify(|r, w| w.adpc().bits(
                                        r.adpc().bits() | (1 << $AnalogIndex)
                                        )
                                    );
                        }
                        Analog { _oldmode: PhantomData}
                    }
                }

                impl<OldMode> Analog<$AnalogPin<OldMode>> {
                    /// Return Analog pin to a normal pin state.
                    pub fn outof_analog(self) -> $AnalogPin<OldMode> {
                        let adc = unsafe {&(*ADC::ptr())};
                        adc.apctl1.modify(|r, w| unsafe {
                            w.adpc().bits(r.adpc().bits() & !(1 << $AnalogIndex))
                        });
                        $AnalogPin {_mode: PhantomData}
                    }
                }
            )+
        }
    }
}

gpio!(GPIOA, gpioa, puel, [
    PTA: 0,
    PTB: 1,
    PTC: 2,
    PTD: 3,
], [
    PTA0: (pta0, 0, 0),
    PTA1: (pta1, 1, 0),
    PTA2: (pta2, 2, 0),
    PTA3: (pta3, 3, 0),
    PTA4: (pta4, 4, 0),
    PTA5: (pta5, 5, 0),
    PTA6: (pta6, 6, 0),
    PTA7: (pta7, 7, 0),
    PTB0: (ptb0, 8, 2),
    PTB1: (ptb1, 9, 2),
    PTB2: (ptb2, 10, 2),
    PTB3: (ptb3, 11, 2),
    PTB4: (ptb4, 12, 2),
    PTB5: (ptb5, 13, 2),
    PTB6: (ptb6, 14, 2),
    PTB7: (ptb7, 15, 2),
    PTC0: (ptc0, 16, 4),
    PTC1: (ptc1, 17, 4),
    PTC2: (ptc2, 18, 4),
    PTC3: (ptc3, 19, 4),
    PTC4: (ptc4, 20, 4),
    PTC5: (ptc5, 21, 4),
    PTC6: (ptc6, 22, 4),
    PTC7: (ptc7, 23, 4),
    PTD0: (ptd0, 24, 6),
    PTD1: (ptd1, 25, 6),
    PTD2: (ptd2, 26, 6),
    PTD3: (ptd3, 27, 6),
    PTD4: (ptd4, 28, 6),
    PTD5: (ptd5, 29, 6),
    PTD6: (ptd6, 30, 6),
    PTD7: (ptd7, 31, 6),
], [
    PTB4: (12, 0),
    PTB5: (13, 1),
    PTD0: (24, 2),
    PTD1: (25, 3),
], [
    0_u8 => PTA0,
    1_u8 => PTA1,
    2_u8 => PTA6,
    3_u8 => PTA7,
    4_u8 => PTB0,
    5_u8 => PTB1,
    6_u8 => PTB2,
    7_u8 => PTB3,
    8_u8 => PTC0,
    9_u8 => PTC1,
    10_u8 => PTC2,
    11_u8 => PTC3,
]);

gpio!(GPIOB, gpiob, pueh, [
    PTE: 0,
    PTF: 1,
    PTG: 2,
    PTH: 3,
], [
    PTE0: (pte0, 0, 8),
    PTE1: (pte1, 1, 8),
    PTE2: (pte2, 2, 8),
    PTE3: (pte3, 3, 8),
    PTE4: (pte4, 4, 8),
    PTE5: (pte5, 5, 8),
    PTE6: (pte6, 6, 8),
    PTE7: (pte7, 7, 8),
    PTF0: (ptf0, 8, 10),
    PTF1: (ptf1, 9, 10),
    PTF2: (ptf2, 10, 10),
    PTF3: (ptf3, 11, 10),
    PTF4: (ptf4, 12, 10),
    PTF5: (ptf5, 13, 10),
    PTF6: (ptf6, 14, 10),
    PTF7: (ptf7, 15, 10),
    PTG0: (ptg0, 16, 12),
    PTG1: (ptg1, 17, 12),
    PTG2: (ptg2, 18, 12),
    PTG3: (ptg3, 19, 12),
    PTH0: (pth0, 24, 14),
    PTH1: (pth1, 25, 14),
    PTH2: (pth2, 26, 14),
    PTH6: (pth6, 30, 14),
    PTH7: (pth7, 31, 14),
], [
    PTE0: (0, 4),
    PTE1: (1, 5),
    PTH0: (24, 6),
    PTH1: (25, 7),
], [
    12_u8 => PTF4,
    13_u8 => PTF5,
    14_u8 => PTF6,
    15_u8 => PTF7,
]);

// fn thingy() {
//     unsafe {
//         let gpioa = &(*pac::GPIOA::ptr());
//         let port = &(*pac::PORT::ptr());
//     }
// }
