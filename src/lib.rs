//! # KEA-HAL
//!
//! A Hardware Abstraction Layer (HAL) implementing
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits for
//! the NXP KEA64 (SKEAZN642) family of MCUs. This HAL depends upon the
//! [SKEAZN642 Peripheral Access Crate](https://github.com/wcpannell/SKEAZN642).
//! The intent is to expand this HAL to cover all MCUs in the KEA family
//!
//! ## KEA64 Module Functional Categories
//!
//! This is how NXP grouped the peripheral modules in this sub-family of devices.
//! The crate won't necessarily follow this layout, as it makes more sense to
//! be as similar and idiomatic as is reasonable to other HAL crates. This is
//! left in NXP's order largely to understand their resoning. This hierarchy
//! will be deleted or modified to match implementation once this HAL is in a
//! production-ready state.
//!
//! * Core - The ARMv6 Cortex-M core
//!     - NVIC - Nested Vectored Interrupt Controller
//!     - AWIC - Asynchronous Wakeup Interrupt Controller
//!     - IOPORT - Single Cycle I/O. Used by Fast GPIO (FGPIO) module
//!     - SWD - Single Wire Debug
//! * System
//!     - SIM - system integration module
//!     - PMC - Power management and mode controller
//!     - MCM - Misc. control module
//!     - BME - Bit manipulation engine. Atomic Read/Modify/Write operations
//!     - AIPS - Peripheral bridge. Interfaces ARM AHB with Peripherals
//!     - WDOG - Watchdog
//! * Memory - Flash, EEPROM, SRAM. FTMRH peripheral used for interaction
//! * Clocks
//!     - OSC
//!         + External Crystal Oscillator / Resonator
//!         + External Clock
//!     - ICS - Interal Clock Reference. 31.25 - 39.0625kHz Oscillator
//!     - LPO - 1kHZ Low Power Oscillator
//!     - Frequency-Locked Loop
//! * Security
//!     - WDOG - Watchdog with independent clock source
//!     - CRC module (error detection)
//! * Analog
//!     - ADC - 12 bit, 16 channels
//!     - 2x ACMP - Analog comparators
//!     - DAC - 6-bit (64-tap) resistor ladder network.
//! * Timers
//!     - FTM
//!         + One 6-channel FlexTimer, full featured
//!         + Two 2-channel FlexTimer, basic TPM function
//!     - 2x PIT - Periodic Interrupt Timer
//!     - RTC - real time clock
//!     - SysTick - System Tick Timer
//! * Communications
//!     - 2x SPI - 8 bit serial peripheral interfaces
//!     - I2C - Inter-integrated circuit
//!     - 3x UART (up to, some devices may have less)
//! * HMI
//!     - GPIO
//!     - 2x KBI - Key board interrupt
//!     - IRQ - Interrupts

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
pub mod clocks;
pub mod gpio;
pub mod port;
pub mod prelude;
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
