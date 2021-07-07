//! The ADC Interface
//!
//! The ADC is disabled at startup and must be enabled (by calling
//! [Adc<Disabled>::enable]) before any of its registers can be accessed
//! (read or write). Attempts to access these registers will trigger a hardware
//! generated HardFault, which by default resets the microcontroller.
//!
//! The ADC can be polled for conversion completion with [Adc::is_done].
//! Completion will trigger an ADC Interrupt if enabled. See
//! [Adc::into_interrupt]
//!
//! ## Input Modes
//!
//! The Adc peripheral can operate in either single input or FIFO modes. Single
//! input mode is the mode most commonly thought of when using an ADC. A
//! multiplexer (via Adc::set_channel) is used to connect a single channel to
//! the ADC, and when the conversion is complete the hardware makes the results
//! available in the results register. The software must call
//! [Adc::set_channel] again to either select a new channel or to restart the
//! conversion on the same channel.
//!
//! The FIFO mode sets up a hardware buffer of selectable depth (2-8 channels).
//! Once the buffer is filled the Adc peripheral shoves the buffer contents
//! into the multiplexer channel by channel. Likewise, as each conversion is
//! completed the results are buffered into the result register in the same
//! order as the channel select buffer.
//!
//! Note: FIFO mode is not yet implemented in this HAL
//!
//! ## Conversion Modes
//!
//! The Adc peripheral offers 2 conversion modes, OneShot and Continuous. In
//! OneShot mode, the conversion is started when the channel is selected (or
//! when the channel select buffer is filled in FIFO mode). After completion no
//! new conversion is started until the channel is set again, even if the same
//! channel is used.
//!
//! In Continuous mode a new conversion is started immediately
//! after the previous one is completed. Changing the channel interrupts the
//! conversion and immediately begins conversion on the new channel (unless the
//! new channel is [DummyDisable], then the conversion is allowed to complete,
//! but no new conversion is started). In FIFO mode the input FIFO is reloaded
//! after completion, in other words the same N values are converted on a loop.
//!
//! Note: Continuous mode is not yet implemented in this HAL
//!
//! ## Comparison Mode
//!
//! Note: Comparison mode is not yet implemented in this HAL
//!
//! Comparison mode is a hardware feature of the Adc Peripheral. If set, the
//! conversion result is compared to the comparison value. If the result
//! is greater than or less than (depending on configuration) the comparison
//! value the result is moved into the result register. Otherwise, the result
//! is discarded \[Note: Unsure if the conversion is restarted in OneShot
//! mode\].
//!
//! A common use case for comparison mode is to enter a low power state with
//! the Adc configured to use the asynchronous clock source and to generate an
//! interrupt on completion. When the input channel crosses the comparison
//! threshold the interrupt is triggered, waking the MCU.
//!
//! ## Clocking
//!
//! The ADC requires a clock signal (ADCK), which is generated from the bus
//! clock, the bus clock divided by 2, the output of the OSC peripheral
//! (OSC_OUT), or an internal asynchronous clock, which, when selected,
//! operates in wait and stop modes. With any of these clock sources a
//! multi-value divider is provided to further divide the incoming clock by 1
//! (i.e. 1:1), 2, 4, or 8.
//!
//! The clock frequency must fall within 400kHz to 8MHz (4MHz in low power
//! mode), This is the same for all KEA MCUs. Ideally, the HAL will only
//! present valid options, but that is not yet implemented (pending clocks
//! improvements to output frequencies). For now you are trusted to input the
//! correct frequency.
//!
//! *Note:* When using the FIFO mode with FIFO scan mode disabled, the bus
//! clock must be faster than half the ADC clock (ADCK). Bus clock >= ADCK / 2.
//!
//! ## Pin Control
//!
//! This functionality is implemented in the GPIO module. See [Analog]
//! for details.
//!
//! ## Conversion Width
//!
//! The ADC can be run in 8, 10, or 12 bit modes. These modes are enumerated in
//! [AdcResolution].
//!
//! ## Hardware Trigger
//!
//! The ADC conversions can be started by a hardware trigger. This is not
//! implemented in all KEA chips, so implementation here will be Delayed. Use
//! the PAC. Enable is ADC_SC2\[ADTRG\] = 1, and trigger is the ADHWT source.
//!
//! ## Usage
//!
//! ### AdcConfig struct
//!
//! [AdcConfig] offers public fields to allow for creation in-place. The
//! [AdcConfig::calculate_divisor] method allows the user to specify the
//! desired Adc Clock frequency (given the clock source frequency). The clock
//! divider which gets the closest to that frequency is chosen.
//!
//! The AdcConfig structure also implements the [Default] trait.
//!
//! ```rust
//! let config: AdcConfig = Default::default();
//!
//! config.calculate_divisor(20_u32.MHz(), 2_u32.MHz());
//! assert!(matches!(config.clock_divisor, ClockDivisor::_8));
//! ```

