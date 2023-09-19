#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::TimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 基于通用pupose 32位定时器TIM2创建延迟抽象

    //let mut delay = hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
    // or
    let mut delay = dp.TIM2.delay_us(&clocks);

    // 等待计时器触发更新并更改LED的状态
    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            // Use `embedded_hal::DelayMs` trait
            delay.delay_ms(1000_u32);
        }
        // or use `fugit` duration units
        delay.delay(3.secs());
    }
}
