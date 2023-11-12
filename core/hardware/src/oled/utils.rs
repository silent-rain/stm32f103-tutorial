//! OLED 通用工具函数封装
use super::font::OLED_FONT;

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::gpio::{OpenDrain, Output, PB8, PB9};

pub struct OLED<Scl, Sda>
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    scl: Scl,
    sda: Sda,
}

impl<Scl, Sda> OLED<Scl, Sda>
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    /// 初始化 OLED 配置
    /// 注意需要提前进行端口初始化
    /// 注意上电延时
    pub fn new(scl: Scl, sda: Sda) -> Self {
        let mut oled = OLED { scl, sda };
        oled.init();
        oled
    }

    /// 初始化配置
    pub fn init(&mut self) {
        // OLED 状态初始化
        self.scl.set_high();
        self.sda.set_high();

        self.write_command(0xAE); //关闭显示

        self.write_command(0xD5); //设置显示时钟分频比/振荡器频率
        self.write_command(0x80);

        self.write_command(0xA8); //设置多路复用率
        self.write_command(0x3F);

        self.write_command(0xD3); //设置显示偏移
        self.write_command(0x00);

        self.write_command(0x40); //设置显示开始行

        self.write_command(0xA1); //设置左右方向，0xA1正常 0xA0左右反置

        self.write_command(0xC8); //设置上下方向，0xC8正常 0xC0上下反置

        self.write_command(0xDA); //设置COM引脚硬件配置
        self.write_command(0x12);

        self.write_command(0x81); //设置对比度控制
        self.write_command(0xCF);

        self.write_command(0xD9); //设置预充电周期
        self.write_command(0xF1);

        self.write_command(0xDB); //设置VCOMH取消选择级别
        self.write_command(0x30);

        self.write_command(0xA4); //设置整个显示打开/关闭

        self.write_command(0xA6); //设置正常/倒转显示

        self.write_command(0x8D); //设置充电泵
        self.write_command(0x14);

        self.write_command(0xAF); //开启显示

        self.clear(); //OLED清屏
    }

    /// I2C 开始
    pub fn i2c_start(&mut self) {
        self.sda.set_high();
        self.scl.set_high();
        self.sda.set_low();
        self.scl.set_low();
    }

    /// I2C 停止
    fn i2c_stop(&mut self) {
        self.sda.set_low();
        self.scl.set_high();
        self.sda.set_high();
    }

    /// I2C发送一个字节
    /// cbyte: 要发送的一个字节
    fn i2c_send_byte(&mut self, cbyte: u8) {
        for i in 0..8u8 {
            if cbyte & (0x80 >> i) == 0 {
                self.sda.set_low();
            } else {
                self.sda.set_high();
            }
            self.scl.set_high();
            self.scl.set_low();
        }
        self.scl.set_high(); //额外的一个时钟, 不处理应答信号
        self.scl.set_low();
    }

    /// OLED写命令
    /// command: 要写入的命令
    fn write_command(&mut self, command: u8) {
        self.i2c_start();
        self.i2c_send_byte(0x78); // 从机地址
        self.i2c_send_byte(0x00); // 写命令
        self.i2c_send_byte(command);
        self.i2c_stop();
    }

    /// OLED写数据
    /// data: 要写入的数据
    fn write_data(&mut self, data: u8) {
        self.i2c_start();
        self.i2c_send_byte(0x78); // 从机地址
        self.i2c_send_byte(0x40); // 写数据
        self.i2c_send_byte(data);
        self.i2c_stop();
    }
}