use crate::hal::adc::{Channel, OneShot};
use crate::{pac::ADC, HALExt};
use core::{convert::Infallible, marker::PhantomData};
use embedded_time::rate::*;

/// Error Enumeration for this module
#[derive(Debug)]
pub enum Error {
    /// The Channel has already been moved
    Moved,
}

/// Analog type state for a GPIO pin.
///
/// This mode "gives" the pin to the ADC hardware peripheral.
/// The ADC Peripheral can take the GPIO pins in any state. The Peripheral will
/// reconfigure the pin to turn off any output drivers, disable input buffers
/// (reading the pin after configuring as analog will return a zero), and
/// disable the pullup. Electrically, an Analog pin that is not currently under
/// conversion is effectively HighImpedence.
///
/// Once a pin is released from the ADC, it will return to its previous state.
/// The previous state includes output enabled, input enabled, pullup enabled,
/// and level (for outputs). Note to accomplish this the pin implements the
/// outof_analog method, which is semantically different from the other type
/// states.
///
/// For example, [crate::gpio::gpioa::PTA0] is configured to be a Output that is set high is
/// converted into the analog mode with the [crate::gpio::gpioa::PTA0::into_analog] method.
/// Once measurements from that pin are completed it will be returned to an
/// Output that is set high by calling the [Analog::outof_analog] method.
///
/// ```rust
/// let pta0 = gpioa.pta0.into_push_pull_output();
/// pta0.set_high();
/// let mut pta0 = pta0.into_analog(); // pta0 is hi-Z
/// let value = adc.read(&mut pta0).unwrap_or(0);
/// let pta0 = pta0.outof_analog();  // pta0 is push-pull output, set high.
/// ```
///
/// Note: This is a hardware feature that requires effectively no clock cycles
/// to complete. "Manually" reconfiguring the pins to HighImpedence before
/// calling into_analog() is discouraged, but it would not hurt anything.
pub struct Analog<Pin> {
    pin: Pin,
}

/// Interface for ADC Peripheral.
///
/// Returned by calling [HALExt::split] on the pac [ADC] structure. Holds state
/// of peripheral.
pub struct Adc<State> {
    peripheral: ADC,
    _state: PhantomData<State>,
    /// Contains the On-Chip ADC Channels, like the MCU's temperature sensor.
    pub onchip_channels: OnChipChannels,
}

impl HALExt for ADC {
    type T = Adc<Disabled>;
    fn split(self) -> Adc<Disabled> {
        Adc {
            peripheral: self,
            _state: PhantomData,
            onchip_channels: OnChipChannels {
                vss: Some(Analog {
                    pin: Vss::<Input> { _mode: PhantomData },
                }),
                temp_sense: Some(Analog {
                    pin: TempSense::<Input> { _mode: PhantomData },
                }),
                bandgap: Some(Analog {
                    pin: Bandgap::<Input> { _mode: PhantomData },
                }),
                vref_h: Some(Analog {
                    pin: VrefH::<Input> { _mode: PhantomData },
                }),
                vref_l: Some(Analog {
                    pin: VrefL::<Input> { _mode: PhantomData },
                }),
            },
        }
    }
}

