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
     [ $($HighDrivePin:ident: ($i2:expr, $HighDriveIndex:expr),)+
     ]) => {

        /// GPIO Port Module
        pub mod $gpiox {
            use super::{PushPull, PullUp, HighDrive, HighImpedence, Floating,
            Input, Output, GPIOExt, DefaultMode
            };
            use crate::hal::digital::v2::{ToggleableOutputPin, InputPin, OutputPin, StatefulOutputPin};
            use crate::pac::{$GPIOx, PORT};
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

                            // Turn off Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                            // Set to Input
                            gpio.pddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
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
                            gpio.pddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            gpio.pidr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });

                            // Turn on Pull Up
                            port.$puex.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
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
]);

// fn thingy() {
//     unsafe {
//         let gpioa = &(*pac::GPIOA::ptr());
//         let port = &(*pac::PORT::ptr());
//     }
// }
