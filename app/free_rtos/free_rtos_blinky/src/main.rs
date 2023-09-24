#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::IOPinSpeed;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Timer;

use freertos_rust::{task_control, Duration, Task};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    let mut led1 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    led1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    let mut led2 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    led2.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    loop {
        // delay.delay_ms(1000_u32);
        // led.set_high();
        // delay.delay_ms(1000_u32);
        // led.set_low();

        // 创建两个任务
        let _task1 = Task::new()
            .name("Task1")
            .stack_size(TASK_STACK_SIZE)
            .priority(TASK1_PRIORITY)
            .start(task1)
            .unwrap();

        let _task2 = Task::new()
            .name("Task2")
            .stack_size(TASK_STACK_SIZE)
            .priority(TASK2_PRIORITY)
            .start(task2)
            .unwrap();

        // 启动调度器
        task_control::start_scheduler();

        // 永远不会到达这里
        loop {}
    }
}

fn task1(_handle: Task) {
    loop {
        led1.set_high();
        task_control::delay(Duration::ms(500));
        led1.set_low();
        task_control::delay(Duration::ms(500));
    }
}

fn task2(_handle: Task) {
    loop {
        led2.set_high();
        task_control::delay(Duration::ms(1000));
        led2.set_low();
        task_control::delay(Duration::ms(1000));
    }
}