/// Configuration struct for Adc peripheral.
pub struct AdcConfig {
    /// Determines the clock source for the ADC peripheral
    ///
    /// Default is [AdcClocks::Bus]
    pub clock_source: AdcClocks,
    /// Divides the clock source to get the ADC clock into it's usable range of
    /// 400kHz - 8MHz (4MHz in low power mode).
    ///
    /// Default is [ClockDivisor::_1] (no divison)
    pub clock_divisor: ClockDivisor,
    /// Set the resolution of ADC conversion
    ///
    /// Default is [AdcResolution::_8bit]
    pub resolution: AdcResolution,

    /// Set ADC sample time.
    ///
    /// Default is [AdcSampleTime::Short]
    pub sample_time: AdcSampleTime,

    /// Set low power mode
    ///
    /// Default is false.
    pub low_power: bool,
}

impl AdcConfig {
    /// Calculate the ADC clock divisor
    ///
    /// Uses the current clock source and clock frequency to determine
    /// the best divisor to use in order to have minimal error between
    /// the ADC clock rate and the desired ADC clock rate.
    ///
    /// Note: This relies on trustworthy values for source_freq and valid
    /// values for req_adc_freq. In the future this should know or
    /// determine what the current clock frequency is instead of relying
    /// on the user to provide it.
    pub fn calculate_divisor(&mut self, source_freq: Hertz, req_adc_freq: Hertz) {
        let denom: u8 = (source_freq.integer() / req_adc_freq.integer()) as u8;

        let mut output: u8 = 1;
        let mut err: i8 = (denom - output) as i8;
        let mut err_old: i8 = err;
        let max_divisor = match self.clock_source {
            AdcClocks::Bus => 16,
            _ => 8,
        };
        while output < max_divisor {
            err = (denom - (output << 1)) as i8;
            if err.is_negative() {
                err = err.abs();
            }
            if err <= err_old {
                output <<= 1;
                err_old = err;
            } else {
                break;
            }
        }

        // I am of the mind that this assert is okay, at least until the input
        // clock can be known at compile time.
        let ad_clock = source_freq.integer() / output as u32;
        assert!(400_000 <= ad_clock);
        assert!(
            ad_clock
                <= match self.low_power {
                    false => 8_000_000,
                    true => 4_000_000,
                }
        );

        self.clock_divisor = match output {
            1 => ClockDivisor::_1,
            2 => ClockDivisor::_2,
            4 => ClockDivisor::_4,
            8 => ClockDivisor::_8,
            _ => ClockDivisor::_16,
        }
    }

    /// Set the divisor directly. panics if divisor isn't supported by the
    /// clock source.
    ///
    /// TODO: Refactor to remove assert. Add Clock Source as a type state
    pub fn set_divisor(&mut self, divisor: ClockDivisor) {
        // divisor can't be 16 unless using the Bus clock
        assert!(
            !(!matches!(self.clock_source, AdcClocks::Bus) && matches!(divisor, ClockDivisor::_16))
        );
        self.clock_divisor = divisor;
    }

    /// Sets the clock source, panics if divisor isn't supported
    ///
    /// TODO: Refactor to remove assert. Add Clock Source as a type state
    pub fn set_clock_source(&mut self, clock: AdcClocks) {
        // Panic if setting the clock to anything other than Bus if the divisor
        // is set to 16
        assert!(
            !matches!(clock, AdcClocks::Bus) && matches!(self.clock_divisor, ClockDivisor::_16)
        );
        self.clock_source = clock;
    }
}

