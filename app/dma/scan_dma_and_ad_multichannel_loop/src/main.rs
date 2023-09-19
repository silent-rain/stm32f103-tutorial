#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::adc;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::adc::SetChannels;
use stm32f1xx_hal::dma;
use stm32f1xx_hal::dma::Half;
use stm32f1xx_hal::flash;
use stm32f1xx_hal::gpio::gpioa;
use stm32f1xx_hal::gpio::Analog;
use stm32f1xx_hal::gpio::{PA0, PA1, PA2, PA3};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::adc1;
use stm32f1xx_hal::pac::ADC1;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::{
    _stm32_hal_adc_ChannelTimeSequence, _stm32_hal_dma_CircReadDma, _stm32_hal_dma_DmaExt,
    _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt,
};
use stm32f1xx_hal::rcc;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysDelay;
use stm32f1xx_hal::timer::SysTimerExt;

static mut AD_VALUE: [[u16; 8]; 2] = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];

pub struct AdcPins(PA0<Analog>, PA1<Analog>, PA2<Analog>, PA3<Analog>);

impl SetChannels<AdcPins> for Adc<ADC1> {
    fn set_samples(&mut self) {
        // 为特定通道设置ADC采样时间
        self.set_channel_sample_time(0, adc::SampleTime::T_55);
        self.set_channel_sample_time(1, adc::SampleTime::T_55);
        self.set_channel_sample_time(2, adc::SampleTime::T_55);
        self.set_channel_sample_time(3, adc::SampleTime::T_55);
    }

    fn set_sequence(&mut self) {
        // ADC设置常规通道转换序列
        // 定义要转换为常规组的通道序列。
        self.set_regular_sequence(&[0, 1, 2, 3]);
        // 我们可以选择设置连续扫描模式
        self.set_continuous_mode(true);
        // 我们还可以使用不连续转换（每次转换4个通道）
        self.set_discontinuous_mode(Some(8));
    }
}

#[entry]
fn main() -> ! {
    // 初始化外设
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash: flash::Parts = dp.FLASH.constrain();
    let rcc: rcc::Rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let dma_ch1 = dp.DMA1.split().1;

    let mut gpioa: gpioa::Parts = dp.GPIOA.split();

    // 配置ADC时钟默认值是最慢的ADC时钟：PCLK2/8。同时ADC时钟可配置。
    // 因此，它的频率可能会被调整以满足某些实际需求。
    // 使用支持的预分频器值2/4/6/8来近似用户指定的值。
    let clocks = rcc.cfgr.adcclk(72.MHz()).freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // 初始化 ADC
    let adc1 = dp.ADC1;
    // Setup ADC
    let mut adc1 = Adc::adc1(adc1, clocks);
    // 设置Adc结果对齐
    adc1.set_align(adc::Align::Right);
    // 外部触发源，软件触发
    adc1.set_external_trigger(adc1::cr2::EXTSEL_A::Swstart);
    // 设置ADC单次转换
    adc1.set_continuous_mode(true);

    // Configure analog input
    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);
    let adc_ch1 = gpioa.pa1.into_analog(&mut gpioa.crl);
    let adc_ch2 = gpioa.pa2.into_analog(&mut gpioa.crl);
    let adc_ch3 = gpioa.pa3.into_analog(&mut gpioa.crl);
    let pins = AdcPins(adc_ch0, adc_ch1, adc_ch2, adc_ch3);

    let adc_dma: stm32f1xx_hal::dma::RxDma<
        adc::AdcPayload<ADC1, AdcPins, adc::Scan>,
        stm32f1xx_hal::dma::dma1::C1,
    > = adc1.with_scan_dma(pins, dma_ch1);

    loop_transfer(adc_dma, &mut delay);
    println!("loop");
    loop {
        wfi();
    }
}

type AdcDma = dma::RxDma<adc::AdcPayload<ADC1, AdcPins, adc::Scan>, dma::dma1::C1>;

#[allow(unconditional_recursion)]
fn loop_transfer(adc_dma: AdcDma, delay: &mut SysDelay) {
    let mut circ_buffer = adc_dma.circ_read(unsafe { &mut AD_VALUE });

    let mut buf: [[u16; 8]; 2] = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
    if let Ok(v) = circ_buffer.readable_half() {
        if v == Half::First {
            buf[0] = circ_buffer.peek(|half, _| *half).unwrap();
            println!("First");
        }
        if v == Half::Second {
            buf[1] = circ_buffer.peek(|half, _| *half).unwrap();
            println!("Second");
        }
    }

    // 停止传输并返回基础缓冲区和RxDma
    let (buf, adc_dma) = circ_buffer.stop();
    println!("{:?} ", buf);
    delay.delay_ms(1000_u32);
    loop_transfer(adc_dma, delay);
}
