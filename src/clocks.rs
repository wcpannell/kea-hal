//! # Clocks meta-module
//!
//! Note: The terms module and peripheral are used interchangably in this text.
//! NXP / Freescale uses module throughout its documentation to refer to what
//! are commonly known as peripherals.
//!
//! ## Clock Architecture
//!
//! The ICS module selects and/or muxes the clock sources to output the system
//! clock sources. The SIM module handles dividing and gating of those clock
//! sources. If an external oscillator/resonator is used, the OSC module passes
//! those signals to the ICS and SIM modules as needed.
//!
//! ## Sim - System Integration Module Peripheral
//!
//! The SIM System Clock Gating Control Register (SIM_SCGC) register can gate
//! on or off the clock to each module. Before enabling a module, the
//! corresponding SIM_SCGC bit must be set. Conversely, a module must be
//! disabled before turning off its clock. Bus access to an "unclocked"
//! peripheral will generate an error termination (a hard fault?)
//!
//! Note that the SIM is not part of the clocks meta-module, but it seemed
//! necessary to discuss it here due to the interplay between the SIM_SCGC and
//! the clocks' peripherals.
//!
//! ## Ics - Internal Clock Source Peripheral
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
//! ## Osc - External Oscillator Clock Source Peripheral
//!
//! The OSC peripheral provides an external clock source for the MCU. The clock
//! source can either be a clock signal or a crystal / resonator type source.
//! This source is then provided to the MCU for use as a reference clock (for
//! use in the Ics module), or as a bus clock (for use in the Sim peripheral).
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

mod ics;
mod osc;

pub use ics::ICSExt;
pub use osc::OSCExt;