impl Default for AdcConfig {
    fn default() -> AdcConfig {
        AdcConfig {
            clock_source: AdcClocks::Bus,
            clock_divisor: ClockDivisor::_1,
            resolution: AdcResolution::_12bit,
            sample_time: AdcSampleTime::Short,
            low_power: false,
        }
    }
}

/// Clock types available to the Adc peripheral
///
/// Dividers will be chosen appropriately to suit requested clock rate.
pub enum AdcClocks {
    /// Use the incoming Bus Clock
    Bus,
    /// jkl
    External,
    /// Available in Wait AND Stop Mode
    Async,
}

/// This enum represents the availabe ADC resolutions
///
/// Regardless of resolution chosen, results are always right justified
#[repr(u8)]
pub enum AdcResolution {
    /// 8 bit AD conversion mode
    _8bit = 0,
    /// 10 bit AD conversion mode
    _10bit = 1,
    /// 12 bit AD conversion mode
    _12bit = 2,
}

/// Adc sample time
pub enum AdcSampleTime {
    /// Sample for 3.5 ADC clock (ADCK) cycles.
    Short = 0,

    /// Sample for 23.5 ADC clock (ADCK) cycles.
    ///
    /// Required for high impedence (>2k @ADCK > 4MHz, >5k @ ADCK < 4MHz)
    /// inputs.
    Long = 1,
}

/// Adc Clock Divisors
///
/// Note 1/16 divisor is only usable for the Bus clock
pub enum ClockDivisor {
    /// Source / 1, No divison
    _1 = 0,
    /// Source / 2
    _2 = 1,
    /// Source / 4
    _4 = 2,
    /// Source / 8
    _8 = 3,
    /// Source / 16
    _16 = 4,
}

/// Enabled state
pub struct Enabled;

/// Disabled state
pub struct Disabled;

impl Adc<Enabled> {
    /// Poll to determine if ADC conversion is complete.
    ///
    /// Note: This flag is cleared when the sampling mode is changed,
    /// interrupts are enabled, [Adc::set_channel] is called, and when [Adc::result] is
    /// called (including [Adc::try_result])
    pub fn is_done(&self) -> bool {
        self.peripheral.sc1.read().coco().bit()
    }

    /// Poll to determine if ADC conversion is underway
    pub fn is_converting(&self) -> bool {
        self.peripheral.sc2.read().adact().bit()
    }

    /// Grab the last ADC conversion result.
    pub fn result(&self) -> u16 {
        self.peripheral.r.read().adr().bits()
    }

    /// Poll for conversion completion, if done return the result.
    pub fn try_result(&self) -> Option<u16> {
        if self.is_done() {
            Some(self.result())
        } else {
            None
        }
    }

    /// Set ADC target channel.
    ///
    /// In Single conversion mode (OneShot), setting the channel begins the conversion. In FIFO mode
    /// the channel is added to the FIFO buffer.
    ///
    /// Note: If the channel is changed while a conversion is in progress the
    /// current conversion will be cancelled. If in FIFO mode, conversion will
    /// resume once the FIFO channels are refilled.
    pub fn set_channel<T: Channel<Adc<Enabled>, ID = u8>>(&self, _pin: &T) {
        self.peripheral
            .sc1
            .modify(|_, w| unsafe { w.adch().bits(T::channel()) });
    }

