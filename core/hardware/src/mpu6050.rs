//! 软件I2C读写MPU6050
//! MPU6050 是一个6轴姿态传感器，可以测量芯片自身X、Y、Z轴的加速度、角速度参数，
//! 通过数据融合，可进一步得到姿态角，常应用于平衡车、飞行器等需要检测自身姿态的场景。

#![allow(unused)]

use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
use stm32f1xx_hal::gpio::{self, OutputSpeed};

const MPU6050_ADDRESS: u8 = 0xD0;

const MPU6050_SMPLRT_DIV: u8 = 0x19;
const MPU6050_CONFIG: u8 = 0x1A;
const MPU6050_GYRO_CONFIG: u8 = 0x1B;
const MPU6050_ACCEL_CONFIG: u8 = 0x1C;

const MPU6050_ACCEL_XOUT_H: u8 = 0x3B;
const MPU6050_ACCEL_XOUT_L: u8 = 0x3C;
const MPU6050_ACCEL_YOUT_H: u8 = 0x3D;
const MPU6050_ACCEL_YOUT_L: u8 = 0x3E;
const MPU6050_ACCEL_ZOUT_H: u8 = 0x3F;
const MPU6050_ACCEL_ZOUT_L: u8 = 0x40;
const MPU6050_TEMP_OUT_H: u8 = 0x41;
const MPU6050_TEMP_OUT_L: u8 = 0x42;
const MPU6050_GYRO_XOUT_H: u8 = 0x43;
const MPU6050_GYRO_XOUT_L: u8 = 0x44;
const MPU6050_GYRO_YOUT_H: u8 = 0x45;
const MPU6050_GYRO_YOUT_L: u8 = 0x46;
const MPU6050_GYRO_ZOUT_H: u8 = 0x47;
const MPU6050_GYRO_ZOUT_L: u8 = 0x48;

const MPU6050_PWR_MGMT_1: u8 = 0x6B;
const MPU6050_PWR_MGMT_2: u8 = 0x6C;
const MPU6050_WHO_AM_I: u8 = 0x75;

pub fn i2c_r_sda<Sda>(sda: &mut Sda) -> bool
where
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    sda.is_set_high().unwrap()
}

/// I2C 开始
pub fn i2c_start<Scl, Sda>(scl: &mut Scl, sda: &mut Sda)
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    let _ = sda.set_high();
    let _ = scl.set_high();
    let _ = sda.set_low();
    let _ = scl.set_low();
}

/// I2C 结束
pub fn i2c_stop<Scl, Sda>(scl: &mut Scl, sda: &mut Sda)
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    let _ = sda.set_low();
    let _ = scl.set_high();
    let _ = sda.set_high();
}

/// I2C 发送字节
pub fn i2c_send_byte<Scl, Sda>(scl: &mut Scl, sda: &mut Sda, byte: u8)
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    for i in 0..8 {
        if byte & (0x80 >> i) > 0 {
            let _ = sda.set_high();
        } else {
            let _ = sda.set_low();
        }
        let _ = scl.set_high();
        let _ = scl.set_low();
    }
}

/// I2C 接收字节
pub fn i2c_receive_byte<Scl, Sda>(scl: &mut Scl, sda: &mut Sda) -> u8
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    let mut byte: u8 = 0x00;
    let _ = sda.set_high();

    for i in 0..8 {
        let _ = scl.set_high();

        if i2c_r_sda(sda) {
            byte |= 0x80 >> i;
        }

        let _ = scl.set_low();
    }
    byte
}

/// I2C 发送 ACK
pub fn i2c_send_ack<Scl, Sda>(scl: &mut Scl, sda: &mut Sda, ack_bit: u8)
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    if ack_bit == 0 {
        let _ = sda.set_low();
    } else {
        let _ = sda.set_high();
    }
    let _ = scl.set_high();
    let _ = scl.set_low();
}

/// I2C 接收 ACK
pub fn i2c_receive_ack<Scl, Sda>(scl: &mut Scl, sda: &mut Sda) -> bool
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    let _ = sda.set_high();
    let _ = scl.set_high();
    let ack_bit = i2c_r_sda(sda);
    let _ = scl.set_low();
    ack_bit
}

