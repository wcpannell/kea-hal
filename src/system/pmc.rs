//! # PMC - Power Management Controller
//!
//! The PMC controls the Low Voltage Detect and the ADC bandgap voltage
//! reference functions.
//!
//! TODO: write an example
//!
//! ## Low Voltage Detect
//!
//! ### Low Voltage Detect Reset
//!
//! This feature, when enabled can trigger a reset when the MCU's input voltage
//! drops below a selectable voltage threshold. This functionality can be
//! active in both running and stop modes (default) or in running mode only.
//!
//! ### Low Voltage Warning
//!
//! This feature sets a warning flag bit and optional triggers an interrupt
//! when the warning voltage threshold has been crossed. This threshold is
//! user-selectable with 8 options grouped into sets of 4 which depend on which
//! low voltage detect reset threshold is chosen.
//!
//! ## Bandgap Voltage Reference
//!
//! The PMC contains a field that enables the internal buffer for the
//! bandgap voltage reference used by the ADC adn ACMP peripherals. This is
//! impelemented in those areas.

use crate::{pac::PMC, HALExt};
use core::marker::PhantomData;

/// Error Enumeration
#[repr(u8)]
pub enum Error {
    /// Somehow we accessed more bits than the field can hold
    OutofBounds,
}

impl HALExt for PMC {
    type T = Pmc<Enabled, _2v6>;
    fn split(self) -> Pmc<Enabled, _2v6> {
        Pmc {
            peripheral: self,
            _state: PhantomData,
            _range: PhantomData,
            lv_reset: LVReset {
                _state: PhantomData,
            },
            lv_warn: LVWarn {
                _state: PhantomData,
                _range: PhantomData,
            },
        }
    }
}

/// Datastructure for PMC interface
pub struct Pmc<State, Range> {
    peripheral: PMC,
    // this state may need to be moved into the lv_reset interface if disabling
    // LV Reset does not also disable the LV Warning.
    _state: PhantomData<State>,
    _range: PhantomData<Range>,
    /// Controls when the Low Voltage Detection is active.
    pub lv_reset: LVReset<RunAndStop>,
    /// Interface for the Low Voltage Warning / Interrupt
    pub lv_warn: LVWarn<Flag, Range>,
}
/// The current state doesn't matter for reasons.
///
/// State-type common to several things.
pub struct DontCare;

/// The function is Enabled.
///
/// State-type common to several things.
pub struct Enabled;

/// The function is Disabled.
///
/// State-type common to several things.
pub struct Disabled;

/// The low voltage warning is a flag only.
///
/// The flag must be polled to detect if the low voltage warning threshold has
/// been crossed.
pub struct Flag;

/// The low voltage warning triggers an interrupt (and a flag).
pub struct Interrupt;

/// The low voltage detection (for reset) is active only while MCU is in a run
/// mode
pub struct RunOnly;

/// The low voltage detection (for reset) is active while the MCU is in run and
/// stop modes.
pub struct RunAndStop;

/// Interface that controls when the Low Voltage Detection is active
pub struct LVReset<State> {
    _state: PhantomData<State>,
}

/// Interface for the low voltage warning / interrupt
pub struct LVWarn<State, Range> {
    _state: PhantomData<State>,
    _range: PhantomData<Range>,
}

/// V_LVDH threshold = 4.3V typ. for all KEA parts.
pub struct _4v3;
/// V_LVDL threshold = 2.61V typ. for all KEA parts
pub struct _2v6;

/// Low Voltage Warning Threshold, Low range (set by V_LVDx).
///
/// Threshold voltages verified same for all KEA parts
#[repr(u8)]
pub enum LvwLow {
    /// LVW1
    _2v7 = 0,
    /// LVW2
    _2v8 = 1,
    /// LVW3
    _2v9 = 2,
    /// LVW4
    _3v0 = 3,
}

/// Low Voltage Warning Threshold, High range (set by V_LVDx).
///
/// Threshold voltages verified same for all KEA parts
#[repr(u8)]
pub enum LvwHigh {
    /// LVW1
    _4v4 = 0,
    /// LVW2
    _4v5 = 1,
    /// LVW3
    _4v6 = 2,
    /// LVW4
    _4v7 = 3,
}

impl<Range> Pmc<Enabled, Range> {
    /// Disable LVD logic and threshold checking
    ///
    /// This field can only be written once after reset, meaning that once
    /// disabled, it cannot be re-enabled. It is automatically re-enabled on
    /// reset.
    pub fn into_disabled(self) -> Pmc<Disabled, Range> {
        self.peripheral.spmsc1.modify(|_, w| w.lvde().clear_bit());
        Pmc {
            peripheral: self.peripheral,
            _state: PhantomData,
            _range: PhantomData,
            lv_reset: self.lv_reset,
            lv_warn: self.lv_warn,
        }
    }
}

impl Pmc<Enabled, _2v6> {
    /// Shift to high range
    pub fn into_4v3(self) -> Pmc<Enabled, _4v3> {
        self.peripheral.spmsc2.modify(|_, w| w.lvdv().set_bit());
        Pmc {
            peripheral: self.peripheral,
            _state: PhantomData,
            _range: PhantomData,
            lv_reset: self.lv_reset,
            lv_warn: self.lv_warn.into_high_range(),
        }
    }
}

