//! Port peripheral module.
//!
//! This module contains only the IO Filter part of the port peripheral.
//! The high current driver and pull-up resist peripherals are implemented as
//! part of the GPIO module.

use crate::pac::PORT;
use core::marker::PhantomData;

/// Default Divisor for each port
pub type DefaultDiv = DivNone;

/// No Divisor (Filter runs a busclk, or disabled)
pub struct DivNone;

/// Use Divisor 1
pub struct Div1;

/// Use Divisor 2
pub struct Div2;

/// Use Divisor 3
pub struct Div3;

/// A struct that represents one of the divisors in the filter module
pub struct Divisor<Offset, Size> {
    _offset: PhantomData<Offset>,
    _size: PhantomData<Size>,
}

fn set_port_divisor(divisor: u32, offset: usize) {
    unsafe {
        let port = &(*PORT::ptr());
        port.ioflt
            .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (divisor << offset)));
    }
}

macro_rules! port_filter {
    ([ $($PORT:ident: ($port:ident, $PORTOffset:expr),)+ ],
     [ $($DIV:ident: ($div:ident, $DIVOffset:expr, $DIVSize:expr), )+ ]) => {
        /// Input Filter control
        ///
        /// per 11.4 of KEA64 ref man, filter acts as lowpass with adjustable
        /// timing. holds the pin in previous state until new state exists for
        /// longer than adjustable time. This feature is effectively a lowpass
        /// filter.

        // @TODO This should probably have some knowledge of the clocks involved
        // in order to abstract the divisors into a cut-off frequency or
        // something.
        // FLTDIV3 -> LPOCLK, FLTDIV2 -> BUSCLK, FLTDIV1 -> BUSCLK
        pub struct Filter {
            $(
                /// Divisor Controller
                pub $div: $DIV,
            )+
            $(
                /// Filterable Port
                pub $port: $PORT<DefaultDiv>,
            )+
        }

        impl Filter {
            /// Get the Filter interface.
            pub fn get() -> Self {
                Filter {
                    $(
                        $div: $DIV {},
                    )+
                    $(
                        $port: $PORT { _div: PhantomData },
                    )+
                }
            }
        }

        $(
            /// Divisor Controller
            pub struct $DIV;


            impl $DIV {

                /// Sets the divisor the filtered port.
                ///
                /// See the PORT_IOFLT documentation in the manual for values.
                /// @TODO Use (cutoff) frequency or the actual value of the
                /// divisor. In either case match it to one of the available
                /// hardware divisors
                pub fn set_divisor(self, divisor: u8) {
                    assert!(divisor & !($DIVSize) == 0);
                    unsafe {
                        (*PORT::ptr()).ioflt.modify(|r, w| {
                            w.bits((r.bits() & !($DIVSize << $DIVOffset)) | ((divisor as u32) << $DIVOffset))
                        })
                    }
                }
            }
        )+
        $(
            /// A Filterable Port type
            pub struct $PORT<DIV> {
                _div: PhantomData<DIV>,
            }

            impl<DIV> $PORT<DIV> {
                /// Configure $PORT to use no divisor (or disable filter)
                pub fn into_no_div(self) -> $PORT<DivNone> {
                    set_port_divisor(0b00, $PORTOffset);
                    $PORT {_div: PhantomData}
                }

                /// Configure $PORT to use Div1
                pub fn into_div1(self) -> $PORT<Div1> {
                    set_port_divisor(0b01, $PORTOffset);
                    $PORT {_div: PhantomData}
                }

                /// Configure $PORT to use Div2
                pub fn into_div2(self) -> $PORT<Div2> {
                    set_port_divisor(0b10, $PORTOffset);
                    $PORT {_div: PhantomData}
                }

                /// Configure $PORT to use Div3
                pub fn into_div3(self) -> $PORT<Div3> {
                    set_port_divisor(0b11, $PORTOffset);
                    $PORT {_div: PhantomData}
                }
            }
        )+
    }
}

port_filter!([
    PORTA: (porta, 0),
    PORTB: (portb, 2),
    PORTC: (portc, 4),
    PORTD: (portd, 8),
    PORTE: (porte, 10),
    PORTF: (portf, 12),
    PORTG: (portg, 14),
], [
    DIV1: (div1, 29, 7),
    DIV2: (div2, 26, 7),
    DIV3: (div3, 24, 3),
]);
