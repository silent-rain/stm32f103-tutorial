#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac::{Interrupt, NVIC, TIM1};
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::prelude::_stm32f4xx_hal_timer_TimerExt;
use stm32f1xx_hal::timer::CounterMs;
use stm32f1xx_hal::timer::Event;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::PA0<gpio::Output<gpio::PushPull>>,
        timer_handler: CounterMs<TIM1>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        let mut nvic = ctx.core.NVIC;

        let mut gpioa = ctx.device.GPIOA.split();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // LED
        let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
        led.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);

        // TIM1 定时器
        let mut timer = ctx.device.TIM1.counter_ms(&clocks);
        timer.start(1.secs()).unwrap();
        timer.listen(Event::Update);

        unsafe {
            // Enable interruptions
            NVIC::unmask(Interrupt::TIM2);
            // 设置中断的优先级
            nvic.set_priority(Interrupt::TIM2, 1);
        }

        // 初始化静态资源以稍后通过RTIC使用它们
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // Optional.
    //
    // https://rtic.rs/dev/book/en/by-example/app_idle.html
    // > 当没有声明空闲功能时，运行时设置 SLEEPONEXIT 位，然后在运行 init 后将微控制器发送到睡眠状态。
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // 根据应用程序的不同，如果您想最大限度地减少具有相同优先级的中断的延迟（如果有），
        // 您可能希望将此处完成的一些工作委派给空闲任务。
        if *cx.local.led_state {
            // 使用rtic管理的资源关闭 led
            cx.local.led.set_high();
            *cx.local.led_state = false;
        } else {
            cx.local.led.set_low();
            *cx.local.led_state = true;
        }
        // 用于更改计时器更新频率的计数
        *cx.local.count += 1;

        if *cx.local.count == 4 {
            // 更改计时器更新频率
            cx.local.timer_handler.start(500.millis()).unwrap();
        } else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
        }

        println!("count: {:?}", cx.local.count);

        // 清除更新标志
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
