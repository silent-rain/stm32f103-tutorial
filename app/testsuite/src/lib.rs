#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal as _;

use defmt_rtt as _; // global logger
                    // use panic_rtt_target as _;
                    // use rtt_target::rtt_init_print;

use panic_probe as _;

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    loop {}
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// See https://crates.io/crates/defmt-test/0.1.0 for more documentation (e.g. about the 'state'
// feature)
// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[cfg(test)]
#[defmt_test::tests]
mod tests {
    use defmt::{assert, assert_eq};

    #[test]
    fn assert_true() {
        assert!(true)
    }

    #[test]
    fn assert_eq() {
        assert_eq!(24, 42, "TODO: write actual tests")
    }
}
