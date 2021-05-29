//! # Clocks meta-module
//!
//! ## Architecture
//!
//! The ICS module selects and/or muxes the clock sources to output the system
//! clock sources. the SIM module handles dividing and gating of those clock
//! sources. If an external oscillator/resonator is used, the OSC module passes
//! those signals to the ICS and SIM modules as needed.
//!
//! ## Definitions
//! - Core clock - ICSOUTCLK / BDIV - clock for the ARM core (HCLK). 40MHz max
//! - Platform Clock - ICSOUTCLK / BDIV - clocks crossbar switch and NVIC
//! (FCLK). 40MHz max
//! - System clock - ICSOUTCLK / BDIV - bus master clock. 40MHz max
//! - Bus Clock - System clock / BUSDIV - bus slave and peripheral clock. 20MHz
//! Max
//! - Flash clock - derived from Bus Clock. 20MHz max
//! - Debug clock - derived from platform clock. 20MHz max
//! - SWD clock - CMSIS-DAP clock, external, driven by debugger. 20MHz max
//! - ICSIRCLK - ICS output of the 32kHZ-ish internal reference clock. Can be
//! source for RTC or WDOG modules. 31.25-39.0625kHz
//! - ICSOUTCLK - output of ICS module's output MUX, drives core, platform,
//! system clocks and more
//! - ICSFLLCLK - 1024 * input clock
//! - ICSFFCLK - input to FLL made available to FTM modules
//! - OSCCLK - aka OSC_OUT - output of external oscillator (OSC) module.
//! DC-40MHz (bypass), 31.25-39.0625kHz (resonator), 4-20MHz (crystal)
//! - OSCERCLK - same as OSCCLK, made available to RTC, WDOG, and ADC modules
//! - LPOCLK - fixed 1kHz output clock, available to RTC or WDOG modules.
//!
//! ## SIM
//!
//! The SIM System Clock Gating Control Register (SIM_SCGC) register can gate
//! on or off the clock to each module. Before enabling a module, the
//! corresponding SIM_SCGC bit must be set. Conversely, a module must be
//! disabled before turning off its clock. Bus access to an "unclocked"
//! peripheral will generate an error termination (a hard fault?)
//!
//! ## Internal Clock Source Peripheral
//!
//! The ICS peripheral provides clock sources for the MCU. This controls the
//! low-power oscillator and a 1024x multiplying frequency locked loop (FLL).
//! This peripheral is documented in Ch. 20 of KEA64 Sub-family reference
//! manual.
//!
//! ### Operational Modes
//!
//! The ICS has 7 operation modes. These modes determine the source of the
//! clock, if the FLL is bypassed, and if the FLL is turned off for power
//! savings.
//!
//! #### FEI - FLL Engaged, Internal Reference Clock Mode
//!
//! This is the default mode of operation (on reset). This multiplies the asdf
//! 31.25kHz IRC by 1024 (in the FLL) to result in a 16MHz system clock. In
//! order to reach the 40MHz maximum clock speed the sctrim must be
//! stuffed with 0x1FF to yield (approximately) 39.0625kHz IRC clock, which
//! in turn gives a 40MHz system clock in this mode. See `Ics::sctrim`
//! documentation for me detail.
//!
//! #### FEE - FLL Engaged, External Reference mode
//!
//! This mode is used with external resonators or a clock source. Configuration
//! of the external source is handled in the OSC module. The OSC module must
//! output this signal in the range of 31.25 - 39.0625kHz. Like the FEI mode this
//! will yield a system clock of 32 - 40MHz.
//!
//! #### FBI - FLL Bypassed, Interal Reference Clock Mode
//!
//! This mode bypasses the FLL (but leaves it on, for reasons?) and runs the
//! IRC straight into the output. This would set the system clock between
//! 31.25-39.0625kHz This mode would be used to transition from FBILP mode to FEI
//! mode, in order to allow the FLL output to stabilize (maximum accuracy)
//! before the switch.
//!
//! #### FBILP - FLL Disabled, Internal Reference Clock Mode
//!
//! This mode is just like FBI, but the FLL is turned off to save power. The
//! FLL needs time to stabilized after restarting. If the FLL needs to be used
//! again, for best accuracy ICS should be switched to FBI mode and held there
//! until stabilization, then an alternative timing mode can be used.
//!
//!
//! #### FBE - FLL Bypassed, External Reference Mode
//!
//! This mode is like FBI, except the external reference clock as provided by
//! the OSC module is used instead of the IRC. This mode would be used with a
//! high frequency crystal or clock. This mode would be used to transition from
//! FBELP mode to FEE mode, in order to allow the FLL output to stabilize
//! (maximum accuracy) before the switch.
//!
//! #### FBELP - FLL Disabled, External Reference Mode
//!
//! This mode is to FBE mode as FBILP is to FBI mode. And they said the SAT/ACT
//! was a useless test...
//!
//! #### STOP - FLL Disabled? No clock output
//!
//! This mode is entered whenever the MCU enters the STOP state. ICSIRCLK could
//! be active if enabled (IRCLKEN set) and allowed to work in stop mode
//! (IREFSTEN set).
//!
//! ### ICSFFCLK - Fixed Frequency Clock
//!
//! Only available in the FBE and FBELP modes. ICSFFCLK < ICSOUT/4. Provides
//! the input of the FLL as an output clock source. Passes through RDIV. IREFS
//! must be set to select the IRC.