    /// Set the ADC's configuration
    pub fn configure(self, config: AdcConfig) -> Adc<Enabled> {
        self.peripheral.sc3.modify(|_, w| {
            use pac::adc::sc3::{ADICLK_A, ADIV_A, ADLSMP_A, MODE_A};
            w.adiclk()
                .variant(match config.clock_source {
                    AdcClocks::Bus =>
                    // If divisor is 16, use the Bus / 2 clock source, else use
                    // the 1:1 Bus clock source
                    {
                        match config.clock_divisor {
                            ClockDivisor::_16 => ADICLK_A::_01,
                            _ => ADICLK_A::_00,
                        }
                    }
                    AdcClocks::External => ADICLK_A::_10,
                    AdcClocks::Async => ADICLK_A::_11,
                })
                .mode()
                .variant(match config.resolution {
                    AdcResolution::_8bit => MODE_A::_00,
                    AdcResolution::_10bit => MODE_A::_01,
                    AdcResolution::_12bit => MODE_A::_10,
                })
                .adlsmp()
                .variant(match config.sample_time {
                    AdcSampleTime::Short => ADLSMP_A::_0,
                    AdcSampleTime::Long => ADLSMP_A::_1,
                })
                .adiv()
                .variant(match config.clock_divisor {
                    ClockDivisor::_1 => ADIV_A::_00,
                    ClockDivisor::_2 => ADIV_A::_01,
                    ClockDivisor::_4 => ADIV_A::_10,
                    _ => ADIV_A::_11,
                })
                .adlpc()
                .bit(config.low_power)
        });

        // It looks like SCGC has to be set before touching the peripheral
        // at all, else hardfault. Go back later to confirm that if using external clock
        // scgc can be cleared.
        // w.adc().variant(match config.clock_source {
        //     AdcClocks::Bus => ADC_A::_1,
        //     _ => ADC_A::_0,
        // })

        Adc {
            peripheral: self.peripheral,
            _state: PhantomData,
            onchip_channels: self.onchip_channels,
        }
    }
}

impl Adc<Disabled> {
    /// Connects the bus clock to the adc via the SIM peripheral, allowing
    /// read and write access to ADC registers.
    ///
    /// Any attempt to access ADC registers while disabled results in a
    /// HardFault, generated by hardware.
    ///
    /// This also enables the bandgap voltage reference.
    pub fn enable(self) -> Adc<Enabled> {
        cortex_m::interrupt::free(|_| {
            unsafe { &(*pac::SIM::ptr()) }.scgc.modify(|_, w| {
                use pac::sim::scgc::ADC_A;
                w.adc().variant(ADC_A::_1)
            });

            // Don't start a conversion (set channel to DummyDisable)
            self.peripheral.sc1.modify(|_, w| w.adch()._11111());

            // Bandgap. Grab directly, Currently the bandgap isn't implemented
            // in [system::PMC]. We will eventually have to pass in the pmc
            // peripheral handle as a variable.
            unsafe { &(*pac::PMC::ptr()) }
                .spmsc1
                .modify(|_, w| w.bgbe()._1());
        });

        Adc {
            peripheral: self.peripheral,
            _state: PhantomData,
            onchip_channels: self.onchip_channels,
        }
    }

    /// Set the ADC's configuration
    ///
    /// This is a sugar method for calling [Adc<Disabled>::enable] followed by
    /// [Adc<Enabled>::configure]
    pub fn configure(self, config: AdcConfig) -> Adc<Enabled> {
        self.enable().configure(config)
    }
}

impl<Mode> Adc<Mode> {
    /// Not Implemented
    pub fn into_interrupt(self) -> Adc<Mode> {
        unimplemented!("Interrupt is not yet implemented");
        // Adc::<Mode> {
        //     peripheral: self.peripheral,
        //     _state: PhantomData,
        //     onchip_channels: self.onchip_channels,
        // }
    }

    /// Not Implemented
    pub fn into_fifo(self, _depth: u8) -> Adc<Mode> {
        // self.peripheral
        //     .sc4
        //     .modify(|_r, w| w.afdep().bits(depth & 0x7));
        // Adc::<Mode> {
        //     peripheral: self.peripheral,
        //     _state: PhantomData,
        //     onchip_channels: self.onchip_channels,
        // }
        unimplemented!("FIFO is not yet implemented");
    }

    /// Not Implemented
    pub fn into_continuous(self) -> Adc<Mode> {
        unimplemented!("Continuous Conversion mode not yet implemented");
    }
}

impl OnChipChannels {
    /// Request an instance of an on-chip [Vss] channel.
    pub fn vss(&mut self) -> Result<Analog<Vss<Input>>, Error> {
        self.vss.take().ok_or(Error::Moved)
    }

