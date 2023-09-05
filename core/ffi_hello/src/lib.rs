#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use stm32f1xx_hal as _;

#[allow(unused)]
extern "C" {
    #[link_name = "Hello"]
    pub fn hello() -> i32;
    pub fn max(num1: i32, num2: i32) -> i32;
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use super::*;

    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    fn it_hello() {
        assert_eq!(unsafe { hello() }, 42)
    }

    #[test]
    fn it_max() {
        assert_eq!(unsafe { max(10, 20) }, 20)
    }
}
