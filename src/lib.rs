#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

pub extern crate SKEAZN642 as pac;
pub extern crate cortex_m;
#[cfg(feature = "rt-selected")]
pub extern crate cortex_m_rt;
pub extern crate embedded_hal as hal;
pub extern crate embedded_hal_alpha as hal_alpha;
pub extern crate embedded_time;
pub extern crate nb;
pub extern crate void;

pub use pac::CorePeripherals;

pub mod adc;
pub mod clocks;
pub mod gpio;
pub mod port;
pub mod prelude;
pub mod spi;
pub mod system;

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

/// This trait implements split method onto PAC peripheral structs.
pub trait HALExt {
    /// The HAL interface struct
    type T;

    /// Consume the PAC struct, split it into reasonable parts, and return them
    /// in an interface struct.
    ///
    /// Each HAL module implements user friendly interface methods onto the
    /// the returned struct(s). See the documentation for the HAL module of
    /// interest for more details about the interface.
    fn split(self) -> Self::T;
}
