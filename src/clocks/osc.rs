//! # External Oscillator Peripheral
//!

use crate::gpio::gpioa::{PTB6, PTB7};
use crate::pac::OSC;
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
