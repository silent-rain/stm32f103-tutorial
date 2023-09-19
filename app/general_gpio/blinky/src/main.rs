//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

// 用于处理错误情况；
use defmt_rtt as _;
use panic_probe as _;

// 将非阻塞表达式转换为阻塞操作；
use nb::block;

// 用于标记程序入口；
use cortex_m_rt::entry;

// 可用于输出和相关AlternateMode引脚的回转速率
use stm32f1xx_hal::gpio::IOPinSpeed;
// 用于设置IO引脚的转换速率; 最初，所有引脚都设置为最大转换速率
use stm32f1xx_hal::gpio::OutputSpeed;
// 微控制器的外围访问API
use stm32f1xx_hal::pac;
// u32速率的简单短手的扩展特性
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
// 用于约束FLASH外围设备的扩展特性
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
// 在独立引脚和寄存器中拆分GPIO外设的扩展特性
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
// 限制Rcc外围设备的扩展特性
use stm32f1xx_hal::rcc::RccExt;
// 计时器包装
use stm32f1xx_hal::timer::Timer;

// 标记接下来的函数是程序的入口点；
#[entry]
fn main() -> ! {
    // 获取对核心外设的访问权限
    let cp = cortex_m::Peripherals::take().unwrap();
    // 获取对特定设备外设的访问权限
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 通过拆分 GPIOA 区块，获取对其各引脚的互斥访问。
    let mut gpioa = dp.GPIOA.split();

    // 将 PA0 引脚配置为推挽式输出。
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    // 设置其输出速度（50 MHz）。
    // 然后在接下来的代码中，我们将使用该引脚来控制 LED 的状态。
    led.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    // 将系统计时器配置为每秒触发一次更新
    // 创建定时器。将把定时器用于切换 led 状态。
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    // 设置定时器的频率
    timer.start(1.Hz()).unwrap();

    // 等待计时器触发更新并更改LED的状态
    // 无限循环，每秒钟切换 LED 状态（高电平/低电平）。
    loop {
        // 等待定时器
        block!(timer.wait()).unwrap();
        // 设置高电平
        led.set_high();
        block!(timer.wait()).unwrap();
        // 设置低电平
        led.set_low();
    }
}