impl<Scl, Sda> OLED<Scl, Sda>
where
    Scl: OutputPin,
    Sda: OutputPin,
{
    /// OLED设置光标位置
    /// y: 以左上角为原点, 向下方向的坐标, 范围: 0~7
    /// x: 以左上角为原点, 向右方向的坐标, 范围: 0~127
    fn set_cursor(&mut self, y: u8, x: u8) {
        self.write_command(0xB0 | y); // 设置y位置
        self.write_command(0x10 | ((x & 0xF0) >> 4)); // 设置x位置高4位
        #[allow(clippy::identity_op)]
        self.write_command(0x00 | (x & 0x0F)); // 设置x位置低4位
    }

    /// OLED清屏
    pub fn clear(&mut self) {
        for j in 0..8u8 {
            self.set_cursor(j, 0);
            for _i in 0..128u8 {
                self.write_data(0x00);
            }
        }
    }

    /// OLED显示一个字符
    /// line: 行位置，范围：1~4
    /// column: 列位置，范围：1~16
    /// cchar: 要显示的一个字符，范围：ASCII可见字符
    pub fn show_char(&mut self, line: u8, column: u8, cchar: char) {
        // 设置光标位置在上半部分
        self.set_cursor((line - 1) * 2, (column - 1) * 8);
        for i in 0..8usize {
            // 显示上半部分内容
            self.write_data(OLED_FONT[cchar as usize - ' ' as usize][i]);
        }

        // 设置光标位置在下半部分
        self.set_cursor((line - 1) * 2 + 1, (column - 1) * 8);

        for i in 0..8usize {
            // 显示下半部分内容
            self.write_data(OLED_FONT[cchar as usize - ' ' as usize][i + 8usize]);
        }
    }

    /// OLED显示字符串
    /// line: 起始行位置，范围：1~4
    /// column: 起始列位置，范围：1~16
    /// string: 要显示的字符串，范围：ASCII可见字符
    pub fn show_string(&mut self, line: u8, column: u8, string: &str) {
        for (i, c) in string.chars().enumerate() {
            if c == '\0' {
                break;
            }
            self.show_char(line, column + i as u8, c);
        }
    }

    /// OLED次方函数
    /// 返回值等于x的y次方
    fn pow(&self, x: u32, y: u32) -> u32 {
        let mut result = 1;
        let mut y_mut = y;
        while y_mut > 0 {
            result *= x;
            y_mut -= 1;
        }
        result
    }

    /// OLED显示数字（十进制, 正数）
    /// line: 起始行位置, 范围: 1-4
    /// column: 起始列位置, 范围: 1-16
    /// number: 要显示的数字, 范围: 0-4294967295
    /// length: 要显示数字的长度, 范围: 1-10
    pub fn show_num(&mut self, line: u8, column: u8, number: u32, length: u8) {
        for i in 0..length {
            let digit = number / self.pow(10, (length - i - 1).into()) % 10;
            let cchar = (digit as u8 + b'0') as char;
            self.show_char(line, column + i, cchar);
        }
    }

    /// OLED显示数字（十进制，带符号数）
    /// line: 起始行位置，范围：1~4
    /// column: 起始列位置，范围：1~16
    /// number: 要显示的数字，范围：-2147483648~2147483647
    /// length: 要显示数字的长度, 范围: 1~10
    pub fn show_signed_num(&mut self, line: u8, column: u8, number: i32, length: u8) {
        #[allow(unused)]
        let mut number1: i32 = 0;
        if number >= 0 {
            self.show_char(line, column, '+');
            number1 = number;
        } else {
            self.show_char(line, column, '-');
            number1 = -number;
        }

        for i in 0..length {
            let digit = number1 / self.pow(10, (length - i - 1).into()) as i32 % 10;
            let cchar = (digit as u8 + b'0') as char;
            self.show_char(line, column + i + 1, cchar);
        }
    }

    /// OLED显示数字（十六进制，正数）
    /// line: 起始行位置，范围：1~4
    /// column: 起始列位置，范围：1~16
    /// number: 要显示的数字，范围：0~0xFFFFFFFF
    /// length: 要显示数字的长度，范围：1~8
    pub fn show_hex_num(&mut self, line: u8, column: u8, number: u32, length: u8) {
        #[allow(unused)]
        let mut single_number = 0;
        for i in 0..length {
            single_number = number / self.pow(16, (length - i - 1).into()) % 16;
            if single_number < 10 {
                let cchar = (single_number as u8 + b'0') as char;
                self.show_char(line, column + i, cchar);
            } else {
                let cchar = (single_number as u8 - 10 + b'A') as char;
                self.show_char(line, column + i, cchar);
            }
        }
    }

    /// OLED显示数字（二进制，正数）
    /// line: 起始行位置，范围：1~4
    /// column: 起始列位置，范围：1~16
    /// number: 要显示的数字，范围：0~1111 1111 1111 1111
    /// length: 要显示数字的长度，范围：1~16
    pub fn show_bin_num(&mut self, line: u8, column: u8, number: u32, length: u8)
    where
        Scl: OutputPin,
        Sda: OutputPin,
    {
        for i in 0..length {
            let digit = number / self.pow(2, (length - i - 1).into()) % 2;
            let cchar = (digit as u8 + b'0') as char;
            self.show_char(line, column + i, cchar);
        }
    }
}
