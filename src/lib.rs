#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _; // global logger

// adjust HAL import
use stm32f1xx_hal as _; // memory layout

use panic_probe as _;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod tests {
    use super::*;

    use defmt::assert_eq;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
