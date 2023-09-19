//! 按键控制 LED

#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::flash::{self, FlashExt};
use stm32f1xx_hal::gpio::{self, gpioa, gpiob, GpioExt, OutputSpeed};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::rcc::{self, RccExt};
use stm32f1xx_hal::timer::{SysDelay, SysTimerExt};

#[entry]
fn main() -> ! {
    // 初始化外设
    let (flash, rcc, system_timer, gpioa, gpiob) = init_peripheral();

    // 封装具有自定义精度的阻塞延迟函数
    let mut delay = sys_delay(flash, rcc, system_timer);

    // LED
    let (mut led1, mut led2) = init_led(gpioa);
    // 按键
    let (mut key1, mut key11) = init_key(gpiob);

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

/// 初始化外设
fn init_peripheral() -> (
    flash::Parts,
    rcc::Rcc,
    cortex_m::peripheral::SYST,
    gpioa::Parts,
    gpiob::Parts,
) {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let system_timer = cp.SYST;
    let gpioa: gpioa::Parts = dp.GPIOA.split();
    let gpiob: gpiob::Parts = dp.GPIOB.split();
    (flash, rcc, system_timer, gpioa, gpiob)
}

/// 封装具有自定义精度的阻塞延迟函数
fn sys_delay(
    mut flash: flash::Parts,
    rcc: rcc::Rcc,
    system_timer: cortex_m::peripheral::SYST,
) -> SysDelay {
    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟
    system_timer.delay(&clocks)
}

/// LED
/// 将 pin 引脚配置为推挽式输出
fn init_led(
    mut gpioa: gpioa::Parts,
) -> (
    gpio::PA1<gpio::Output<gpio::PushPull>>,
    gpio::PA2<gpio::Output<gpio::PushPull>>,
) {
    let mut led1 = gpioa
        .pa1
        .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);
    let mut led2 = gpioa
        .pa2
        .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);
    // 设置其输出速度（50 MHz）。
    led1.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    led2.set_speed(&mut gpioa.crl, gpio::IOPinSpeed::Mhz50);
    (led1, led2)
}

/// 按键
/// 将引脚配置为作为上拉输入引脚操作
/// 输入模式中速度是没有用的, 无需配置
fn init_key(
    mut gpiob: gpiob::Parts,
) -> (
    gpio::PB1<gpio::Input<gpio::PullUp>>,
    gpio::PB11<gpio::Input<gpio::PullUp>>,
) {
    let key1 = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
    let key11 = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);
    (key1, key11)
}

/// 获取按键的值
fn get_key_num(
    key1: &mut gpio::Pin<'B', 1, gpio::Input<gpio::PullUp>>,
    key11: &mut gpio::Pin<'B', 11, gpio::Input<gpio::PullUp>>,
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
