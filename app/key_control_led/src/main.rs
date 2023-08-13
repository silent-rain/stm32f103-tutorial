//! 按键控制 LED

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::IOPinSpeed;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::gpio::PullUp;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;

use panic_halt as _;

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

    // 获取GPIO外围设备
    let mut gpioa = dp.GPIOA.split();

    // LED
    // 将 pin 引脚配置为推挽式输出。
    let mut led1 = gpioa
        .pa1
        .into_push_pull_output_with_state(&mut gpioa.crl, PinState::High);
    let mut led2 = gpioa
        .pa2
        .into_push_pull_output_with_state(&mut gpioa.crl, PinState::High);
    // 设置其输出速度（50 MHz）。
    led1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);
    led2.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    let mut gpiob = dp.GPIOB.split();

    // 按键
    // 将引脚配置为作为上拉输入引脚操作
    // 输入模式中速度是没有用的, 无需配置
    let mut key1 = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
    let mut key11 = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);

    // 等待计时器触发更新并更改LED的状态
    loop {
        // 获取按键的值
        let key_num = get_key_num(&mut key1, &mut key11, &mut delay);
        // 根据不同的按键点亮不同的灯
        if key_num == 1 {
            led1_turn(&mut led1);
        }
        if key_num == 11 {
            led2_turn(&mut led2);
        }
    }
}

/// 获取按键的值
fn get_key_num(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<PullUp>>,
    key11: &mut gpio::Pin<'B', 11, gpio::Input<PullUp>>,
    delay: &mut SysDelay,
) -> i32 {
    let mut key_num = 0;

    if key1.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key1.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        key_num = 1;
    }

    if key11.is_low() {
        // 按键按下抖动
        delay.delay_ms(20_u16);
        // 按着不动, 松手后跳出循环
        while key11.is_low() {}
        // 按键松开抖动
        delay.delay_ms(20_u16);

        key_num = 11;
    }
    key_num
}

/// led1 端口电平翻转
fn led1_turn(led1: &mut gpio::Pin<'A', 1, gpio::Output>) {
    if led1.is_set_low() {
        led1.set_high();
    } else {
        led1.set_low();
    }
}

/// led2 端口电平翻转
fn led2_turn(led2: &mut gpio::Pin<'A', 2, gpio::Output>) {
    if led2.is_set_low() {
        led2.set_high();
    } else {
        led2.set_low();
    }
}
