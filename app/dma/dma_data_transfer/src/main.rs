#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::flash;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_dma_DmaExt;
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
    let syst = cp.SYST;
    let mut dma_ch1 = dp.DMA1.split().1;

    let mut gpiob: gpiob::Parts = dp.GPIOB.split();

    // 配置ADC时钟默认值是最慢的ADC时钟：PCLK2/8。同时ADC时钟可配置。
    // 因此，它的频率可能会被调整以满足某些实际需求。
    // 使用支持的预分频器值2/4/6/8来近似用户指定的值。
    let clocks = rcc.cfgr.adcclk(72.MHz()).freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 存储器到存储器转运
    // 定义u8类型的数组
    let data_a: [u8; 4] = [1, 2, 3, 4];

    // Flash 到 SRAM 转运, 使用 const
    // const data_a: [u8; 4] = [1, 2, 3, 4];

    // 定义u8类型的数组
    let data_b: [u8; 4] = [0, 0, 0, 0];

    // 关联的外围设备地址
    // inc指示地址是否在每次字节传输后递增
    dma_ch1.set_peripheral_address(data_a.as_ptr() as u32, true);
    // address where from/to data will be read/write
    // inc指示地址是否在每次字节传输后递增
    // 数组间的转运需要自增
    dma_ch1.set_memory_address(data_b.as_ptr() as u32, true);
    // 要传输的字节数
    // 注意：这里要乘以源数据的字节宽度
    #[allow(clippy::identity_op)]
    dma_ch1.set_transfer_length(data_b.len() * 1);

    // 数据传输方向
    dma_ch1.ch().cr.modify(|_, w| w.dir().from_peripheral());

    dma_ch1.ch().cr.modify(|_, w| {
        w.mem2mem()
            .enabled()
            .pl()
            .medium()
            // .msize()
            // .bits8()
            // .psize()
            // .bits32()
            // .circ()
            // .clear_bit()
            .dir()
            .from_peripheral()
    });

    // 启动DMA传输
    dma_ch1.start();

    oled.show_hex_num(1, 1, data_a[0].into(), 2);
    oled.show_hex_num(1, 4, data_a[1].into(), 2);
    oled.show_hex_num(1, 7, data_a[2].into(), 2);
    oled.show_hex_num(1, 10, data_a[3].into(), 2);
    oled.show_hex_num(2, 1, data_b[0].into(), 2);
    oled.show_hex_num(2, 4, data_b[1].into(), 2);
    oled.show_hex_num(2, 7, data_b[2].into(), 2);
    oled.show_hex_num(2, 10, data_b[3].into(), 2);

    oled.show_hex_num(3, 1, data_a[0].into(), 2);
    oled.show_hex_num(3, 4, data_a[1].into(), 2);
    oled.show_hex_num(3, 7, data_a[2].into(), 2);
    oled.show_hex_num(3, 10, data_a[3].into(), 2);
    oled.show_hex_num(4, 1, data_b[0].into(), 2);
    oled.show_hex_num(4, 4, data_b[1].into(), 2);
    oled.show_hex_num(4, 7, data_b[2].into(), 2);
    oled.show_hex_num(4, 10, data_b[3].into(), 2);
    loop {
        wfi();
    }
}
