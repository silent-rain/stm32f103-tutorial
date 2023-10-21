#![no_std]
#![no_main]

// 定义应用程序资源和任务
#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
mod app {
    use defmt::println;
    use defmt_rtt as _;
    use panic_probe as _;

    use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
    use stm32f1xx_hal::gpio::Edge;
    use stm32f1xx_hal::gpio::OutputSpeed;
    use stm32f1xx_hal::{
        afio::AfioExt,
        flash::FlashExt,
        gpio::{self, ExtiPin},
        pac::{Interrupt, NVIC},
        prelude::{_stm32_hal_gpio_GpioExt, _stm32_hal_rcc_RccExt},
        timer::{SysDelay, SysTimerExt},
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        delay: SysDelay,
        button: gpio::PB1<gpio::Input<gpio::PullUp>>,
        led: gpio::PA0<gpio::Output<gpio::PushPull>>,
    }

    // 初始化函数
    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        // 获取外设实例
        let mut afio = ctx.device.AFIO.constrain();
        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        let syst = ctx.core.SYST;
        let mut nvic = ctx.core.NVIC;

        let mut gpioa = ctx.device.GPIOA.split();
        let mut gpiob = ctx.device.GPIOB.split();

        // 初始化时钟
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        // 具有自定义精度的阻塞延迟
        let mut delay = syst.delay(&clocks);

        delay.delay_ms(1000_u16);
        println!("init start ...");

        // LED
        let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
        led.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);

        // KEY
        let mut button = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
        // 配置 AFIO 外部中断引脚选择
        button.make_interrupt_source(&mut afio);
        // 从该引脚启用外部中断
        button.enable_interrupt(&mut ctx.device.EXTI);
        // 下升沿生成中断
        button.trigger_on_edge(&mut ctx.device.EXTI, Edge::RisingFalling);

        // 使能中断
        unsafe {
            NVIC::unmask(Interrupt::EXTI1);
            nvic.set_priority(Interrupt::EXTI1, 1);
        }

        println!("init end ...");
        // 初始化静态资源以稍后通过RTIC使用它们
        (Shared {}, Local { button, led, delay })
    }

    // 中断处理函数
    #[task(binds = EXTI1, local = [delay, button, led])]
    fn button_click(ctx: button_click::Context) {
        println!("button click ...");
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
    }

    // 空闲任务，什么也不做
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            continue;
        }
    }
}
