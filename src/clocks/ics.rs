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

use crate::gpio::gpioa::{PTB6, PTB7};
use crate::pac::{ICS, OSC};
use core::marker::PhantomData;

/// Custom Error Types
pub enum Error {
    /// Value was not valid for the function it was passed into.
    InvalidValue,

    /// Pin has already been returned. Can't give it again
    NoPin,
}

// Common state-types

/// Peripheral doesn't care about this type.
///
/// It will be set to something useful later.
pub struct DontCare;

/// Clock feature is disabled
pub struct Stopped;

/// Clock feature is Enabled, but is Disabled on entry to Stop Mode.
pub struct Running;

/// Clock feature is always Enabled, even in Stop mode.
pub struct Unstoppable;

/// Grabs ownership of ICS from the PAC.
pub trait ICSExt {
    /// ICS struct
    type Ics;
    /// grab the Peripheral from PAC;
    fn split(self) -> Self::Ics;
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

    /// The state of the frequency-locked loop monitor
    pub lock_status: LockStatus<Poll>,
}

impl ICSExt for ICS {
    type Ics = Ics;
    fn split(self) -> Ics {
        Ics {
            system_clock: SystemClock {
                _source: PhantomData,
                _fll: PhantomData,
                _low_power: PhantomData,
            },
            irc_out: IrcOut { _mode: PhantomData },
            lock_status: LockStatus { _mode: PhantomData },
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
    pub fn sctrim(&self) -> u16 {
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
    pub fn set_sctrim(&self, value: u16) -> Result<(), Error> {
        if value > 0x1FF {
            return Err(Error::InvalidValue);
        }
        unsafe {
            let ics = &(*ICS::ptr());
            ics.c3.write(|w| w.bits((value >> 1) as u8));
            ics.c4.modify(|_, w| w.scftrim().bit((value & 1) == 1));
        }
        Ok(())
    }

    /// Returns which reference clock is currently active.
    ///
    /// Indicates the current clock mode, either Internal or External Reference
    /// Clock 0 = ExtRefClock, 1 = IntRefClock.
    ///
    /// This should probably return an Enum value, or the sub-types used in the
    /// SystemClock struct. For now this just returns the field value until
    /// this can be rethought.
    // this should probably also get a better name.
    // Is this even needed since we know this from the current type of
    // SystemClock's Source sub-type? The into_whatever methods should be
    // waiting for this to switch before returning.
    pub fn internal_reference(&self) -> bool {
        unsafe { (*ICS::ptr()).s.read().irefst().bit() }
    }

    /// Returns which clock mode is currently active.
    ///
    /// 0 = FllEnabled
    /// 1 = IntRefClock,FllBypassed (FBI mode)
    /// 2 = ExtRefClock,FllBypassed (FBE mode)
    /// 3 = Reserved, not used.
    ///
    /// This should probably return an Enum value, or the sub-types used in the
    /// SystemClock struct. For nwo this just returns the field value until
    /// this can be rethough
    // is this even needed? see `internal_reference`
    pub fn clock_mode(&self) -> u8 {
        unsafe { (*ICS::ptr()).s.read().clkst().bits() }
    }
}

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
            (*ICS::ptr()).c2.modify(|_, w| w.lp()._0());
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

/// Any Internal mode (cannot be set in FEE, FBE, or FBELP mode)
impl<Fll, LowPower> SystemClock<IntRefClock, Fll, LowPower> {
    /// Set the RDIV value
    ///
    /// RDIV divides the output of the external reference clock by powers of 2.
    /// RDIV_OUT = OSC_OUT / (2 ** n + 1)
    /// The value at reset is 0 (RDIV_OUT = OSC_OUT)
    ///
    /// Note that per RDIV definition (pg272 in ICS_C1 of KEA64 ref man) this
    /// cannot be changed while the SystemClock is in FEE or FBE mode. To use
    /// set this first, then transition to FBE or FEE mode.
    pub fn rdiv(&self, div: u8) -> Result<(), Error> {
        if div > 0b111 {
            return Err(Error::InvalidValue);
        }

        // values 0b110 & 0b111 are undefined / reserved. When osc is in high
        // range mode.
        if unsafe { (*OSC::ptr()).cr.read().range().is_1() } && (div >= 0b110) {
            return Err(Error::InvalidValue);
        }

        unsafe {
            &(*ICS::ptr()).c1.modify(|_, w| w.rdiv().bits(div));
        }
        Ok(())
    }
}

/// Any External mode (cannot be set in FEI, FBI, or FBILP)
impl<Fll, LowPower> SystemClock<ExtRefClock, Fll, LowPower> {
    /// Enable/Disable Clock Monitor.
    ///
    /// When enabled, the MCU will reset if it loses the external clock signal
    pub fn clock_monitor(&self, enable: bool) {
        unsafe {
            (*ICS::ptr()).c4.modify(|_, w| w.cme().bit(enable));
        }
    }
}

/// For any state of SystemClock
impl<Source, Fll, LowPower> SystemClock<Source, Fll, LowPower> {
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
}

/// State of ICSIRCLK (output of the ICS module's Internal Reference CLocK)
pub struct IrcOut<MODE> {
    _mode: PhantomData<MODE>,
}

/// ICSIRCLK defaults to Disabled. Uses Stopped/Running/Unstoppable types
pub type DefaultIrcOut = Stopped;

impl IrcOut<Stopped> {
    /// Enable the Internal Reference clock output on ICSIRCLK
    pub fn into_running(self) -> IrcOut<Running> {
        unsafe {
            (*ICS::ptr()).c1.modify(|_, w| w.irclken()._1());
        }
        IrcOut { _mode: PhantomData }
    }
}

impl IrcOut<Running> {
    /// Disable the Internal Reference Clock output on ICSIRCLK
    pub fn into_stopped(self) -> IrcOut<Stopped> {
        unsafe {
            (*ICS::ptr()).c1.modify(|_, w| w.irclken()._0());
        }
        IrcOut { _mode: PhantomData }
    }

    /// Allow Internal reference clock and ICSIRCLK to continue running during
    /// Stop mode.
    ///
    /// This allows other peripherals such as the RTC and Watchdog to continue
    /// using this clock. This mode is also recommended if the systemclock is
    /// using FEI, FBI, of FBILP and needs to restart quickly after Stop mode.
    pub fn into_unstoppable(self) -> IrcOut<Unstoppable> {
        unsafe {
            (*ICS::ptr()).c1.modify(|_, w| w.irefsten()._1());
        }
        IrcOut { _mode: PhantomData }
    }
}

impl IrcOut<Unstoppable> {
    /// make stoppable, but keep ICSIRCLK running.
    pub fn into_running(self) -> IrcOut<Running> {
        unsafe {
            (*ICS::ptr()).c1.modify(|_, w| w.irefsten()._0());
        }
        IrcOut { _mode: PhantomData }
    }

    /// Make stoppable and disable ICSIRCLK.
    pub fn into_stopped(self) -> IrcOut<Stopped> {
        let temp = self.into_running();
        temp.into_stopped()
    }
}

/// Monitor the frequency-locked loop for loss of lock
///
/// This may make more sense to be a method of SystemClock, so that it can
/// only be used while the FLL is active. The default mode is polled (Poll).
pub struct LockStatus<MODE> {
    _mode: PhantomData<MODE>,
}

/// FLL monitor must be polled to determine if lock has been lost.
pub struct Poll;

/// FLL monitor generates an interrupt when lock is lossed
pub struct Interrupt;

/// Any state of lock status
impl<MODE> LockStatus<MODE> {
    /// Is the FLL currently locked?
    pub fn locked(&self) -> bool {
        unsafe { (*ICS::ptr()).s.read().lock().bit() }
    }

    /// Has the FLL lost its lock since the last time it was acknowledged
    pub fn lock_lost(&self) -> bool {
        let ics = unsafe { &(*ICS::ptr()) };
        let ret_val: bool = ics.s.read().lols().bit();

        // Acknowledge the loss of lock (if happened)
        if ret_val {
            // write 1 to clear
            ics.s.modify(|_, w| w.lols()._1());
        }

        ret_val
    }
}

impl LockStatus<Poll> {
    /// Trigger an Interrupt on loss of lock
    ///
    /// Polling is still functional in the Interrupt state, if needed to verify
    /// no loss occurred or current state.
    pub fn into_interrupt(self) -> LockStatus<Interrupt> {
        unsafe {
            (*ICS::ptr()).c4.modify(|_, w| w.lolie()._1());
        }
        LockStatus { _mode: PhantomData }
    }
}

impl LockStatus<Interrupt> {
    /// Disable interrupt on loss of lock.
    pub fn into_poll(self) -> LockStatus<Poll> {
        unsafe {
            (*ICS::ptr()).c4.modify(|_, w| w.lolie()._0());
        }
        LockStatus { _mode: PhantomData }
    }
}

/// External Oscillator Peripheral
///
/// Oscillator is Stopped (Disabled) by default.
pub struct Osc<Status, OscType, Range, Gain, ExtalPinState, XtalPinState> {
    _status: PhantomData<Status>,
    _osc_type: PhantomData<OscType>,
    _range: PhantomData<Range>,
    _gain: PhantomData<Gain>,
    extal_pin: Option<PTB7<ExtalPinState>>,
    xtal_pin: Option<PTB6<XtalPinState>>,
}

/// Grabs ownership of OSC from the PAC
pub trait OSCExt {
    /// This module's state struct
    type Osc;

    /// grab the peripheral from the PAC and return the state struct
    fn split(self) -> Self::Osc;
}

impl OSCExt for OSC {
    type Osc = Osc<Stopped, ExtClock, LowRange, VariableGain, DontCare, DontCare>;
    fn split(self) -> Osc<Stopped, ExtClock, LowRange, VariableGain, DontCare, DontCare> {
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData, //ExtClock is default, but don't care.
            _range: PhantomData,    // LowRange is default, but don't care.
            _gain: PhantomData,     // Variable (low power) is default, but don't care.
            extal_pin: None,
            xtal_pin: None,
        }
    }
}

/// External Clock used.
pub struct ExtClock;

/// External Oscillator or Resonator used
pub struct ExtOsc;

/// External Oscillator set for low range (roughly 32kHZ).
pub struct LowRange;

/// External oscillator set for high range (4-20MHz).
pub struct HighRange;

/// Oscillator module uses Variable gain to save power
pub struct VariableGain;

/// Oscillator module uses High Gain to get rail-to-rail oscillations.
pub struct HighGain;

impl<OscType, Range, Gain, ExtalPinState, XtalPinState>
    Osc<Stopped, OscType, Range, Gain, ExtalPinState, XtalPinState>
{
    /// Start-up the oscillator. Blocks until the oscillator is running.
    pub fn into_running(
        self,
        extal_pin: PTB7<ExtalPinState>,
    ) -> Osc<Running, OscType, Range, Gain, ExtalPinState, XtalPinState> {
        let osc = unsafe { &(*OSC::ptr()) };

        // start the oscillator
        osc.cr.modify(|_, w| w.oscen().set_bit());

        // block until it's stable
        while osc.cr.read().oscinit().is_0() {
            cortex_m::asm::nop();
        }

        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: Some(extal_pin),
            xtal_pin: self.xtal_pin,
        }
    }

    /// Returns the pin used for the EXTAL input
    ///
    /// @TODO what is the state of the pin after decoupling from OSC
    /// peripheral?
    pub fn release_extal_pin(
        self,
    ) -> (
        Result<PTB7<ExtalPinState>, Error>,
        Osc<Stopped, OscType, Range, Gain, DontCare, XtalPinState>,
    ) {
        (
            self.extal_pin.ok_or(Error::NoPin),
            Osc {
                _status: PhantomData,
                _osc_type: PhantomData,
                _range: PhantomData,
                _gain: PhantomData,
                extal_pin: None,
                xtal_pin: self.xtal_pin,
            },
        )
    }
}

impl<OscType, Range, Gain, ExtalPinState, XtalPinState>
    Osc<Running, OscType, Range, Gain, ExtalPinState, XtalPinState>
{
    /// Allow the OSC peripheral to continue running when the system goes into
    /// STOP mode.
    pub fn into_unstoppable(
        self,
    ) -> Osc<Unstoppable, OscType, Range, Gain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.oscsten().set_bit());
        }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }

    /// Stop (disable) the OSC peripheral
    ///
    /// @TODO think of ways to not ruin someones day if they are using OSC_OUT
    /// as a reference clock.
    pub fn into_stopped(self) -> Osc<Stopped, OscType, Range, Gain, ExtalPinState, XtalPinState> {
        let osc = unsafe { &(*OSC::ptr()) };
        osc.cr.modify(|_, w| w.oscen().clear_bit());
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

impl<OscType, Range, Gain, ExtalPinState, XtalPinState>
    Osc<Unstoppable, OscType, Range, Gain, ExtalPinState, XtalPinState>
{
    /// Allow the OSC peripheral to be stopped when the system goes into STOP
    /// mode.
    pub fn into_running(self) -> Osc<Running, OscType, Range, Gain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.oscsten().clear_bit());
        }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }

    /// Stop (disable) the OSC peripheral
    ///
    /// @TODO think of ways to not ruin someones day if they are using OSC_OUT
    /// as a reference clock.
    pub fn into_stopped(self) -> Osc<Stopped, OscType, Range, Gain, ExtalPinState, XtalPinState> {
        let osc = unsafe { &(*OSC::ptr()) };
        osc.cr.modify(|_, w| w.oscen().clear_bit());
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

impl<Status, Range, Gain, ExtalPinState, XtalPinState>
    Osc<Status, ExtOsc, Range, Gain, ExtalPinState, XtalPinState>
{
    /// Change to External Clock mode (not resonator or crystal)
    ///
    /// Consumes Pin PTB
    pub fn into_ext_clock(self) -> Osc<Status, ExtClock, Range, Gain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.oscos()._0());
        }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

