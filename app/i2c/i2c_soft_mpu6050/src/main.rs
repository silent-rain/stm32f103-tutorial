#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use hardware::{mpu6050::mpu6050_reg, oled};

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
    let mut delay = syst.delay(&clocks);

    // 初始化 OLED 显示屏
    let mut oled = oled::simple::init_oled(gpiob.pb8, gpiob.pb9, &mut gpiob.crh);

    // 初始化 MPU6050
    let mut mpu_scl = gpiob.pb10.into_open_drain_output(&mut gpiob.crh);
    let mut mpu_sda = gpiob.pb11.into_open_drain_output(&mut gpiob.crh);
    mpu_scl.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    mpu_sda.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    let mut mpu = mpu6050_reg::Mpu6050::new(&mut mpu_scl, &mut mpu_sda, &mut delay);
    mpu.init_mpu6050();

    let id = mpu.get_id();
    oled.show_string(1, 1, "ID:");
    oled.show_hex_num(1, 4, id as u32, 2);

    loop {
        let data = mpu.get_data();

        // 打印读取到的数据
        println!("Accel: ({}, {}, {})", data.acc_x, data.acc_y, data.acc_z);
        println!("Gyro: ({}, {}, {})", data.gyro_x, data.gyro_y, data.gyro_z);

        oled.show_signed_num(2, 1, data.acc_x as i32, 5);
        oled.show_signed_num(3, 1, data.acc_y as i32, 5);
        oled.show_signed_num(4, 1, data.acc_z as i32, 5);
        oled.show_signed_num(2, 8, data.gyro_x as i32, 5);
        oled.show_signed_num(3, 8, data.gyro_y as i32, 5);
        oled.show_signed_num(4, 8, data.gyro_z as i32, 5);
    }
}
