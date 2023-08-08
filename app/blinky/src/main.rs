//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

// 停止恐慌行为
// 这个箱包含一个panic_fmt的实现，它只是在一个无限循环中停止。
use panic_halt as _;

// 将非阻塞表达式$e转换为阻塞操作。
use nb::block;

// 程序入口
use cortex_m_rt::entry;

// 开发板硬件抽象层 (HAL)
use stm32f1xx_hal::{
    pac,
    prelude::{_fugit_RateExtU32, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt},
    rcc::RccExt,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    // 从 cortex-m crate 访问核心外围设备
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    // 从访问外围设备的 crate 中访问特定的外围设备
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    // 获取GPIOC外设
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    // 将gpio C引脚13配置为推挽式输出。
    // “crh”寄存器被传递给函数以配置端口。
    // 对于引脚0-7，应该传递crl。
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    // 将系统计时器配置为每秒触发一次更新
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    // 用于创建表示赫兹的速率的速记。
    timer.start(1.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    // 等待计时器触发更新并更改LED的状态
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
