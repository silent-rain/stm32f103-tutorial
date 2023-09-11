#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use mpu6050::Mpu6050;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::i2c::BlockingI2c;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::{_stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::{gpio, i2c};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let i2c2 = dp.I2C2;

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks: stm32f1xx_hal::rcc::Clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);

    // MPU6050 初始化
    let mpu_scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let mpu_sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);
    let pins = (mpu_scl, mpu_sda);
    // 创建i2c实例
    let i2c = BlockingI2c::i2c2(
        i2c2,
        pins,
        i2c::Mode::standard(10.kHz()),
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    // 创建mpu6050实例，使用默认的从机地址和灵敏度
    let mut mpu = Mpu6050::new(i2c);

    // 初始化mpu6050
    mpu.init(&mut delay).unwrap();

    // 获取 mpu6050 ID
    let address = mpu.read_byte(mpu6050::device::WHOAMI).unwrap();
    println!("mpu6050 address: {}", address);

    // 循环读取加速度和角速度数据
    loop {
        // 获取温度数据，单位为摄氏度
        let temp = mpu.get_temp().unwrap();
        println!("Temperature: {}°C", temp);

        // 获取加速度数据，单位为g
        let acc = mpu.get_acc().unwrap();
        println!("Accel: ({}, {}, {})", acc.x, acc.y, acc.z);

        // 获取角速度数据，单位为弧度每秒
        let gyro = mpu.get_gyro().unwrap();
        println!("Gyro: ({}, {}, {})", gyro.x, gyro.y, gyro.z);

        // 延时一秒
        delay.delay_ms(1000_u32);
    }
}

/// 初始化 OLED 显示屏
pub fn init_oled(
    pb8: gpio::Pin<'B', 8>,
    pb9: gpio::Pin<'B', 9>,
    crh: &mut gpio::Cr<'B', true>,
) -> (
    gpio::PB8<gpio::Output<gpio::OpenDrain>>,
    gpio::PB9<gpio::Output<gpio::OpenDrain>>,
) {
    // 将引脚配置为作为开漏输出模式
    let mut scl = pb8.into_open_drain_output(crh);
    let mut sda = pb9.into_open_drain_output(crh);
    scl.set_speed(crh, gpio::IOPinSpeed::Mhz50);
    sda.set_speed(crh, gpio::IOPinSpeed::Mhz50);

    // 始化 OLED 配置
    hardware::oled::init_oled_config(&mut scl, &mut sda);
    (scl, sda)
}
