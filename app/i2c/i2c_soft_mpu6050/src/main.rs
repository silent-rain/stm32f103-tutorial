#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use hardware::{mpu6050, oled};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

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

    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 具有自定义精度的阻塞延迟函数
    let mut _delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    println!("load oled...");
    let (mut scl, mut sda) = init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 初始化 MPU6050
    let mut mpu_scl = gpiob.pb10.into_open_drain_output(&mut gpiob.crh);
    let mut mpu_sda = gpiob.pb11.into_open_drain_output(&mut gpiob.crh);
    mpu_sda.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    mpu_scl.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    mpu6050::mpu6050_init(&mut mpu_scl, &mut mpu_sda);

    let id = mpu6050::get_mpu6050_id(&mut mpu_scl, &mut mpu_sda);
    oled::show_string(&mut scl, &mut sda, 1, 1, "ID:");
    oled::show_hex_num(&mut scl, &mut sda, 1, 4, id as u32, 2);

    let mut data = mpu6050::MPU6050Data::default();
    loop {
        mpu6050::get_mpu6050_data(&mut mpu_scl, &mut mpu_sda, &mut data);

        oled::show_signed_num(&mut scl, &mut sda, 2, 1, data.acc_x as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 3, 1, data.acc_y as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 4, 1, data.acc_z as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 2, 8, data.gyro_x as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 3, 8, data.gyro_y as i32, 5);
        oled::show_signed_num(&mut scl, &mut sda, 4, 8, data.gyro_z as i32, 5);
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
    oled::init_oled_config(&mut scl, &mut sda);
    (scl, sda)
}