use crate::pac::ICS;
use core::marker::PhantomData;

/// struct that represents the state of the System Clock output ICSOUT
pub struct SystemClock<Source, FLL, LowPower> {
    _source: PhantomData<Source>,
    _fll: PhantomData<FLL>,
    _low_power: PhantomData<LowPower>,
}

// needed for Copy
impl<Source, FLL, LowPower> Clone for SystemClock<Source, FLL, LowPower> {
    fn clone(&self) -> SystemClock<Source, FLL, LowPower> {
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
}

// Need to implement Copy so we can "move" the SystemClock type around. This is
// needed to be able to redefine the peripheral.
impl<Source, FLL, LowPower> Copy for SystemClock<Source, FLL, LowPower> {}

/// Internal Reference Clock source
pub struct IntRefClock;

/// External Reference Clock source
pub struct ExtRefClock;

/// FLL Disabled, no clock source provided to MCU.
///
/// This mode is entered whenever the MCU enters the STOP state. ICSIRCLK could
/// be active if enabled (IRCLKEN set) and allowed to work in stop mode
/// (IREFSTEN set).
pub struct Stop;

/// Frequency-Locked Loop (1024x mutliplier) Used
pub struct FllEnabled;

/// FLL bypassed
///
/// CLKS mux needs to be set to select the appropriate Internal/External Source
pub struct FllBypassed;

/// Low Power Mode Enabled
pub struct LpEnabled;

/// Low Power Mode Disabled
pub struct LpDisabled;

/// In FBILP mode
impl SystemClock<IntRefClock, FllBypassed, LpEnabled> {
    /// Can only transition to FBI mode
    pub fn into_fbi(self) -> SystemClock<IntRefClock, FllBypassed, LpDisabled> {
        unsafe {
            // Disable Low Power (turn on FLL, leave disconnected)
            &(*ICS::ptr()).c2.write(|w| w.lp()._0());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
}

/// In FBELP mode
impl SystemClock<ExtRefClock, FllBypassed, LpEnabled> {
    /// Can only transition to FBE mode
    pub fn into_fbe(self) -> SystemClock<ExtRefClock, FllBypassed, LpDisabled> {
        unsafe {
            // Disable Low Power (turn on FLL, leave disconnected)
            &(*ICS::ptr()).c2.modify(|_, w| w.lp()._0());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
}

impl<Source, Fll> SystemClock<Source, Fll, LpDisabled> {
    /// Transition from any non-lowpower mode
    pub fn into_fei(self) -> SystemClock<IntRefClock, FllEnabled, LpDisabled> {
        unsafe {
            let ics = &(*ICS::ptr());
            // may be faster to check if IREFST is set first. @TODO verify
            // switch IREFS to use IRC
            ics.c1.modify(|_, w| w.irefs()._1());

            // wait for IREFST to indicate mux is set to IRC
            while !ics.s.read().irefst().is_1() {
                cortex_m::asm::nop(); // may not be needed? @TODO verify
            }

            // Change CLKS to select FLL output
            ics.c1.modify(|_, w| w.clks()._00());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }

    /// Transition from any non-lowpower mode
    ///
    /// Technically, fbi mode isn't active until CLKST shows it. It's up to the
    /// caller to decide if they want to wait
    pub fn into_fbi(self) -> SystemClock<IntRefClock, FllBypassed, LpDisabled> {
        unsafe {
            &(*ICS::ptr()).c1.modify(|_, w| w.clks()._01());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }

    /// Transition from any non-lowpower mode.
    pub fn into_fee(self) -> SystemClock<ExtRefClock, FllEnabled, LpDisabled> {
        unsafe {
            let ics = &(*ICS::ptr());

            // Switch IREFS to use External ref
            ics.c1.modify(|_, w| w.irefs()._0());

            // Wait for mux to switch
            while !ics.s.read().irefst().is_0() {
                cortex_m::asm::nop();
            }

            // Set CLKS to select FLL output
            ics.c1.modify(|_, w| w.clks()._00());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
    /// Transition from any non-lowpower mode
    ///
    /// Technically, fbe mode isn't active until CLKST shows it. It's up to the
    /// caller to decide if they want to wait
    pub fn into_fbe(self) -> SystemClock<ExtRefClock, FllBypassed, LpDisabled> {
        unsafe {
            &(*ICS::ptr()).c1.modify(|_, w| w.clks()._10());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
}

/// In FBE or FBI mode, depending on Source
impl<Source> SystemClock<Source, FllBypassed, LpDisabled> {
    /// Transition to FBELP or FBILP mode, whichever is the low power version
    /// of the current mode.
    ///
    /// If in FBE mode, calling this method will transition to FBELP.
    /// Likewise for FBI and FBILP.
    pub fn into_low_power(self) -> SystemClock<Source, FllBypassed, LpEnabled> {
        unsafe {
            &(*ICS::ptr()).c2.modify(|_, w| w.lp()._1());
        }
        SystemClock {
            _source: PhantomData,
            _fll: PhantomData,
            _low_power: PhantomData,
        }
    }
}

// Rough First pass at mapping out stuff. Delete after verifying.
// // Modes of operation.
// // Control whether the internal or external clock is connected to the Frequency-
// // Locked Loop (FLL) node, and if the FLL output is used as a clock source.
// //
// // 7 modes of operation:
// // FEI - FLL out selected,
//
// // These are the states of the clock source select MUX.
// // Note: CLKS = 0b11 exists, but is reserved, changes to 0b00
// /// FLL engaged, internal
// pub struct ClksFll; // 0b00, default
//
// /// Internal Ref Clock
// pub struct ClksIrc; // 0b01
//
// /// External Ref Clock
// pub struct ClksErc; // 0b10
//
// // IRCLKEN, FLL control states.
// //
// // These control the behavior of the FLL when the FLL is not selected by the
// // clock source select mux.
//
// /// Keep the FLL enabled and running when bypassed. Default
// pub struct NormalPower;
//
// /// Disabled the FLL when bypassed. Low power.
// pub struct LowPower;
//
// // IREFS, FLL Input mux states
// //
// // These determine which source is used as an input for the FLL
//
// /// internal clock source is connected to FLL. Default
// pub struct FllSrcInternal; // 0b1, Default
//
// /// External clock source is connected to FLL
// pub struct FllSrcExternal; //0b0
//
// // ICSIRCLK output settings
// //
// // Determines if the internal reference clock is output to ICSIRCLK
//
// /// No ICSIRCLK, Default
// pub struct ICSIRCLKDisabled;
//
// /// ICSIRCLK output enabled
// pub struct ICSIRCLKEnabled;
//
// /// Stop Internal Reference clock in Stop mode. Default
// pub struct NoIrcInStop;
//
// /// Run the Internal Reference Clock in stop mode if ICSIRCLK enabled or IRC
// /// as clock source.
// pub struct IrcRunInStop;
//
// /// Disable clock monitor (default)
// pub struct ClockMonDisabled;
//
// /// Enable clock monitor to reset when external clock is lost.
// pub struct ClockMonEnabled;
//
//
// pub struct S {
//     _0: (),
// }
//
// // pub struct Config<CLKS_MODE, LP_MODE, IREFS_MODE, IRCLKEN_MODE, IREFSTEN_MODE, LOLIE_MODE> {
// //     clks: ClockSource<CLKS_MODE>,
// //     lp: LowPower<LP_MODE>,
// //     irefs: FllSource<IREFS_MODE>,
// //     irclken: ICSIRCLK<IRCLKEN_MODE>,
// //     irefsten: IrcRunInStop<IREFSTEN_MODE>,
// //     lolie: LossOfLockInt<LOLIE_MODE>,
// //     rdiv: Option<u32>,
// //     bdiv: Option<u32>,
// // }

/// Custom Error Types
pub enum Error {
    /// Value was not valid for the function it was passed into.
    InvalidValue,
}

/// State of ICSIRCLK (output of the ICS module's Internal Reference CLocK)
pub struct IrcOut<MODE> {
    _mode: PhantomData<MODE>,
}

/// ICSIRCLK defaults to Disabled
pub type DefaultIrcOut = Stopped;

/// ICSIRCLK disabled
pub struct Stopped;

/// ICSIRCLK Enabled, but is Disabled on entry to Stop Mode.
pub struct Stoppable;

/// ICSIRCLK Always Enabled, even in Stop mode.
pub struct Unstoppable;

/// Grabs ownership of ICS from the PAC.
pub trait ICSExt {
    /// ICS struct
    type Ics;
    /// grab the Peripheral from PAC;
    fn constrain(self) -> Self::Ics;
}

/// HAL struct for the Internal clock system.
pub struct Ics {
    /// The state of the source of the system clock
    ///
    /// The ICS defaults to the FEI mode of operation. In this mode, ICSOUT,
    /// which is the system clock, is
    /// 16MHz = IRC * FLL * BDIV = 31.25kHz * 1024 / 2; at default values.
    pub system_clock: SystemClock<IntRefClock, FllEnabled, LpDisabled>,

    /// The state of ICSIRCLK output
    pub irc_out: IrcOut<DefaultIrcOut>,
}

impl ICSExt for ICS {
    type Ics = Ics;
    fn constrain(self) -> Ics {
        Ics {
            system_clock: SystemClock {
                _source: PhantomData,
                _fll: PhantomData,
                _low_power: PhantomData,
            },
            irc_out: IrcOut { _mode: PhantomData },
        }
    }
}
impl Ics {
    /// Read the IRC's trim value.
    ///
    /// This SCTRIM is used to tweak the frequency of the Internal
    /// Reference Clock. This factory trimmed value is loaded from
    /// nonvolatile memory on boot to set the IRC to be 31.25kHZ (yielding
    /// a 16MHz system clock in FEI mode). Note that this interface
    /// includes the SCFTRIM bit, which contains the LSB of the value used
    /// by the ICS module.
    pub fn sctrim(self) -> u16 {
        unsafe {
            let ics = &(*ICS::ptr());
            ((ics.c3.read().bits() as u16) << 1) + ics.c4.read().scftrim().bit() as u16
        }
    }

    /// Set the IRC's trim value.
    ///
    /// This SCTRIM is used to tweak the frequency of the Internal
    /// Reference Clock. This factory trimmed value is loaded from
    /// nonvolatile memory on boot to set the IRC to be 31.25kHZ (yielding
    /// a 16MHz system clock in FEI mode). Note that this interface
    /// includes the SCFTRIM bit, which contains the LSB of the value used
    /// by the ICS module.
    ///
    /// Write 0x1FF to max out the system clock frequency to (close to)
    /// 40MHz.
    ///
    /// If system_clock configured in FEI mode this should probably wait for
    /// the FLL to stabilize (LOCK)
    pub fn set_sctrim(self, value: u16) -> Result<(), Error> {
        if value > 0x1FF {
            return Err(Error::InvalidValue);
        }
        unsafe {
            let ics = &(*ICS::ptr());
            ics.c3.write(|w| w.bits((value >> 1) as u8));
            ics.c4.modify(|_, w| w.scftrim().bit((value & 0x1FE) == 1));
        }
        Ok(())
    }

    /// Return the ICS Status Struct.
    ///
    /// It's a PAC thing. deal with it until I make this prettier.
    pub fn status(&mut self) -> &pac::ics::S {
        unsafe { &(*ICS::ptr()).s }
    }

    // ran out of time to deal with this.
    // /// Read the BDIV value
    // ///
    // /// BDIV divides the output of the ICS (in any mode) by powers of 2.
    // /// ICSOUT = CLKSoutput / (2 ** n + 1)
    // /// The value at reset is 1 (ICSOUT = CLKSoutput / 2)
    // pub fn bdiv(self) -> u8 {
    //     unsafe {
    //         &(*ICS::ptr()).c2.read().bdiv().bits()
    //     }
    // }

    /// Set the BDIV value
    ///
    /// BDIV divides the output of the ICS (in any mode) by powers of 2.
    /// ICSOUT = CLKSoutput / (2 ** n + 1)
    /// The value at reset is 1 (ICSOUT = CLKSoutput / 2)
    ///
    /// Note that by default the bus clock and system clock run at the same
    /// multiplier, but the max bus clock is 20MHz. SIM_BUSDIV must be set
    /// appropriately to ensure this limit is not exceeded before BDIV is set
    /// to 1.
    // Once SIM is implemented, add checks to prevent overclocking the bus.
    pub fn set_bdiv(self, div: u8) -> Result<(), Error> {
        if div > 0b111 {
            return Err(Error::InvalidValue);
        }

        unsafe {
            &(*ICS::ptr()).c2.modify(|_, w| w.bdiv().bits(div));
        }
        Ok(())
    }

    /// Enable/Disable the Internal Reference clock output on ICSIRCLK
    pub fn output_irc(self, output: bool) {
        unsafe {
            &(*ICS::ptr()).c1.modify(|_, w| w.irclken().bit(output));
        }
    }
}
