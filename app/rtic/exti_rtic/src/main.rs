#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use stm32f1xx_hal::afio::AfioExt;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        delay: SysDelay,
        button: gpio::PB1<gpio::Input<gpio::PullDown>>,
        led: gpio::PA0<gpio::Output<gpio::PushPull>>,
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

        // KEY
        let mut gpiob = ctx.device.GPIOB.split();
        let mut button = gpiob.pb1.into_pull_down_input(&mut gpiob.crl);
        button.make_interrupt_source(&mut afio);
        button.enable_interrupt(&mut ctx.device.EXTI);
        button.trigger_on_edge(&mut ctx.device.EXTI, gpio::Edge::Rising);

        println!("init end ...");
        // 初始化静态资源以稍后通过RTIC使用它们
        (Shared {}, Local { button, led, delay })
    }

    #[task(binds = EXTI1, local = [delay, button, led])]
    fn button_click(ctx: button_click::Context) {
        println!("button click ...");
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
    }
}
