//! 按键控制 LED

#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::IOPinSpeed;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();
    let dp: pac::Peripherals = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    let mut delay = cp.SYST.delay(&clocks);

    let mut gpiob = dp.GPIOB.split();

    // 上电延时
    delay.delay_ms(20u16);

    // 蜂鸣器
    // 将 pin 引脚配置为推挽式输出
    let mut buzzer = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    // 设置其输出速度（50 MHz）。
    buzzer.set_speed(&mut gpiob.crh, IOPinSpeed::Mhz50);

    // 光敏传感器
    // 将 pin 引脚配置为上拉输入
    let light_sensor = gpiob.pb13.into_pull_up_input(&mut gpiob.crh);

    loop {
        if light_sensor.is_high() {
            buzzer_on(&mut buzzer);
        } else {
            buzzer_off(&mut buzzer);
        }
        // 检测间隔延时
        delay.delay_ms(200_u16);
    }
}

/// 打开蜂鸣器
fn buzzer_on(buzzer: &mut gpio::Pin<'B', 12, gpio::Output>) {
    buzzer.set_high();
}

/// 关闭蜂鸣器
fn buzzer_off(buzzer: &mut gpio::Pin<'B', 12, gpio::Output>) {
    buzzer.set_low();
}
