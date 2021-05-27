//! # Clocks meta-module
//!
//! Right now this is just docs talking about how the clock modules work
//! together. This will probably turn into a mod containing the other clock
//! mods.
//!
//! ## Architecture
//!
//! The ISC module selects and/or muxes the clock sources to output the system
//! clock sources. the SIM module handles dividing and gating of those clock
//! sources. If an external oscillator/resonator is used, the OSC module passes
//! those signals to the ISC and SIM modules as needed.
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
//! - ICSIRCLK - ISC output of the 32kHZ-ish internal reference clock. Can be
//! source for RTC or WDOG modules. 31.25-39.0625kHz
//! - ICSOUTCLK - output of ISC module's output MUX, drives core, platform,
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
//! low-power oscillator and a frequency locked loop (FLL).
//! This peripheral is documented in Ch. 20 of KEA64 Sub-family reference
//! manual.
//!

use core::marker::PhantomData;

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

/// Holds one of the 7 available ICS modes of operation
pub struct ClockSource<MODE> {
    _mode: PhantomData<MODE>,
}

/// The ICS defaults to the FEI mode of operation. at 32MHz (SCTRIM defaults
/// back to factory programmed trim value, IRC = 31.25kHz)
pub type DefaultClockSource = Fei;

/// FLL Engaged, Internal Reference clock mode
///
/// FLL is the 1024x Frequency-Locked Loop (clock multiplier)
/// This makes the 31.25kHZ - 39.0625kHz IRC into a 32 - 40 MHz System clock
pub struct Fei;

/// FLL Engaged, External mode
///
/// This mode is used with external resonators or a clock source. Configuration
/// of the external source is handled in the OSC module. The OSC module must
/// output this signal in the range of 32 - 39.0625kHz
pub struct Fee;

/// FLL Bypassed, IRC mode
///
/// This mode bypasses the FLL (but leaves it on, for reasons?) and runs the
/// IRC straight into the output. This would set the system clock between
/// 32-39.0625kHz This mode would be used to transition from FBILP mode to FEI
/// mode, in order to allow the FLL output to stabilize (maximum accuracy)
/// before the switch.
pub struct Fbi;

/// FLL Disabled and Bypassed, IRC mode
///
/// This mode is the same as FBI, but the FLL is turned off to save power
pub struct FbiLp;

/// FLL Bypased, External mode
///
/// This mode is like FBI, except the external reference clock as provided by
/// the OSC module is used instead of the IRC. This mode would be used with a
/// high frequency crystal or clock. This mode would be used to transition from
/// FBELP mode to FEE mode, in order to allow the FLL output to stabilize
/// (maximum accuracy) before the switch.
pub struct Fbe;

/// FLL Disabled and Bypassed, External Mode
///
/// This mode is to FBE mode as FBILP is to FBI mode. And they said the SAT/ACT
/// was a useless test...
pub struct FbeLp;

/// FLL Disabled, no clock source provided to MCU.
///
/// This mode is entered whenever the MCU enters the STOP state. ICSIRCLK could
/// be active if enabled (IRCLKEN set) and allowed to work in stop mode
/// (IREFSTEN set).
pub struct Stop;

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
pub trait IcsExt {
    /// grab the Peripheral from PAC;
    fn constrain(self) -> Ics;
}
use crate::pac::ICS;

/// HAL struct for the Internal clock system.
pub struct Ics {
    /// The state of the source of the system clock
    pub clock_source: ClockSource<DefaultClockSource>,

    /// The state of ICSIRCLK output
    pub irc_out: IrcOut<DefaultIrcOut>,
}

impl IcsExt for ICS {
    fn constrain(self) -> Ics {
        Ics {
            clock_source: ClockSource { _mode: PhantomData },
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
    /// a 32MHz system clock in FEI mode). Note that this interface
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
    /// a 32MHz system clock in FEI mode). Note that this interface
    /// includes the SCFTRIM bit, which contains the LSB of the value used
    /// by the ICS module.
    ///
    /// Write 0x1FF to max out the system clock frequency to (close to)
    /// 40MHz.
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
}

impl<MODE> ClockSource<MODE> {
    /// Sets the system clock to Fee mode.
    pub fn into_fee(self) -> ClockSource<Fee> {
        // implementation here.
        ClockSource { _mode: PhantomData }
    }
}
