#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Timer;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    let mut pin = gpioa.pa0.into_floating_input(&mut gpioa.crl);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        println!("{}", pin.is_high());
        pin.as_push_pull_output(&mut gpioa.crl, |out| {
            out.set_high();
            block!(timer.wait()).unwrap();
            out.set_low();
            block!(timer.wait()).unwrap();
        });
        pin.as_push_pull_output_with_state(&mut gpioa.crl, PinState::High, |out| {
            block!(timer.wait()).unwrap();
            out.set_low();
            block!(timer.wait()).unwrap();
        });
    }
}
