//! 寄存器版本实现
#![allow(unused)]

use super::conf::*;
pub use super::AccelGyroData;

use embedded_hal::{
    digital::v2::{InputPin, OutputPin, StatefulOutputPin},
    prelude::_embedded_hal_blocking_delay_DelayUs,
};
use stm32f1xx_hal::{
    gpio::{self, OutputSpeed},
    timer::SysDelay,
};

const DEFAULT_SLAVE_ADDR: u8 = 0xD0;

/// MPU6050 芯片
pub struct Mpu6050<'a, Scl, Sda>
where
    Scl: OutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: InputPin + OutputPin,
    <Sda as InputPin>::Error: core::fmt::Debug,
{
    scl: &'a mut Scl,
    sda: &'a mut Sda,
    delay: &'a mut SysDelay,
}

impl<'a, Scl, Sda> Mpu6050<'a, Scl, Sda>
where
    Scl: OutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: InputPin + OutputPin,
    <Sda as InputPin>::Error: core::fmt::Debug,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    pub fn new(scl: &'a mut Scl, sda: &'a mut Sda, delay: &'a mut SysDelay) -> Self {
        Mpu6050 { scl, sda, delay }
    }

    fn i2c_w_scl(&mut self, bit_value: u8) {
        if bit_value == 0 {
            self.scl.set_low().unwrap();
        } else {
            self.scl.set_high().unwrap();
        }
        self.delay.delay_us(10_u32);
    }

    fn i2c_w_sda(&mut self, bit_value: u8) {
        if bit_value == 0 {
            self.sda.set_low().unwrap();
        } else {
            self.sda.set_high().unwrap();
        }
        self.delay.delay_us(10_u32);
    }

    fn i2c_r_sda(&mut self) -> u8 {
        let bit_value = self.sda.is_high().unwrap();
        self.delay.delay_us(10_u32);
        if bit_value {
            1
        } else {
            0
        }
    }

    /// 产生 I2C 协议起始信号
    pub fn i2c_start(&mut self) {
        self.i2c_w_sda(1);
        self.i2c_w_scl(1);
        self.i2c_w_sda(0);
        self.i2c_w_scl(0);
    }

    /// 产生 I2C 协议结束信号
    pub fn i2c_stop(&mut self) {
        self.i2c_w_sda(0);
        self.i2c_w_scl(1);
        self.i2c_w_sda(1);
    }

    /// 发送八位数据（不包含应答）
    pub fn i2c_send_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.i2c_w_sda(byte & (0x80 >> i));
            self.i2c_w_scl(1);
            self.i2c_w_scl(0);
        }
    }

    /// 读取八位数据（不包含应答）
    pub fn i2c_receive_byte(&mut self) -> u8 {
        self.i2c_w_sda(1);

        let mut byte = 0x00;
        for i in 0..8 {
            self.i2c_w_scl(1);
            if self.i2c_r_sda() == 1 {
                byte |= (0x80 >> i);
            }
            self.i2c_w_scl(0);
        }
        byte
    }

    /// 发送应答信号
    pub fn i2c_send_ack(&mut self, ack_bit: u8) {
        self.i2c_w_sda(ack_bit);
        self.i2c_w_scl(1);
        self.i2c_w_scl(0);
    }

    /// 接收应答信号
    pub fn i2c_receive_ack(&mut self) -> u8 {
        self.i2c_w_sda(1);
        self.i2c_w_scl(1);
        let ack_bit = self.i2c_r_sda();
        self.i2c_w_scl(0);
        ack_bit
    }

    /// I2C 初始化
    /// open-drain output pin 10,11
    pub fn init_i2c(&mut self) {
        self.i2c_w_scl(1);
        self.i2c_w_sda(1);
    }
}

