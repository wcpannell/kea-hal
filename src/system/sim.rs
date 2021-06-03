//! # SIM - System Integration Module
//!
//! ## SIM_SCGC - System Clock Gating Control Register
//!
//! This module controls whether or not a module is connected to the bus clock.
//! This register is controlled by each peripherals respective (software)
//! module.
//!
//! ## SIM_BUSDIV
//!
//! This contains a single bit field that controls whether the bus divider is
//! equal to or half that of the system clock. Because the maximum allowed
//! speed of the bus clock is 20MHz compared to the 40MHz allowed for the
//! system clock, the bus clock divider must be set appropriately if the system
//! clock exceeds 20MHz. By default the system clock is 16MHz.
//!
//!
//! ## SIM_SOPT - System Options Register
//!
//! The SIM holds system configuration options, such as trigger sources, RESET
//! pin usage, etc. All of the options will be implemented in their respective
//! (software) modules. For example reset pin usage is handled by [crate::gpio]
//! , and the bus divider is handled by [crate::clocks].
//!
//! ## UUID
//!
//! the MCU's 64bit UUID.
//!
//! ## SIM_PINSEL - Pin Selection Register
//!
//! The SIM also holds the Pin Selection Register (SIM_PINSEL) which configures
//! the pins that certain peripherals use for inputs and outputs. This
//! functionality is not controlled by this (software) module, but is instead
//! implemented by their repsective modules. For example SIM_PINSEL\[UART0\] is
//! handled within the UART (software) module.

use crate::pac::SIM;

/// A trait to implement a method that will take ownership of the SIM
/// peripheral and return an interface for it.
pub trait SIMExt {
    /// A struct containing fields exposed directly through this peripheral's
    /// interface.
    type Sim;

    /// Grab the SIM peripheral and split out the useful chunks
    fn split(self) -> Self::Sim;
}

impl SIMExt for SIM {
    type Sim = Sim;
    /// Split the SIM peripheral into useful chunks
    fn split(self) -> Sim {
        Sim {
            id: Id { _0: () },
            status: Status { _0: () },
            uuid: UUID,
        }
    }
}

/// Struct containings type structs for the Sim interface.
pub struct Sim {
    /// Getters for the ID portion of the Status and ID register
    pub id: Id,
    /// Getters for the Status portion of the Status and ID register
    pub status: Status,
    /// Getters for the UUID register
    pub uuid: UUID,
}

/// Status type
pub struct Status {
    _0: (),
}

/// ID type
pub struct Id {
    _0: (),
}

/// Enumeration for the number of pins on the device.
///
/// returned by status_id.pinout()
#[repr(u8)]
pub enum DevicePinOuts {
    /// 8 Pin device
    Pin8 = 0,
    /// 16 Pin device
    Pin16 = 1,
    /// 20 Pin device
    Pin20 = 2,
    /// 24 Pin device
    Pin24 = 3,
    /// 32 Pin device
    Pin32 = 4,
    /// 44 Pin device
    Pin44 = 5,
    /// 48 Pin device
    Pin48 = 6,
    /// 64 Pin device
    Pin64 = 7,
    /// 80 Pin device
    Pin80 = 8,
    /// 100 Pin device
    Pin100 = 10,
    /// Effectively Invalid
    Reserved,
}

impl Id {
    /// Return the Kinetis Family ID (4 bits)
    pub fn family(&self) -> u8 {
        unsafe { &(*SIM::ptr()) }.srsid.read().famid().bits()
    }

    /// Return the Kinetis sub-family ID (4 bits)
    pub fn subfamily(&self) -> u8 {
        unsafe { &(*SIM::ptr()) }.srsid.read().subfamid().bits()
    }

    /// Return the device revision number (4 bits)
    pub fn revision(&self) -> u8 {
        unsafe { &(*SIM::ptr()) }.srsid.read().rev_id().bits()
    }

    /// Device pin id
    pub fn pinout(&self) -> DevicePinOuts {
        let sim = unsafe { &(*SIM::ptr()) };

        match sim.srsid.read().pinid().bits() {
            0 => DevicePinOuts::Pin8,
            1 => DevicePinOuts::Pin16,
            2 => DevicePinOuts::Pin20,
            3 => DevicePinOuts::Pin24,
            4 => DevicePinOuts::Pin32,
            5 => DevicePinOuts::Pin44,
            6 => DevicePinOuts::Pin48,
            7 => DevicePinOuts::Pin64,
            8 => DevicePinOuts::Pin80,
            10 => DevicePinOuts::Pin100,
            _ => DevicePinOuts::Reserved,
        }
    }
}

impl Status {
    /// A reset was caused by a module failing to acknowledge entering Stop
    /// mode
    pub fn stop_error_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().sackerr().bit()
    }

    /// A reset was caused by debugger request
    ///
    /// The request is made in the MDM-AP register, this is called MDMAP in
    /// SIM_SRSID (section 12.2.1 of KEA64RM).
    pub fn debugger_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().mdmap().bit()
    }

    /// A reset was caused by software request
    pub fn software_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().sw().bit()
    }

    /// A reset was caused by a core lockup event
    pub fn lockup_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().lockup().bit()
    }

    /// A reset was caused by power-on
    ///
    /// For normal startup, this is set, if there was some other reason for
    /// reseting, this is cleared.
    pub fn power_on_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().por().bit()
    }

    /// A reset was caused by the reset Pin
    pub fn pin_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().pin().bit()
    }

    /// A reset was caused by the watchdog.
    pub fn watchdog_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().wdog().bit()
    }

    /// A reset was caused by the Internal Clock Source Peripheral
    ///
    /// This is called LOC (Loss Of Clock?) rest in KEA64RM (sect 12.2.1)
    pub fn ics_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().loc().bit()
    }

    /// A reset was caused by low voltage (brown out)
    ///
    /// Brown out detection is controlled by the Power Mangement Controller
    /// (PMC) Peripheral
    pub fn lv_reset(&self) -> bool {
        unsafe { &(*SIM::ptr()) }.srsid.read().lvd().bit()
    }
}

/// Universally Unique IDentifier
pub struct UUID;

impl UUID {
    /// Returns the UUID as a u64
    pub fn uuid(&self) -> u64 {
        ((self.uuid_h() as u64) << 32) | (self.uuid_l() as u64)
    }

    /// Returns lower 32bits of UUID as u32
    pub fn uuid_l(&self) -> u32 {
        unsafe { &(*SIM::ptr()) }.uuidh.read().bits()
    }

    /// Returns upper 32bits of UUID as u32
    pub fn uuid_h(&self) -> u32 {
        unsafe { &(*SIM::ptr()) }.uuidl.read().bits()
    }
}
