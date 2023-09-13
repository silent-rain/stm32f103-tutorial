//! HAL 库版本实现
#![allow(unused)]

use super::conf::*;
pub use super::AccelGyroData;

use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
use embedded_hal::prelude::{
    _embedded_hal_blocking_i2c_Write, _embedded_hal_blocking_i2c_WriteRead,
};
use stm32f1xx_hal::gpio::{self, OutputSpeed};
use stm32f1xx_hal::i2c::{self, BlockingI2c, I2c, Pins};
use stm32f1xx_hal::pac::{self, I2C2};
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::rcc;

/// MPU6050 芯片
pub struct Mpu6050<PINS>
where
    PINS: i2c::Pins<pac::I2C2>,
{
    i2c: BlockingI2c<I2C2, PINS>,
}

impl<PINS> Mpu6050<PINS>
where
    PINS: i2c::Pins<pac::I2C2>,
{
    ///  初始化 MPU6050
    pub fn new(pins: PINS, i2c2: pac::I2C2, clocks: rcc::Clocks) -> Self
    where
        PINS: i2c::Pins<pac::I2C2>,
    {
        let mut i2c = BlockingI2c::i2c2(
            i2c2,
            pins,
            i2c::Mode::standard(10.kHz()),
            clocks,
            1000,
            10,
            1000,
            1000,
        );
        // 唤醒 mpu6050
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_PWR_MGMT_1, 0x01])
        //     .unwrap();
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_PWR_MGMT_2, 0x00])
        //     .unwrap();
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_SMPLRT_DIV, 0x09])
        //     .unwrap();
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_CONFIG, 0x06])
        //     .unwrap();
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_GYRO_CONFIG, 0x18])
        //     .unwrap();
        // i2c.write(DEFAULT_SLAVE_ADDR, &[MPU6050_ACCEL_CONFIG, 0x18])
        //     .unwrap();

        // i2c
        Mpu6050 { i2c }
    }

    /// 获取 MPU6050 ID
    pub fn get_id(&mut self) -> u8 {
        // 创建一个缓冲区用于存储数据
        let mut buffer: [u8; 14] = [0; 14];

        // 检查mpu6050的设备ID是否正确
        self.i2c
            .write_read(DEFAULT_SLAVE_ADDR, &[MPU6050_WHO_AM_I], &mut buffer[0..1])
            .unwrap();

        // assert_eq!(buffer[0], DEFAULT_SLAVE_ADDR);
        buffer[0]
    }

    /// 获取 MPU6050 数据
    /// 读取加速度和角速度数据
    pub fn get_data(&mut self) -> AccelGyroData {
        // 创建一个缓冲区用于存储数据
        let mut buffer: [u8; 14] = [0; 14];

        // 从mpu6050中读取14个字节的数据，包括加速度和角速度
        self.i2c
            .write_read(DEFAULT_SLAVE_ADDR, &[MPU6050_ACCEL_XOUT_H], &mut buffer)
            .unwrap();

        // 将数据转换为有符号的16位整数
        let acc_x = (buffer[0] as i16) << 8 | buffer[1] as i16;
        let acc_y = (buffer[2] as i16) << 8 | buffer[3] as i16;
        let acc_z = (buffer[4] as i16) << 8 | buffer[5] as i16;
        let gyro_x = (buffer[8] as i16) << 8 | buffer[9] as i16;
        let gyro_y = (buffer[10] as i16) << 8 | buffer[11] as i16;
        let gyro_z = (buffer[12] as i16) << 8 | buffer[13] as i16;

        AccelGyroData {
            acc_x,
            acc_y,
            acc_z,
            gyro_x,
            gyro_y,
            gyro_z,
        }
    }
}
