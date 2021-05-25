//! # SKEAZN642-HAL
//!
//! A Hardware Abstraction Layer (HAL) implementing
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits for
//! the NXP KEA64 (SKEAZN642) family of MCUs. This HAL depends upon the
//! [SKEAZN642 Peripheral Access Crate](https://github.com/wcpannell/SKEAZN642).
//! The intent is to expand this HAL to cover all MCUs in the KEA family

#![no_std]
#![warn(missing_docs)]

pub extern crate SKEAZN642 as pac;
pub extern crate cortex_m;
#[cfg(feature = "rt-selected")]
pub extern crate cortex_m_rt;
pub extern crate embedded_hal as hal;
pub extern crate embedded_hal_alpha as hal_alpha;
pub extern crate nb;
pub extern crate void;

pub use pac::CorePeripherals;

//pub mod adc;
pub mod gpio;
pub mod port;
pub mod prelude;
//pub mod rcc;

/// State of Peripheral
pub mod init_state {
    /// Indicates peripheral is enabled
    pub struct Enabled<T = ()>(pub T);

    /// Indicates Disabled.
    pub struct Disabled;
}

mod private {
    pub trait Sealed {}
}
