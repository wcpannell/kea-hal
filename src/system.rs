//! # System Meta-Module
//!
//! The system meta-module does not actually exist as it is a collection of
//! related peripheral modules.
//!
//! ## SIM - System Integration Module
//!
//! ## PMC - Power Management Controller
//!
//! The PMC controls the Low Voltage Detect and the ADC bandgap voltage
//! reference functions.
//!
//! ## MCM - Miscellaneous Control Module
//!
//! This peripheral contains several miscellaneous features grouped together.
//! At this time, none of the MCM registers are included in the HAL. This
//! documentation is for future implementation, but it may be helpful when
//! using the PAC in the mean-time.
//!
//! ### PLASC - Crossbar switch slave configuration
//!
//! This register allows the user to check which ports on the Crossbar switch
//! (AXBS) are connected to bus slaves.
//!
//! ### PLAMC - Crossbar Switch Master Configuration
//!
//! This register allows the user to check which ports on the Crossbar switch
//! (AXBS) are connected to bus masters.
//!
//! ### PLACR - Platform Control Register
//!
//! * ESFC - If set, allows flash to read code from the same block on which
//! flash operations are being perfomed. Cannot be within the same sector.
//! * DFCS - If set, Disables flash specualtion buffer for data and
//! instructions
//! * EFDS - If set, Enables flash data speculation
//! * DFCC - If set, Disables flash controller caching for data and
//! instructions
//! * DFCIC - If Set, Disables Flash instruct caching
//! * DFCDA - If set, disables flash data caching. Set by default
//! * CFCC - Clear Flash Controller Cache. Write 1 to clear.
//!
//! ## WDOG - Watchdog Timer
//!
//! This peripheral runs from an independent timer, and resets the MCU when
//! that timer overflows. Clearing the count value of this timer ensures that
//! software does not leave the cpu stuck in an infinite loop or execute from
//! unknown data.
//!
//! ## BME - Bit Manipulation Engine
//!
//! Provides Hardware-based atomic read-modify-write memory operations on
//! peripherals' memory mapped registers? Referred to as decorated storage.
//!
//! "The resulting architectural capability defined by this core platform
//! function is targeted at the manipulation of n-bit fields in peripheral
//! registers and is consistent with I/O hardware addressing in the Embedded C
//! Standard. For most BME commands, a single core read or write bus cycle is
//! converted into an atomic read-modify-write [...] bus sequency" - KEA64RM,
//! 17.1, pg 199.
//!
//! This guy is pretty interesting/odd, pull requests desired, else I'll come
//! back to it later.

pub mod pmc;
pub mod sim;
pub mod watchdog;