impl Pmc<Enabled, _4v3> {
    /// Shift to low range
    pub fn into_4v3(self) -> Pmc<Enabled, _2v6> {
        self.peripheral.spmsc2.modify(|_, w| w.lvdv().set_bit());
        Pmc {
            peripheral: self.peripheral,
            _state: PhantomData,
            _range: PhantomData,
            lv_reset: self.lv_reset,
            lv_warn: self.lv_warn.into_low_range(),
        }
    }
}

impl<State, Range> LVWarn<State, Range> {
    /// Set to high range
    ///
    /// Called by PMC range function. passes knowledge of the higher level
    /// range here. There is probably a better way to do this
    pub fn into_high_range(self) -> LVWarn<State, _4v3> {
        LVWarn {
            _state: PhantomData,
            _range: PhantomData,
        }
    }
    /// Set to low range
    ///
    /// Called by PMC range function. passes knowledge of the lower level
    /// range here. There is probably a better way to do this
    pub fn into_low_range(self) -> LVWarn<State, _2v6> {
        LVWarn {
            _state: PhantomData,
            _range: PhantomData,
        }
    }

    /// Check for warning
    pub fn warning(&self) -> bool {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc1.read().lvwf().bit()
    }

    /// Acknowledge warning
    pub fn clear_warning(&self) {
        unsafe { (*PMC::ptr()).spmsc1.modify(|_, w| w.lvwack().bit(true)) };
    }
}

impl<Range> LVWarn<Flag, Range> {
    /// Interrupt on Low Voltage Warning
    pub fn into_interrupt(self) -> LVWarn<Interrupt, Range> {
        unsafe { (*PMC::ptr()).spmsc1.modify(|_, w| w.lvwie().set_bit()) };
        LVWarn {
            _state: PhantomData,
            _range: PhantomData,
        }
    }
}

impl<Range> LVWarn<Interrupt, Range> {
    /// Disable Low Voltage Warning interrupt.
    pub fn into_flag(self) -> LVWarn<Flag, Range> {
        unsafe { (*PMC::ptr()).spmsc1.modify(|_, w| w.lvwie().clear_bit()) };
        LVWarn {
            _state: PhantomData,
            _range: PhantomData,
        }
    }
}

impl<State> LVWarn<State, _2v6> {
    /// adjust warning threshold
    ///
    /// The enum argument type is only valid in low range.
    pub fn set_threshold(&self, threshold: LvwLow) {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc2.modify(|_, w| w.lvwv().bits(threshold as u8));
    }

    /// get the current warning threshold
    pub fn threshold(&self) -> Result<LvwLow, Error> {
        let pmc = unsafe { &(*PMC::ptr()) };
        match pmc.spmsc2.read().lvwv().bits() {
            0 => Ok(LvwLow::_2v7),
            1 => Ok(LvwLow::_2v8),
            2 => Ok(LvwLow::_2v9),
            3 => Ok(LvwLow::_3v0),
            _ => Err(Error::OutofBounds),
        }
    }
}

impl<State> LVWarn<State, _4v3> {
    /// adjust warning threshold
    ///
    /// The enum argument type is only valid in high range.
    pub fn set_threshold(&self, threshold: LvwHigh) {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc2.modify(|_, w| w.lvwv().bits(threshold as u8));
    }

    /// get the current warning threshold
    pub fn threshold(&self) -> Result<LvwHigh, Error> {
        let pmc = unsafe { &(*PMC::ptr()) };
        match pmc.spmsc2.read().lvwv().bits() {
            0 => Ok(LvwHigh::_4v4),
            1 => Ok(LvwHigh::_4v5),
            2 => Ok(LvwHigh::_4v6),
            3 => Ok(LvwHigh::_4v7),
            _ => Err(Error::OutofBounds),
        }
    }
}

impl LVReset<RunAndStop> {
    /// Do not run the Low Voltage Detection Reset during stop mode
    pub fn into_run_only(self) -> LVReset<RunOnly> {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc1.modify(|_, w| w.lvdse().clear_bit());
        LVReset {
            _state: PhantomData,
        }
    }

    /// Disable Reset upon Low Voltage Detection
    ///
    /// This can only be changed one time, and it defaults to enabled.
    pub fn into_disabled(self) -> LVReset<Disabled> {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc1.modify(|_, w| w.lvdre().clear_bit());
        LVReset {
            _state: PhantomData,
        }
    }
}

impl LVReset<RunOnly> {
    /// Run the Low Voltage Detection Reset during stop mode
    pub fn into_run_and_stop(self) -> LVReset<RunAndStop> {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc1.modify(|_, w| w.lvdse().set_bit());
        LVReset {
            _state: PhantomData,
        }
    }

    /// Disable Reset upon Low Voltage Detection
    ///
    /// This can only be changed one time, and it defaults to enabled.
    pub fn into_disabled(self) -> LVReset<Disabled> {
        let pmc = unsafe { &(*PMC::ptr()) };
        pmc.spmsc1.modify(|_, w| w.lvdre().clear_bit());
        LVReset {
            _state: PhantomData,
        }
    }
}