impl<'a, Scl, Sda> Mpu6050<'a, Scl, Sda>
where
    Scl: OutputPin,
    <Scl as OutputPin>::Error: core::fmt::Debug,
    Sda: InputPin + OutputPin,
    <Sda as InputPin>::Error: core::fmt::Debug,
    <Sda as OutputPin>::Error: core::fmt::Debug,
{
    /// MPU6050 写寄存器函数
    /// reg_address：寄存器地址
    /// data：待写入寄存器值
    pub fn write_reg(&mut self, reg_address: u8, data: u8) {
        // 发送起始信号
        self.i2c_start();

        // 发送设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR);
        self.i2c_receive_ack();

        // 发送寄存器地址
        self.i2c_send_byte(reg_address);
        self.i2c_receive_ack();

        // 写数据到寄存器
        self.i2c_send_byte(data);
        self.i2c_receive_ack();

        self.i2c_stop();
    }

    /// 读取寄存器
    pub fn read_reg(&mut self, reg_address: u8) -> i16 {
        // 发送起始信号
        self.i2c_start();

        // 发送设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR);
        self.i2c_receive_ack();

        // 发送寄存器地址
        self.i2c_send_byte(reg_address);
        self.i2c_receive_ack();

        // 发送重复起始信号
        self.i2c_start();
        // 发送读模式设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR | 0x01);
        self.i2c_receive_ack();

        // 读寄存器数据
        let data = self.i2c_receive_byte();
        // 非应答信号
        self.i2c_send_ack(1);

        self.i2c_stop();

        data as i16
    }

    /// MPU6050 初始化
    /// ```rust
    /// let mut scl = gpiob.pb10.into_open_drain_output(&mut gpiob.crh);
    /// let mut sda = gpiob.pb11.into_open_drain_output(&mut gpiob.crh);
    /// sda.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    /// scl.set_speed(&mut gpiob.crh, gpio::IOPinSpeed::Mhz50);
    /// hardware::mpu6050::init_mpu6050(&mut scl, &mut sda);
    /// ```
    pub fn init_mpu6050(&mut self) {
        // I2C 初始化
        self.init_i2c();

        // 解除休眠状态
        self.write_reg(MPU6050_PWR_MGMT_1, 0x01);
        self.write_reg(MPU6050_PWR_MGMT_2, 0x00);
        // 陀螺仪采样率，典型值：0x07(125Hz)
        self.write_reg(MPU6050_SMPLRT_DIV, 0x09);
        // 低通滤波频率，典型值：0x06(5Hz)
        self.write_reg(MPU6050_CONFIG, 0x06);
        // 陀螺仪自检及测量范围，典型值：0x18(不自检，2000deg/s)
        self.write_reg(MPU6050_GYRO_CONFIG, 0x18);
        // 加速计自检、测量范围及高通滤波频率，典型值：0x01(不自检，2G，5Hz)
        self.write_reg(MPU6050_ACCEL_CONFIG, 0x18);
    }

    /// 获取 MPU6050 ID
    pub fn get_id(&mut self) -> u8 {
        self.read_reg(MPU6050_WHO_AM_I) as u8
    }

    /// 基本数据读取
    /// 连续读两个寄存器并合成 16 位数据
    pub fn get_data(&mut self) -> AccelGyroData {
        let mut data = AccelGyroData::default();

        let data_h = self.read_reg(MPU6050_ACCEL_XOUT_H);
        let data_l = self.read_reg(MPU6050_ACCEL_XOUT_L);
        data.acc_x = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_ACCEL_YOUT_H);
        let data_l = self.read_reg(MPU6050_ACCEL_YOUT_L);
        data.acc_y = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_ACCEL_ZOUT_H);
        let data_l = self.read_reg(MPU6050_ACCEL_ZOUT_L);
        data.acc_z = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_XOUT_H);
        let data_l = self.read_reg(MPU6050_GYRO_XOUT_L);
        data.gyro_x = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_YOUT_H);
        let data_l = self.read_reg(MPU6050_GYRO_YOUT_L);
        data.gyro_y = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_ZOUT_H);
        let data_l = self.read_reg(MPU6050_GYRO_ZOUT_L);
        data.gyro_z = (data_h << 8) | data_l;

        data
    }
}
