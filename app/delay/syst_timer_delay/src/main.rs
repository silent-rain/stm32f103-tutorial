#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use defmt::println;
use nb::block;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Timer;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 将系统计时器配置为每秒触发一次更新
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    // 等待计时器触发更新并更改LED的状态
    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            block!(timer.wait()).unwrap();
        }
    }
}
