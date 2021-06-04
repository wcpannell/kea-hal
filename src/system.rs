//! # System Meta-Module
//!
//! The system meta-module does not actually exist as it is a collection of
//! related peripheral modules.
//!
//! ## SIM - System Integration Module
//!
//! # PMC - Power Management Controller
//!
//! The PMC controls the Low Voltage Detect and the ADC bandgap voltage
//! reference functions.
//!

pub mod pmc;
pub mod sim;

pub use pmc::PMCExt;
pub use sim::SIMExt;
