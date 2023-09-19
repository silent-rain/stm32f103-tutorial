#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use cortex_m::asm::wfi;
use cortex_m::interrupt::Mutex;
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::pac::Interrupt;
use stm32f1xx_hal::pac::EXTI;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::rtc::Rtc;
use stm32f1xx_hal::timer::SysTimerExt;

// A type definition for the GPIO pin to be used for our LED
type LedPin = gpioa::PA0<Output<PushPull>>;

// Make LED pin globally available
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
// Make RTC globally available
static G_RTC: Mutex<RefCell<Option<Rtc>>> = Mutex::new(RefCell::new(None));
// Make EXTI registers globally available
static G_EXTI: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

// Toggle LED every 3 seconds
const TOGGLE_INTERVAL_SECONDS: u32 = 3;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut pwr = dp.PWR;

    let mut gpioa = dp.GPIOA.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 设置RTC
    // 启用对备份域的写入
    let mut backup_domain = rcc.bkp.constrain(dp.BKP, &mut pwr);
    // 启动RTC
    let mut rtc = Rtc::new(dp.RTC, &mut backup_domain);
    // 将当前RTC计数器值设置为指定值
    rtc.set_time(0);
    // 设置触发警报的时间
    // 如果设置了报警标志，也会清除该标志
    rtc.set_alarm(TOGGLE_INTERVAL_SECONDS);
    // 使RTC中断在计数器达到报警值时触发。
    // 此外，如果EXTI控制器设置正确，此功能也会启用RTCALARM中断。
    rtc.listen_alarm();

    cortex_m::interrupt::free(|cs| *G_RTC.borrow(cs).borrow_mut() = Some(rtc));

    // 初始化 LED
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    led.set_high(); // Turn off
    cortex_m::interrupt::free(|cs| *G_LED.borrow(cs).borrow_mut() = Some(led));

    // 设置EXTI（参见参考手册第18.4.2节中的注释）
    let exti = dp.EXTI;
    // 下降触发器选择寄存器
    exti.ftsr.write(|w| w.tr17().set_bit());
    // 中断屏蔽寄存器
    exti.imr.write(|w| w.mr17().set_bit());
    cortex_m::interrupt::free(|cs| G_EXTI.borrow(cs).replace(Some(exti)));

    // Enable RTCALARM IRQ
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::RTCALARM) };

    println!("loop");
    loop {
        wfi();
    }
}

#[interrupt]
fn RTCALARM() {
    static mut LED: Option<LedPin> = None;
    static mut RTC: Option<Rtc> = None;
    static mut EXTI: Option<EXTI> = None;

    let led = LED.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_LED.borrow(cs).replace(None).unwrap())
    });
    let rtc = RTC.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_RTC.borrow(cs).replace(None).unwrap())
    });
    let exti = EXTI.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_EXTI.borrow(cs).replace(None).unwrap())
    });

    // 挂起寄存器
    exti.pr.write(|w| w.pr17().set_bit());

    let current_time = rtc.current_time();
    println!("current_time: {:?}", current_time);

    // 设置触发警报的时间
    rtc.set_alarm(current_time + TOGGLE_INTERVAL_SECONDS);

    led.toggle();
}
