#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use hardware::mpu6050::mpu6050_hal;
use hardware::oled;

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::{_stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt};
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

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

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // MPU6050 初始化
    let mpu_scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let mpu_sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);
    let mut mpu = mpu6050_hal::init((mpu_scl, mpu_sda), i2c2, clocks);

    let id = mpu6050_hal::get_id(&mut mpu);
    oled::show_string(&mut scl, &mut sda, 1, 1, "ID:");
    oled::show_hex_num(&mut scl, &mut sda, 1, 4, id as u32, 2);

    // 循环读取加速度和角速度数据
    loop {
        let data = mpu6050_hal::get_data(&mut mpu);
        // 打印读取到的数据
        println!("Accel: ({}, {}, {})", data.acc_x, data.acc_y, data.acc_z);
        println!("Gyro: ({}, {}, {})", data.gyro_x, data.gyro_y, data.gyro_z);

        oled::show_signed_num(&mut scl, &mut sda, 2, 1, data.acc_x as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 3, 1, data.acc_y as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 4, 1, data.acc_z as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 2, 8, data.gyro_x as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 3, 8, data.gyro_y as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 4, 8, data.gyro_z as i32, 5);

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
