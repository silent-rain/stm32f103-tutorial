//! Prints "Hello, world" on the OpenOCD console
#![no_std]
// 标记该程序没有使用标准的 main 函数作为程序入口；
#![no_main]
// 允许有空的循环结构；
#![allow(clippy::empty_loop)]
// 禁止使用 Rust 的 unsafe 代码；
#![deny(unsafe_code)]

// 用于处理错误情况；
use panic_semihosting as _;

// 便于程序调试；
use cortex_m_semihosting::debug;
// 用于在 OpenOCD 终端上输出信息；
use cortex_m_semihosting::hprintln;
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
    hprintln!("y: {:#?}", _y);
    // 通过 OpenOCD 终端输出 "Hello, world!"；
    hprintln!("Hello, world!");

    // 退出 Renode
    // 执行到此处时通知 OpenOCD 结束程序，并返回正常退出状态。
    // 注意不要在实际硬件上运行这一行代码，它会导致 OpenOCD 失败；
    debug::exit(debug::EXIT_SUCCESS);

    // 进入一个空的无限循环，防止程序异常退出。
    loop {}
}