impl<Status, Range, Gain, ExtalPinState, XtalPinState>
    Osc<Status, ExtClock, Range, Gain, ExtalPinState, XtalPinState>
{
    /// Change to External oscillator / resonator mode
    ///
    /// Consumes Pins PTB6:PTB7
    pub fn into_ext_osc(self) -> Osc<Status, ExtOsc, Range, Gain, ExtalPinState, XtalPinState> {
        unsafe { (*OSC::ptr()).cr.modify(|_, w| w.oscos()._1()) }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }

    /// Returns the pin used for the XTAL input.
    ///
    /// This pin is only needed for the external oscillator / resonator mode.
    ///
    /// @TODO what is the state of the pin after decoupling from OSC
    /// peripheral?
    pub fn release_xtal_pin(
        self,
    ) -> (
        Result<PTB6<XtalPinState>, Error>,
        Osc<Status, ExtClock, Range, Gain, ExtalPinState, DontCare>,
    ) {
        (
            self.xtal_pin.ok_or(Error::NoPin),
            Osc {
                _status: PhantomData,
                _osc_type: PhantomData,
                _range: PhantomData,
                _gain: PhantomData,
                extal_pin: self.extal_pin,
                xtal_pin: None,
            },
        )
    }
}

// Osc Module is stopped
impl<OscType, Gain, ExtalPinState, XtalPinState>
    Osc<Stopped, OscType, LowRange, Gain, ExtalPinState, XtalPinState>
{
    /// Set to high range.
    ///
    /// 4-20MHz input. Range must be set while the Osc module is disabled.
    pub fn into_high_range(
        self,
    ) -> Osc<Stopped, OscType, HighRange, Gain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.range().set_bit());
        }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

