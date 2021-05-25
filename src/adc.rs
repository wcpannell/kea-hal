///! The ADC Interface
///
///Talks to stuff for reasons
use crate::init_state;

/// State type fore ADC Peripheral
pub struct ADC<State = init_state::Enabled> {
    adc: pac::ADC,
    _state: State,
}