/// I2C 初始化
/// open-drain output pin 10,11
pub fn i2c_init<Scl, Sda>(scl: &mut Scl, sda: &mut Sda)
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    let _ = sda.set_high();
    let _ = scl.set_high();
}

/// 写入寄存器
pub fn mpu6050_write_reg<Scl, Sda>(scl: &mut Scl, sda: &mut Sda, reg_address: u8, data: u8)
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    i2c_start(scl, sda);
    i2c_send_byte(scl, sda, MPU6050_ADDRESS);
    i2c_receive_ack(scl, sda);
    i2c_send_byte(scl, sda, reg_address);
    i2c_receive_ack(scl, sda);
    i2c_send_byte(scl, sda, data);
    i2c_receive_ack(scl, sda);
    i2c_stop(scl, sda);
}

/// 读取寄存器
pub fn mpu6050_read_reg<Scl, Sda>(scl: &mut Scl, sda: &mut Sda, reg_address: u8) -> i16
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    i2c_start(scl, sda);
    i2c_send_byte(scl, sda, MPU6050_ADDRESS);
    i2c_receive_ack(scl, sda);
    i2c_send_byte(scl, sda, reg_address);
    i2c_receive_ack(scl, sda);

    i2c_start(scl, sda);
    i2c_send_byte(scl, sda, MPU6050_ADDRESS | 0x01);
    i2c_receive_ack(scl, sda);
    let data = i2c_receive_byte(scl, sda);
    i2c_send_ack(scl, sda, 1);
    i2c_stop(scl, sda);

    data as i16
}

/// MPU6050 初始化
/// ```rust
/// let mut scl = gpiob.pb10.into_open_drain_output(&mut gpiob.crh);
/// let mut sda = gpiob.pb11.into_open_drain_output(&mut gpiob.crh);
/// sda.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
/// scl.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
/// hardware::mpu6050::mpu6050_init(&mut scl, &mut sda);
/// ```
pub fn mpu6050_init<Scl, Sda>(scl: &mut Scl, sda: &mut Sda)
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    i2c_init(scl, sda);
    mpu6050_write_reg(scl, sda, MPU6050_PWR_MGMT_1, 0x01);
    mpu6050_write_reg(scl, sda, MPU6050_PWR_MGMT_2, 0x00);
    mpu6050_write_reg(scl, sda, MPU6050_SMPLRT_DIV, 0x09);
    mpu6050_write_reg(scl, sda, MPU6050_CONFIG, 0x06);
    mpu6050_write_reg(scl, sda, MPU6050_GYRO_CONFIG, 0x18);
    mpu6050_write_reg(scl, sda, MPU6050_ACCEL_CONFIG, 0x18);
}

/// 获取 MPU6050 ID
pub fn get_mpu6050_id<Scl, Sda>(scl: &mut Scl, sda: &mut Sda) -> i16
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    mpu6050_read_reg(scl, sda, MPU6050_WHO_AM_I)
}

/// 获取 MPU6050 轴数据
#[derive(Default)]
pub struct MPU6050Data {
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
}

/// 获取数据
pub fn get_mpu6050_data<Scl, Sda>(scl: &mut Scl, sda: &mut Sda, data: &mut MPU6050Data)
where
    Scl: OutputPin + StatefulOutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: OutputPin + StatefulOutputPin,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    let data_h = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_XOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_XOUT_L);
    data.acc_x = (data_h << 8) | data_l;

    let data_h = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_YOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_YOUT_L);
    data.acc_y = (data_h << 8) | data_l;

    let data_h = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_ZOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_ACCEL_ZOUT_L);
    data.acc_z = (data_h << 8) | data_l;

    let data_h = mpu6050_read_reg(scl, sda, MPU6050_GYRO_XOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_GYRO_XOUT_L);
    data.gyro_x = (data_h << 8) | data_l;

    let data_h = mpu6050_read_reg(scl, sda, MPU6050_GYRO_YOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_GYRO_YOUT_L);
    data.gyro_y = (data_h << 8) | data_l;

    let data_h = mpu6050_read_reg(scl, sda, MPU6050_GYRO_ZOUT_H);
    let data_l = mpu6050_read_reg(scl, sda, MPU6050_GYRO_ZOUT_L);
    data.gyro_z = (data_h << 8) | data_l;
}
