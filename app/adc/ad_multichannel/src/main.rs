#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_adc_OneShot;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::adc;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::adc::SampleTime;
use stm32f1xx_hal::flash;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::adc1;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_adc_ChannelTimeSequence;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

#[entry]
fn main() -> ! {
    // 初始化外设
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let adc1 = dp.ADC1;
    let syst: pac::SYST = cp.SYST;

    let mut gpioa: gpioa::Parts = dp.GPIOA.split();
    let mut gpiob: gpiob::Parts = dp.GPIOB.split();

    // Alternative configuration using dividers and multipliers directly
    // let clocks = rcc.cfgr.freeze_with_config(
    //     rcc::Config {
    //         hse: Some(8_000_000),
    //         pllmul: Some(7),
    //         hpre: rcc::HPre::DIV1,
    //         ppre1: rcc::PPre::DIV2,
    //         ppre2: rcc::PPre::DIV1,
    //         usbpre: rcc::UsbPre::DIV1_5,
    //         adcpre: rcc::AdcPre::Div6,
    //     },
    //     &mut flash.acr,
    // );
    // println!("sysclk freq: {}", clocks.sysclk().to_Hz());
    // println!("adc freq: {}", clocks.adcclk().to_Hz());

    // 配置ADC时钟默认值是最慢的ADC时钟：PCLK2/8。同时ADC时钟可配置。
    // 因此，它的频率可能会被调整以满足某些实际需求。
    // 使用支持的预分频器值2/4/6/8来近似用户指定的值。
    let clocks = rcc.cfgr.adcclk(72.MHz()).freeze(&mut flash.acr);
    println!("adc freq: {:?}", clocks.adcclk().to_MHz());

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 电位器
    let mut ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);
    // 光敏传感器
    let mut ch1 = gpioa.pa1.into_analog(&mut gpioa.crl);
    let mut ch2 = gpioa.pa2.into_analog(&mut gpioa.crl);
    // 反射式红外传感器
    let mut ch3 = gpioa.pa3.into_analog(&mut gpioa.crl);

    // Setup ADC
    let mut adc1 = Adc::adc1(adc1, clocks);

    // 设置ADC采样时间
    adc1.set_sample_time(SampleTime::T_55);

    // 为特定通道设置ADC采样时间
    adc1.set_channel_sample_time(1, SampleTime::T_55);

    // 设置Adc结果对齐
    adc1.set_align(adc::Align::Right);

    // 外部触发源，软件触发
    adc1.set_external_trigger(adc1::cr2::EXTSEL_A::Swstart);

    // 设置ADC单次转换
    adc1.set_continuous_mode(false);

    // 设置ADC连续转换
    // 当启用连续转换时，转换不会在最后一个选定的组通道停止，而是从第一个选定组通道再次继续。
    // AD 单通道、连续转换、非扫描模式
    // adc1.set_continuous_mode(true);

    // 使用不连续转换（每次转换3个通道）
    // adc1.set_discontinuous_mode(Some(1));

    oled.show_string(1, 1, "AD0:");
    oled.show_string(2, 1, "AD1:");
    oled.show_string(3, 1, "AD2:");
    oled.show_string(4, 1, "AD3:");
    println!("loop ...");
    loop {
        let ad0: u16 = adc1.read(&mut ch0).unwrap();
        let ad1: u16 = adc1.read(&mut ch1).unwrap();
        let ad2: u16 = adc1.read(&mut ch2).unwrap();
        let ad3: u16 = adc1.read(&mut ch3).unwrap();

        println!("ad0={:?} ad1={:?} ad2={:?} ad3={:?}", ad0, ad1, ad2, ad3);
        oled.show_num(1, 5, ad0 as u32, 4);
        oled.show_num(2, 5, ad1 as u32, 1);
        oled.show_num(3, 5, ad2 as u32, 1);
        oled.show_num(4, 5, ad3 as u32, 1);

        delay.delay_ms(100_u32);
    }
}
