//! OLED 通用工具函数封装
#![allow(unused)]

use super::font::OLED_FONT;

use stm32f1xx_hal::gpio;

// 引脚类型别名配置
type OledWScl = gpio::PB8<gpio::Output<gpio::OpenDrain>>;
type OledWSda = gpio::PB9<gpio::Output<gpio::OpenDrain>>;

/// I2C 开始
pub fn i2c_start(scl: &mut OledWScl, sda: &mut OledWSda) {
    sda.set_high();
    scl.set_high();
    sda.set_low();
    scl.set_low();
}

/// I2C 停止
pub fn i2c_stop(scl: &mut OledWScl, sda: &mut OledWSda) {
    sda.set_low();
    scl.set_high();
    sda.set_high();
}

/// I2C发送一个字节
/// cbyte: 要发送的一个字节
fn i2c_send_byte(scl: &mut OledWScl, sda: &mut OledWSda, cbyte: u8) {
    for i in 0..8u8 {
        if cbyte & (0x80 >> i) == 0 {
            sda.set_low();
        } else {
            sda.set_high();
        }
        scl.set_high();
        scl.set_low();
    }
    scl.set_high(); //额外的一个时钟, 不处理应答信号
    scl.set_low();
}

/// OLED写命令
/// command: 要写入的命令
fn write_command(scl: &mut OledWScl, sda: &mut OledWSda, command: u8) {
    i2c_start(scl, sda);
    i2c_send_byte(scl, sda, 0x78); // 从机地址
    i2c_send_byte(scl, sda, 0x00); // 写命令
    i2c_send_byte(scl, sda, command);
    i2c_stop(scl, sda);
}

/// OLED写数据
/// data: 要写入的数据
fn write_data(scl: &mut OledWScl, sda: &mut OledWSda, data: u8) {
    i2c_start(scl, sda);
    i2c_send_byte(scl, sda, 0x78); // 从机地址
    i2c_send_byte(scl, sda, 0x40); // 写数据
    i2c_send_byte(scl, sda, data);
    i2c_stop(scl, sda);
}

/// OLED设置光标位置
/// y: 以左上角为原点, 向下方向的坐标, 范围: 0~7
/// x: 以左上角为原点, 向右方向的坐标, 范围: 0~127
fn set_cursor(scl: &mut OledWScl, sda: &mut OledWSda, y: u8, x: u8) {
    write_command(scl, sda, 0xB0 | y); // 设置y位置
    write_command(scl, sda, 0x10 | ((x & 0xF0) >> 4)); // 设置x位置高4位
    #[allow(clippy::identity_op)]
    write_command(scl, sda, 0x00 | (x & 0x0F)); // 设置x位置低4位
}

/// OLED清屏
pub fn clear(scl: &mut OledWScl, sda: &mut OledWSda) {
    for j in 0..8u8 {
        set_cursor(scl, sda, j, 0);
        for _i in 0..128u8 {
            write_data(scl, sda, 0x00);
        }
    }
}

/// OLED显示一个字符
/// line: 行位置，范围：1~4
/// column: 列位置，范围：1~16
/// cchar: 要显示的一个字符，范围：ASCII可见字符
pub fn show_char(scl: &mut OledWScl, sda: &mut OledWSda, line: u8, column: u8, cchar: char) {
    // 设置光标位置在上半部分
    set_cursor(scl, sda, (line - 1) * 2, (column - 1) * 8);
    for i in 0..8usize {
        // 显示上半部分内容
        write_data(scl, sda, OLED_FONT[cchar as usize - ' ' as usize][i]);
    }

    // 设置光标位置在下半部分
    set_cursor(scl, sda, (line - 1) * 2 + 1, (column - 1) * 8);

    for i in 0..8usize {
        // 显示下半部分内容
        write_data(
            scl,
            sda,
            OLED_FONT[cchar as usize - ' ' as usize][i + 8usize],
        );
    }
}

/// OLED显示字符串
/// line: 起始行位置，范围：1~4
/// column: 起始列位置，范围：1~16
/// string: 要显示的字符串，范围：ASCII可见字符
pub fn show_string(scl: &mut OledWScl, sda: &mut OledWSda, line: u8, column: u8, string: &str) {
    for (i, c) in string.chars().enumerate() {
        if c == '\0' {
            break;
        }
        show_char(scl, sda, line, column + i as u8, c);
    }
}

/// OLED次方函数
/// 返回值等于x的y次方
fn pow(x: u32, y: u32) -> u32 {
    let mut result = 1;
    let mut y_mut = y;
    while y_mut > 0 {
        result *= x;
        y_mut -= 1;
    }
    result
}

