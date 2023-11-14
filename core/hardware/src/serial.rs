//! 串行接口常用工具集

use core::u32;
use heapless::String;

use nb::block;
use stm32f1xx_hal::serial::{Instance, Rx, Tx};
use unwrap_infallible::UnwrapInfallible;

/// 发送字节
pub fn send_byte<USART>(tx: &mut Tx<USART>, word: u8)
where
    USART: Instance,
{
    // tx.write(word)
    block!(tx.write(word)).unwrap_infallible()
}

/// 发送字节数组
pub fn send_bytes<USART>(tx: &mut Tx<USART>, words: &[u8])
where
    USART: Instance,
{
    words.iter().for_each(|w| send_byte(tx, *w));
}

/// 发送字符串
pub fn send_string<USART>(tx: &mut Tx<USART>, words: &str)
where
    USART: Instance,
{
    for word in words.as_bytes() {
        if *word == b'\0' {
            break;
        }
        send_byte(tx, *word);
    }
}

/// 发送数字
pub fn send_number<USART>(tx: &mut Tx<USART>, number: u32)
where
    USART: Instance,
{
    let mut length = 0;
    loop {
        length += 1;
        let rounding = number / (10_u32.pow(length));
        if rounding == 0 {
            break;
        }
    }

    for i in 0..length {
        let v = number / 10_u32.pow(length - i - 1) % 10 + '0' as u32;
        send_byte(tx, v as u8);
    }
}

/// 接收字节
pub fn recv_byte<USART>(rx: &mut Rx<USART>) -> u8
where
    USART: Instance,
{
    block!(rx.read()).unwrap()
}

/// 接收字节数组
/// 最大长度: 4096
pub fn recv_bytes<USART>(rx: &mut Rx<USART>, buffer: &mut [u8])
where
    USART: Instance,
{
    let mut widx: usize = 0;
    loop {
        let w = block!(rx.read()).unwrap();
        if w == b'\n' {
            break;
        }
        if widx < buffer.len() {
            buffer[widx] = w;
            widx += 1;
        }
    }
}

/// 接收字符串
/// 最大长度: 4096
pub fn recv_string<USART>(rx: &mut Rx<USART>) -> String<4096>
where
    USART: Instance,
{
    let mut s: String<4096> = String::new();

    loop {
        let w = block!(rx.read()).unwrap();
        if w == b'\n' {
            break;
        }
        s.push(w as char).unwrap();
    }
    s
}