// Osc Module is stopped
impl<OscType, Gain, ExtalPinState, XtalPinState>
    Osc<Stopped, OscType, HighRange, Gain, ExtalPinState, XtalPinState>
{
    /// Set to low range.
    ///
    /// Roughly 32kHZ. Range must be set while the Osc module is disabled.
    pub fn into_low_range(
        self,
    ) -> Osc<Stopped, OscType, HighRange, Gain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.range().clear_bit());
        }
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

// I'm pretty sure that there's no harm in changing this in External clock
// mode, if so then change generic OscType to ExtOsc
impl<Status, OscType, Range, ExtalPinState, XtalPinState>
    Osc<Status, OscType, Range, HighGain, ExtalPinState, XtalPinState>
{
    /// Set the Oscillator to use Variable Gain.
    ///
    /// This mode adjusts the oscillator gain to get an acceptable amplitude,
    /// minimizing power used.
    pub fn into_variable_gain(
        self,
    ) -> Osc<Status, OscType, Range, VariableGain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.hgo().clear_bit());
        };
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}

impl<Status, OscType, Range, ExtalPinState, XtalPinState>
    Osc<Status, OscType, Range, HighGain, ExtalPinState, XtalPinState>
{
    /// Set the Oscillator to use High Gain
    ///
    /// This mode attempts to get rail-to-rail output from the oscillator or
    /// resonator.
    pub fn into_high_gain(
        self,
    ) -> Osc<Status, OscType, Range, HighGain, ExtalPinState, XtalPinState> {
        unsafe {
            (*OSC::ptr()).cr.modify(|_, w| w.hgo().set_bit());
        };
        Osc {
            _status: PhantomData,
            _osc_type: PhantomData,
            _range: PhantomData,
            _gain: PhantomData,
            extal_pin: self.extal_pin,
            xtal_pin: self.xtal_pin,
        }
    }
}
