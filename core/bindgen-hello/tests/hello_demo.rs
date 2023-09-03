#![no_std]
#![no_main]

use bindgen_hello::max;
use bindgen_hello::Hello;

use defmt_rtt as _;
use panic_probe as _;

use stm32f1xx_hal as _;

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use super::*;

    use defmt::assert;
    use defmt::assert_eq;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    fn it_hello() {
        assert_eq!(unsafe { Hello() }, 42)
    }

    #[test]
    fn it_max() {
        assert_eq!(unsafe { max(10, 20) }, 20)
    }
}
