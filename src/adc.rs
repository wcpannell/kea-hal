//! The ADC Interface
//!
//! The ADC can be polled for conversion completion with [Adc::is_done]. Completion
//! will trigger an ADC Interrupt if enabled. See [Adc::into_interrupt]
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
//! The ADC Peripheral can take the GPIO pins in any state. The Peripheral will
//! reconfigure the pin to turn off any output drivers (i.e. HighImpedence
//! state), disable input buffers (reading the pin after configuring as analog
//! will return a zero), and disable the pullup.
//!
//! Once a pin is released from the ADC, it will return to its previous state.
//! The previous state includes output enabled, input enabled, pullup enabled,
//! and level (for outputs).
//!
//! ## Conversion Width - [AdcResolution]
//!
//! The ADC can be run in 8, 10, or 12 bit modes
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

/// State type fore ADC Peripheral
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
                vss: Some(Vss::<Input> { _mode: PhantomData }),
                temp_sense: Some(TempSense::<Input> { _mode: PhantomData }),
                bandgap: Some(Bandgap::<Input> { _mode: PhantomData }),
                vref_h: Some(VrefH::<Input> { _mode: PhantomData }),
                vref_l: Some(VrefL::<Input> { _mode: PhantomData }),
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
                output = output << 1;
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
    pub fn is_done(&self) -> bool {
        self.peripheral.sc1.read().coco().bit()
    }

    /// Grab the last ADC conversion result.
    pub fn result(&self) -> u16 {
        self.peripheral.r.read().adr().bits()
    }

    /// Poll for conversion completion, if done return the result.
    pub fn try_result(&self) -> Option<u16> {
        if self.is_done() {
            return Some(self.result());
        } else {
            return None;
        }
    }

    /// Set ADC target channel.
    ///
    /// In Single conversion mode (OneShot), setting the channel begins the conversion. In FIFO mode
    /// the channel is added to the FIFO buffer.
    ///
    /// Note: If in either single or continuous, single channel or FIFO modes,
    /// if the channel is changed while conversion is in progress the current
    /// conversion will be cancelled. If in FIFO mode, conversion will resume
    /// once the FIFO channels are refilled.
    pub fn set_channel<T: Channel<Adc<Enabled>, ID = u8>>(&self, _pin: &T) {
        self.peripheral
            .sc1
            .modify(|_, w| unsafe { w.adch().bits(T::channel()) });
    }
}

impl<Mode> Adc<Mode> {
    /// Set the ADC's configuration
    pub fn configure(self, config: AdcConfig) -> Adc<Enabled> {
        self.peripheral.sc3.modify(|_, w| {
            w.adiclk()
                .variant(match config.clock_source {
                    AdcClocks::Bus =>
                    // If divisor is 16, use the Bus / 2 clock source, else use
                    // the 1:1 Bus clock source
                    {
                        match config.clock_divisor {
                            ClockDivisor::_16 => pac::adc::sc3::ADICLK_A::_01,
                            _ => pac::adc::sc3::ADICLK_A::_00,
                        }
                    }
                    AdcClocks::External => pac::adc::sc3::ADICLK_A::_10,
                    AdcClocks::Async => pac::adc::sc3::ADICLK_A::_11,
                })
                .mode()
                .variant(match config.resolution {
                    AdcResolution::_8bit => pac::adc::sc3::MODE_A::_00,
                    AdcResolution::_10bit => pac::adc::sc3::MODE_A::_01,
                    AdcResolution::_12bit => pac::adc::sc3::MODE_A::_11,
                })
                .adlsmp()
                .variant(match config.sample_time {
                    AdcSampleTime::Short => pac::adc::sc3::ADLSMP_A::_0,
                    AdcSampleTime::Long => pac::adc::sc3::ADLSMP_A::_1,
                })
                .adiv()
                .variant(match config.clock_divisor {
                    ClockDivisor::_1 => pac::adc::sc3::ADIV_A::_00,
                    ClockDivisor::_2 => pac::adc::sc3::ADIV_A::_01,
                    ClockDivisor::_4 => pac::adc::sc3::ADIV_A::_10,
                    _ => pac::adc::sc3::ADIV_A::_11,
                })
                .adlpc()
                .bit(config.low_power)
        });
        Adc {
            peripheral: self.peripheral,
            _state: PhantomData,
            onchip_channels: OnChipChannels {
                vss: Some(Vss::<Input> { _mode: PhantomData }),
                temp_sense: Some(TempSense::<Input> { _mode: PhantomData }),
                bandgap: Some(Bandgap::<Input> { _mode: PhantomData }),
                vref_h: Some(VrefH::<Input> { _mode: PhantomData }),
                vref_l: Some(VrefL::<Input> { _mode: PhantomData }),
            },
        }
    }