/// OLED显示数字（十进制, 正数）
/// line: 起始行位置, 范围: 1~4
/// column: 起始列位置, 范围: 1~16
/// number: 要显示的数字, 范围: 0~4294967295
/// length: 要显示数字的长度, 范围: 1~10
pub fn show_num(
    scl: &mut OledWScl,
    sda: &mut OledWSda,
    line: u8,
    column: u8,
    number: u32,
    length: u8,
) {
    for i in 0..length {
        let digit = number / pow(10, (length - i - 1).into()) % 10;
        let cchar = (digit as u8 + b'0') as char;
        show_char(scl, sda, line, column + i, cchar);
    }
}

/// OLED显示数字（十进制，带符号数）
/// line: 起始行位置，范围：1~4
/// column: 起始列位置，范围：1~16
/// number: 要显示的数字，范围：-2147483648~2147483647
/// length: 要显示数字的长度, 范围: 1~10
pub fn show_signed_num(
    scl: &mut OledWScl,
    sda: &mut OledWSda,
    line: u8,
    column: u8,
    number: i32,
    length: u8,
) {
    #[allow(unused)]
    let mut number1: i32 = 0;
    if number >= 0 {
        show_char(scl, sda, line, column, '+');
        number1 = number;
    } else {
        show_char(scl, sda, line, column, '-');
        number1 = -number;
    }

    for i in 0..length {
        let digit = number1 / pow(10, (length - i - 1).into()) as i32 % 10;
        let cchar = (digit as u8 + b'0') as char;
        show_char(scl, sda, line, column + i + 1, cchar);
    }
}

/// OLED显示数字（十六进制，正数）
/// line: 起始行位置，范围：1~4
/// column: 起始列位置，范围：1~16
/// number: 要显示的数字，范围：0~0xFFFFFFFF
/// length: 要显示数字的长度，范围：1~8
pub fn show_hex_num(
    scl: &mut OledWScl,
    sda: &mut OledWSda,
    line: u8,
    column: u8,
    number: u32,
    length: u8,
) {
    #[allow(unused)]
    let mut single_number = 0;
    for i in 0..length {
        single_number = number / pow(16, (length - i - 1).into()) % 16;
        if single_number < 10 {
            let cchar = (single_number as u8 + b'0') as char;
            show_char(scl, sda, line, column + i, cchar);
        } else {
            let cchar = (single_number as u8 - 10 + b'A') as char;
            show_char(scl, sda, line, column + i, cchar);
        }
    }
}

/// OLED显示数字（二进制，正数）
/// line: 起始行位置，范围：1~4
/// column: 起始列位置，范围：1~16
/// number: 要显示的数字，范围：0~1111 1111 1111 1111
/// length: 要显示数字的长度，范围：1~16
pub fn show_bin_num(
    scl: &mut OledWScl,
    sda: &mut OledWSda,
    line: u8,
    column: u8,
    number: u32,
    length: u8,
) {
    for i in 0..length {
        let digit = number / pow(2, (length - i - 1).into()) % 2;
        let cchar = (digit as u8 + b'0') as char;
        show_char(scl, sda, line, column + i, cchar);
    }
}

/// OLED配置初始化
/// 注意需要提前进行端口初始化
/// 注意上电延时
pub fn init_oled_config(scl: &mut OledWScl, sda: &mut OledWSda) {
    // OLED 状态初始化
    scl.set_high();
    sda.set_high();

    write_command(scl, sda, 0xAE); //关闭显示

    write_command(scl, sda, 0xD5); //设置显示时钟分频比/振荡器频率
    write_command(scl, sda, 0x80);

    write_command(scl, sda, 0xA8); //设置多路复用率
    write_command(scl, sda, 0x3F);

    write_command(scl, sda, 0xD3); //设置显示偏移
    write_command(scl, sda, 0x00);

    write_command(scl, sda, 0x40); //设置显示开始行

    write_command(scl, sda, 0xA1); //设置左右方向，0xA1正常 0xA0左右反置

    write_command(scl, sda, 0xC8); //设置上下方向，0xC8正常 0xC0上下反置

    write_command(scl, sda, 0xDA); //设置COM引脚硬件配置
    write_command(scl, sda, 0x12);

    write_command(scl, sda, 0x81); //设置对比度控制
    write_command(scl, sda, 0xCF);

    write_command(scl, sda, 0xD9); //设置预充电周期
    write_command(scl, sda, 0xF1);

    write_command(scl, sda, 0xDB); //设置VCOMH取消选择级别
    write_command(scl, sda, 0x30);

    write_command(scl, sda, 0xA4); //设置整个显示打开/关闭

    write_command(scl, sda, 0xA6); //设置正常/倒转显示

    write_command(scl, sda, 0x8D); //设置充电泵
    write_command(scl, sda, 0x14);

    write_command(scl, sda, 0xAF); //开启显示

    clear(scl, sda); //OLED清屏
}
