#![no_std]
// 标记该程序没有使用标准的 main 函数作为程序入口；
#![no_main]
// 允许有空的循环结构；
#![allow(clippy::empty_loop)]
// 禁止使用 Rust 的 unsafe 代码；
#![deny(unsafe_code)]

// 用于处理错误情况；
use panic_probe as _;
// global logger
use defmt_rtt as _;

// adjust HAL import
use stm32f1xx_hal as _; // memory layout

// 用于标记程序入口；
use cortex_m_rt::entry;

// 标记接下来的函数是程序的入口点；
// fn main() -> ! 定义程序的入口函数 main，返回类型为 `!`，表示这个函数不应该返回；
#[entry]
fn main() -> ! {
    // _y 的可变变量，但未初始化；
    let mut _y;
    // 定义变量 x，并将其值设置为 42；
    let x = 42;
    // 将变量 x 的值赋给 _y；
    _y = x;
    // 通过 OpenOCD 终端输出 `_y` 变量的值，使用十六进制表示；
    defmt::println!("_y={:?}", _y);
    defmt::println!("x={=u8}", x);

    // try setting the DEFMT_LOG environment variable
    // e.g. `export DEFMT_LOG=info` or `DEFMT_LOG=trace cargo rb levels`
    defmt::trace!("trace");
    defmt::debug!("debug");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::error!("error");

    // 进入一个空的无限循环，防止程序异常退出。
    loop {}
}