    /// Not Implemented
    pub fn into_interrupt(self) -> Adc<Mode> {
        Adc::<Mode> {
            peripheral: self.peripheral,
            _state: PhantomData,
            onchip_channels: OnChipChannels {
                vss: Some(Vss::<Input> { _mode: PhantomData }),
                temp_sense: Some(TempSense::<Input> { _mode: PhantomData }),
                bandgap: Some(Bandgap::<Input> { _mode: PhantomData }),
                vref_h: Some(VrefH::<Input> { _mode: PhantomData }),
                vref_l: Some(VrefL::<Input> { _mode: PhantomData }),
            },
        };
        unimplemented!("Interrupt is not yet implemented");
    }
}

impl OnChipChannels {
    /// Request an instance of an on-chip Adc Input Channel
    pub fn vss(&mut self) -> Result<Vss<Input>, ()> {
        Ok(self.vss.take().ok_or(())?)
    }

    /// Return the instance of Vss
    pub fn return_vss(&mut self, inst: Vss<Input>) {
        self.vss.replace(inst);
    }

    /// Try to grab an instance of the onchip TempSense channel.
    pub fn tempsense(&mut self) -> Result<TempSense<Input>, ()> {
        Ok(self.temp_sense.take().ok_or(())?)
    }

    /// Return the instance of Vss
    pub fn return_tempsense(&mut self, inst: TempSense<Input>) {
        self.temp_sense.replace(inst);
    }

    /// Try to grab an instance of the onchip Bandgap channel.
    pub fn bandgap(&mut self) -> Result<Bandgap<Input>, ()> {
        Ok(self.bandgap.take().ok_or(())?)
    }

    /// Return the instance of Bandgap
    pub fn return_bandgap(&mut self, inst: Bandgap<Input>) {
        self.bandgap.replace(inst);
    }

    /// Try to grab an instance of the onchip Voltage Reference High Channel.
    pub fn vref_h(&mut self) -> Result<VrefH<Input>, ()> {
        Ok(self.vref_h.take().ok_or(())?)
    }

    /// Return the instance of VrefH
    pub fn return_vref_h(&mut self, inst: VrefH<Input>) {
        self.vref_h.replace(inst);
    }

    /// Try to grab an instance of the onchip Voltage Reference Low Channel.
    pub fn vref_l(&mut self) -> Result<VrefL<Input>, ()> {
        Ok(self.vref_l.take().ok_or(())?)
    }

    /// Return the instance of VrefL
    pub fn return_vref_l(&mut self, inst: VrefL<Input>) {
        self.vref_l.replace(inst);
    }

    /// Grab a DummyDisable instance. Multiple Instances possible.
    pub fn dummy_disable(&self) -> DummyDisable<Input> {
        DummyDisable::<Input> { _mode: PhantomData }
    }
}

/// Holds On-Chip ADC Channel inputs and provides an interface to grab and return them.
pub struct OnChipChannels {
    vss: Option<Vss<Input>>,
    temp_sense: Option<TempSense<Input>>,
    bandgap: Option<Bandgap<Input>>,
    vref_h: Option<VrefH<Input>>,
    vref_l: Option<VrefL<Input>>,
}

/// Dummy type state for on-chip ADC input channels
pub struct Input;

/// Adc Input Channel, measures ground (should be 0?)
pub struct Vss<Input> {
    _mode: core::marker::PhantomData<Input>,
}

/// Adc Input Channel, measures internal temperature sensor
pub struct TempSense<Input> {
    _mode: core::marker::PhantomData<Input>,
}

/// Adc Input Channel, Bandgap internal voltage reference
pub struct Bandgap<Input> {
    _mode: core::marker::PhantomData<Input>,
}

/// Adc Input Channel, Voltage Reference, High
pub struct VrefH<Input> {
    _mode: core::marker::PhantomData<Input>,
}

/// Adc Input Channel, Voltage Reference, Low
pub struct VrefL<Input> {
    _mode: core::marker::PhantomData<Input>,
}

/// Dummy Channel that temporarily disables the Adc Module.
pub struct DummyDisable<Input> {
    _mode: core::marker::PhantomData<Input>,
}

macro_rules! adc_input_channels {
    ( $($Chan:expr => $Pin:ident),+ $(,)*) => {
        $(
            impl<Mode> Channel<Adc<Enabled>> for $Pin<Mode> {
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

impl<Pin> OneShot<Adc<Enabled>, u16, Pin> for Adc<Enabled>
where
    Pin: Channel<Adc<Enabled>, ID = u8>,
{
    type Error = Infallible;

    fn read(&mut self, pin: &mut Pin) -> nb::Result<u16, Self::Error> {
        self.set_channel(pin);
        while !self.is_done() {}
        Ok(self.result())
    }
}