    /// Return the instance of [Vss]
    pub fn return_vss(&mut self, inst: Analog<Vss<Input>>) {
        self.vss.replace(inst);
    }

    /// Try to grab an instance of the onchip [TempSense] channel.
    pub fn tempsense(&mut self) -> Result<Analog<TempSense<Input>>, Error> {
        self.temp_sense.take().ok_or(Error::Moved)
    }

    /// Return the instance of [TempSense]
    pub fn return_tempsense(&mut self, inst: Analog<TempSense<Input>>) {
        self.temp_sense.replace(inst);
    }

    /// Try to grab an instance of the onchip [Bandgap] channel.
    ///
    /// The bandgap reference is a fixed 1.16V (nom, Factory trimmed to +/-
    /// 0.02V at Vdd=5.0 at 125C) signal that is available to the ADC Module.
    /// It can be used as a voltage reference for the ACMP and as an [Analog]
    /// channel that can be used to (roughly) check the VDD voltage
    pub fn bandgap(&mut self) -> Result<Analog<Bandgap<Input>>, Error> {
        self.bandgap.take().ok_or(Error::Moved)
    }

    /// Return the instance of [Bandgap]
    pub fn return_bandgap(&mut self, inst: Analog<Bandgap<Input>>) {
        self.bandgap.replace(inst);
    }

    /// Try to grab an instance of the onchip Voltage Reference High ([VrefH]) channel.
    pub fn vref_h(&mut self) -> Result<Analog<VrefH<Input>>, Error> {
        self.vref_h.take().ok_or(Error::Moved)
    }

    /// Return the instance of [VrefH]
    pub fn return_vref_h(&mut self, inst: Analog<VrefH<Input>>) {
        self.vref_h.replace(inst);
    }

    /// Try to grab an instance of the onchip Voltage Reference Low ([VrefL]) channel.
    pub fn vref_l(&mut self) -> Result<Analog<VrefL<Input>>, Error> {
        self.vref_l.take().ok_or(Error::Moved)
    }

    /// Return the instance of [VrefL]
    pub fn return_vref_l(&mut self, inst: Analog<VrefL<Input>>) {
        self.vref_l.replace(inst);
    }

    /// Grab a [DummyDisable] instance. Multiple Instances possible.
    pub fn dummy_disable(&self) -> Analog<DummyDisable<Input>> {
        Analog {
            pin: DummyDisable::<Input> { _mode: PhantomData },
        }
    }
}

/// Holds On-Chip ADC Channel inputs and provides an interface to grab and return them.
// These have to have the Input dummy type to allow them to have the Channel
// trait.
pub struct OnChipChannels {
    vss: Option<Analog<Vss<Input>>>,
    temp_sense: Option<Analog<TempSense<Input>>>,
    bandgap: Option<Analog<Bandgap<Input>>>,
    vref_h: Option<Analog<VrefH<Input>>>,
    vref_l: Option<Analog<VrefL<Input>>>,
}

/// Dummy type state for on-chip ADC input channels
pub struct Input;

/// Adc Input Channel, measures ground (should be 0?)
pub struct Vss<Input> {
    _mode: PhantomData<Input>,
}

/// Adc Input Channel, measures internal temperature sensor
pub struct TempSense<Input> {
    _mode: PhantomData<Input>,
}

/// Adc Input Channel, Bandgap internal voltage reference
pub struct Bandgap<Input> {
    _mode: PhantomData<Input>,
}

/// Adc Input Channel, Voltage Reference, High
pub struct VrefH<Input> {
    _mode: PhantomData<Input>,
}

/// Adc Input Channel, Voltage Reference, Low
pub struct VrefL<Input> {
    _mode: PhantomData<Input>,
}

