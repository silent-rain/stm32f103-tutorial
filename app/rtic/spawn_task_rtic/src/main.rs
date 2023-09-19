#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(type_alias_impl_trait)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use rtic_monotonics::systick::Systick;
use stm32f1xx_hal::afio::AfioExt;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::PA0<gpio::Output<gpio::PushPull>>,
        state: bool,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        let mut afio = ctx.device.AFIO.constrain();
        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        let syst = ctx.core.SYST;

        // 初始化时钟
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // 具有自定义精度的阻塞延迟
        let delay = syst.delay(&clocks);

        // LED
        let mut gpioa = ctx.device.GPIOA.split();
        let led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

        // Schedule the blinking task
        button_click::spawn().ok();

        // 初始化静态资源以稍后通过RTIC使用它们
        (Shared {}, Local { led, state: false })
    }

    #[task(local = [state, led])]
    async fn button_click(ctx: button_click::Context) {
        loop {
            println!("blink");
            if *ctx.local.state {
                ctx.local.led.set_high();
                *ctx.local.state = false;
            } else {
                ctx.local.led.set_low();
                *ctx.local.state = true;
            }
            Systick::delay(1000.millis()).await;
        }
    }
}
