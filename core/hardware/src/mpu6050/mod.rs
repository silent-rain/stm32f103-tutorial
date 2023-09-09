//! 软件I2C读写MPU6050
//! MPU6050 是一个6轴姿态传感器，可以测量芯片自身X、Y、Z轴的加速度、角速度参数，
//! 通过数据融合，可进一步得到姿态角，常应用于平衡车、飞行器等需要检测自身姿态的场景。
pub mod conf;
pub mod mpu6050_hal;
pub mod mpu6050_reg;

/// 加速度和角速度数据
#[derive(Default)]
pub struct AccelGyroData {
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
}
