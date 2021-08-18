# KEA-HAL

[![docs.rs](https://docs.rs/kea-hal/badge.svg)](https://docs.rs/kea-hal)
[![crates.io](https://img.shields.io/crates/v/kea-hal.svg)](https://crates.io/crates/kea-hal)
[![crates.io](https://img.shields.io/crates/d/kea-hal.svg)](https://crates.io/crates/kea-hal)

A Hardware Abstraction Layer (HAL) implementing
[embedded-hal](https://github.com/rust-embedded/embedded-hal) traits for
the NXP KEA64 (SKEAZN642) family of MCUs. This HAL depends upon the
[SKEAZN642 Peripheral Access Crate](https://github.com/wcpannell/SKEAZN642).
The intent is to expand this HAL to cover all MCUs in the KEA family

## Status

This project is under development. If it doesn't work, then it's probably not
fully implemented. Pull requests welcome. This is a side project, expect delays
in response.

## Compatibility

This crate was developed against a S9KEAZN32AMLH on a FRDM-KEAZ32Q64 dev board.
It should be compatible with all parts in the KEA family. Differences within
the family should be reconciled with feature flags (e.g. No onboard EEPROM).

Outside of the KEA family, NXP née Freescale, keeps a fairly consistent
interface to their peripherals. At the briefest glance, the S32K11x and the
KE06 look like they would be well supported by this crate.

## KEA64 Module Functional Categories

This is how NXP grouped the peripheral modules in this sub-family of devices.
The crate won't necessarily follow this layout, as it makes more sense to
be as similar and idiomatic as is reasonable to other HAL crates. This is
left in NXP's order largely to understand their resoning. This hierarchy
will be deleted or modified to match implementation once this HAL is in a
production-ready state. A checkmark will placed next to modules or peripherals
once they are (mostly) implemented.

- [x] Core - The ARMv6 Cortex-M core (see [cortex-m](https://github.com/rust-embedded/cortex-m))
    - NVIC - Nested Vectored Interrupt Controller
    - AWIC - Asynchronous Wakeup Interrupt Controller
    - IOPORT - Single Cycle I/O. Used by Fast GPIO (FGPIO) module
    - SWD - Single Wire Debug
- [x] System
    - SIM - system integration module
    - PMC - Power management and mode controller
    - MCM - Misc. control module
    - BME - Bit manipulation engine. Atomic Read/Modify/Write operations
    - AIPS - Peripheral bridge. Interfaces ARM AHB with Peripherals
    - WDOG - Watchdog
- [ ] Memory - Flash, EEPROM, SRAM. FTMRH peripheral used for interaction
- [x] Clocks
    - OSC
        + External Crystal Oscillator / Resonator
        + External Clock
    - ICS - Interal Clock Reference. 31.25 - 39.0625kHz Oscillator
    - LPO - 1kHZ Low Power Oscillator
    - Frequency-Locked Loop
- [ ] Security
    - [x] WDOG - Watchdog with independent clock source
    - [ ] CRC module (error detection)
- [ ] Analog
    - [x] ADC - 12 bit, 16 channels
    - [ ] 2x ACMP - Analog comparators
    - [ ] DAC - 6-bit (64-tap) resistor ladder network.
- [ ] Timers
    - FTM
        + One 6-channel FlexTimer, full featured
        + Two 2-channel FlexTimer, basic TPM function
    - 2x PIT - Periodic Interrupt Timer
    - RTC - real time clock
    - SysTick - System Tick Timer
- [ ] Communications
    - [x] 2x SPI - 8 bit serial peripheral interfaces
    - [ ] I2C - Inter-integrated circuit
    - [ ] 3x UART (up to, some devices may have less)
- [ ] HMI
    - [x] GPIO
    - [ ] 2x KBI - Key board interrupt
    - [ ] IRQ - Interrupts
