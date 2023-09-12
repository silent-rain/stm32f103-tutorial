//!配置
pub const DEFAULT_SLAVE_ADDR: u8 = 0x68;

// 采样率分频，典型值：0x07(125Hz)
pub const MPU6050_SMPLRT_DIV: u8 = 0x19;
// 低通滤波频率，典型值：0x06(5Hz)
pub const MPU6050_CONFIG: u8 = 0x1A;
// 陀螺仪自检及测量范围，典型值：0x18(不自检，2000deg/s)
pub const MPU6050_GYRO_CONFIG: u8 = 0x1B;
// 加速计自检、测量范围及高通滤波频率，典型值：0x01(不自检，2G，5Hz)
pub const MPU6050_ACCEL_CONFIG: u8 = 0x1C;

// 存储最近的X轴、Y轴、Z轴加速度感应器的测量值
pub const MPU6050_ACCEL_XOUT_H: u8 = 0x3B;
pub const MPU6050_ACCEL_XOUT_L: u8 = 0x3C;
pub const MPU6050_ACCEL_YOUT_H: u8 = 0x3D;
pub const MPU6050_ACCEL_YOUT_L: u8 = 0x3E;
pub const MPU6050_ACCEL_ZOUT_H: u8 = 0x3F;
pub const MPU6050_ACCEL_ZOUT_L: u8 = 0x40;

// 存储的最近温度传感器的测量值
pub const MPU6050_TEMP_OUT_H: u8 = 0x41;
pub const MPU6050_TEMP_OUT_L: u8 = 0x42;

// 存储最近的X轴、Y轴、Z轴陀螺仪感应器的测量值
pub const MPU6050_GYRO_XOUT_H: u8 = 0x43;
pub const MPU6050_GYRO_XOUT_L: u8 = 0x44;
pub const MPU6050_GYRO_YOUT_H: u8 = 0x45;
pub const MPU6050_GYRO_YOUT_L: u8 = 0x46;
pub const MPU6050_GYRO_ZOUT_H: u8 = 0x47;
pub const MPU6050_GYRO_ZOUT_L: u8 = 0x48;

// 电源管理，典型值：0x00(正常启用)
pub const MPU6050_PWR_MGMT_1: u8 = 0x6B;
pub const MPU6050_PWR_MGMT_2: u8 = 0x6C;
// IIC地址寄存器(默认数值0x68，只读)
pub const MPU6050_WHO_AM_I: u8 = 0x75;