/// Dummy Channel that temporarily disables the Adc Module.
pub struct DummyDisable<Input> {
    _mode: PhantomData<Input>,
}
macro_rules! adc_input_channels {
    ( $($Chan:expr => $Pin:ident),+ $(,)*) => {
        $(
            impl<OldMode> Channel<Adc<Enabled>> for Analog<$Pin<OldMode>> {
                type ID = u8;
                fn channel() -> u8 { $Chan }
            }
        )+
    };
}

use crate::gpio::{gpioa::*, gpiob::*};
adc_input_channels! (
    0_u8 => PTA0,
    1_u8 => PTA1,
    2_u8 => PTA6,
    3_u8 => PTA7,
    4_u8 => PTB0,
    5_u8 => PTB1,
    6_u8 => PTB2,
    7_u8 => PTB3,
    8_u8 => PTC0,
    9_u8 => PTC1,
    10_u8 => PTC2,
    11_u8 => PTC3,
    12_u8 => PTF4,
    13_u8 => PTF5,
    14_u8 => PTF6,
    15_u8 => PTF7,
    16_u8 => Vss,
    22_u8 => TempSense,
    23_u8 => Bandgap,
    24_u8 => VrefH,
    25_u8 => VrefL,
    0x1F_u8 => DummyDisable,
);

macro_rules! impl_analog_pin {
    ( $($Chan:expr => $Pin:ident),+ $(,)*) => {
        $(
            impl<OldMode> $Pin<OldMode> {
                /// Convert Pin into the [Analog] state for use by the ADC.
                ///
                /// This implementation provides the GPIO interface a method to
                /// give an eligible pin to the ADC peripheral for conversion
                /// into an Analog pin. This method is only implemented in
                /// eligible pins. The ADC peripheral disables the GPIO and
                /// PORT control over the pin and connects it to the ADC mux
                /// (controlled by [Adc::set_channel].
                ///
                /// Note: The [Analog::outof_analog] method must be used to
                /// return the pin to a normal Input/Output typestate. The pin
                /// will be returned in the same typestate as it was received.
                pub fn into_analog(self) -> Analog<$Pin<OldMode>> {
                    unsafe {
                        (*ADC::ptr())
                            .apctl1
                            .modify(|r, w| w.adpc().bits(r.adpc().bits() | (1 << $Chan)));
                    }
                    Analog { pin: self }
                }
            }

            impl<OldMode> Analog<$Pin<OldMode>> {
                /// Return Analog state Pin to normal GPIO-state interface.
                ///
                /// The Pin will be in the same state that it was when it
                /// entered the Analog type state.
                pub fn outof_analog(self) -> $Pin<OldMode> {
                    let adc = unsafe { &(*ADC::ptr()) };
                    adc.apctl1
                        .modify(|r, w| unsafe { w.adpc().bits(r.adpc().bits() & !(1 << $Chan)) });
                    self.pin
                }
            }
        )+
    };
}

impl_analog_pin!(
    0_u8 => PTA0,
    1_u8 => PTA1,
    2_u8 => PTA6,
    3_u8 => PTA7,
    4_u8 => PTB0,
    5_u8 => PTB1,
    6_u8 => PTB2,
    7_u8 => PTB3,
    8_u8 => PTC0,
    9_u8 => PTC1,
    10_u8 => PTC2,
    11_u8 => PTC3,
    12_u8 => PTF4,
    13_u8 => PTF5,
    14_u8 => PTF6,
    15_u8 => PTF7,
);

impl<Pin> OneShot<Adc<Enabled>, u16, Pin> for Adc<Enabled>
where
    Pin: Channel<Adc<Enabled>, ID = u8>,
{
    type Error = Infallible;

    fn read(&mut self, pin: &mut Pin) -> nb::Result<u16, Self::Error> {
        self.set_channel(pin);
        while !self.is_done() {}
        let ret_val = Ok(self.result());
        let disable = self.onchip_channels.dummy_disable();
        self.set_channel(&disable);
        ret_val
    }
}
