#![no_std]
#![no_main]

use defmt_rtt as _; // global logger

// adjust HAL import
use stm32f1xx_hal as _; // memory layout

use panic_probe as _;

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
