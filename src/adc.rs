//! The ADC Interface
//!
//! The ADC can be polled for conversion completion with [is_done]. Completion
//! will trigger an ADC Interrupt if enabled. See [into_interrupt]
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
//! ## Conversion Width - [Resolution]
//!
//! The ADC can be run in 8, 10, or 12 bit modes
//!
//! ## Hardware Trigger
//!
//! The ADC conversions can be started by a hardware trigger. This is not
//! implemented in all KEA chips, so implementation here will be Delayed. Use
//! the PAC. Enable is ADC_SC2[ADTRG] = 1, and trigger is the ADHWT source.

use crate::hal::adc::{Channel, OneShot};
use crate::{pac::ADC, HALExt};
use core::marker::PhantomData;
use embedded_time::{duration, rate};

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
    clock: AdcClocks,
    clock_freq: rate::Megahertz,
    req_ADC_clock: rate::Megahertz,
    resolution: Resolution,
    req_sample_time: duration::Nanoseconds,
}

/// Clock types available to the Adc peripheral
///
/// Dividers will be chosen appropriately to suit requested clock rate.
pub enum AdcClocks {
    /// asdf
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
pub enum Resolution {
    /// 8 bit AD conversion mode
    _8bit = 0,
    /// 10 bit AD conversion mode
    _10bit = 1,
    /// 12 bit AD conversion mode
    _12bit = 2,
}

/// Adc sample time
pub enum SampleTime {
    /// Sample for 3.5 ADC clock (ADCK) cycles.
    Short = 0,

    /// Sample for 23.5 ADC clock (ADCK) cycles.
    ///
    /// Used for high impedence (>2k @ADCK > 4MHz, >5k @ ADCK < 4MHz
    Long = 1,
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
    /// In OneShot mode, setting the channel begins the conversion. In FIFO mode
    /// the channel is added to the FIFO buffer.
    pub fn set_channel<T: Channel<Adc<Enabled>, ID = u8>>(&self, _pin: &T) {
        self.peripheral
            .sc1
            .modify(|_, w| unsafe { w.adch().bits(T::channel()) });
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
);

impl<Pin> OneShot<Adc<Enabled>, u16, Pin> for Adc<Enabled>
where
    Pin: Channel<Adc<Enabled>, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut Pin) -> nb::Result<u16, Self::Error> {
        self.peripheral
            .sc1
            .modify(|_, w| unsafe { w.adch().bits(Pin::channel()) });
        while self.peripheral.sc2.read().adact().bit_is_set() {}
        Ok(self.peripheral.r.read().adr().bits())
    }
}
