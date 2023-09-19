#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::{
    _fugit_RateExtU32, _stm32_hal_afio_AfioExt, _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::time::ms;
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let tim2 = dp.TIM2;

    let mut gpioa = dp.GPIOA.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        // .use_hse(8.MHz())
        // 设置系统时钟
        // .sysclk(72.MHz())
        // .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    // TIM2
    // 复用推挽输出
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c1, c2);

    // TIM3
    // let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    // let c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    // let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    // let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // TIM4 (Only available with the "medium" density feature)
    // let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    // let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    // let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    println!("load pwm...");
    //let mut pwm =
    //    Timer::new(tim2, &clocks).pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz());
    // or
    let mut pwm = tim2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    // Enable clock on each of the channels
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);

    // 影响计时器上所有定义通道的操作

    // 将周期调整为0.5秒
    pwm.set_period(ms(500).into_rate());

    // 返回到原始频率
    pwm.set_period(100.kHz());

    // 使处理器处于调试状态。调试器可以将其作为“断点”。
    // 注意：当处理器未连接到调试器时调用bkpt将导致异常。
    asm::bkpt();

    let max = pwm.get_max_duty();

    // 可以通过Pwm对象或通过取消对引脚的引用来访问影响单个通道的操作。

    // 使用Pwm对象将C1设置为最大强度
    pwm.set_duty(Channel::C1, max);

    asm::bkpt();

    // 使用Pwm对象将C1设置为暗淡
    pwm.set_duty(Channel::C1, max / 4);

    asm::bkpt();

    // 使用Pwm对象将C1设置为零
    pwm.set_duty(Channel::C1, 0);

    asm::bkpt();

    // Extract the PwmChannel for C2
    let mut pwm_channel = pwm.split().1;

    // 使用PwmChannel对象将C2设置为全强度
    pwm_channel.set_duty(max);

    asm::bkpt();

    // 使用PwmChannel对象将C2设置为暗淡
    pwm_channel.set_duty(max / 4);

    asm::bkpt();

    // 使用PwmChannel对象将C2设置为零
    pwm_channel.set_duty(0);

    asm::bkpt();

    loop {}
}
