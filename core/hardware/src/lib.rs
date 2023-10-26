#![no_std]
#![no_main]

use panic_probe as _;

pub mod flash_store;
pub mod key;
pub mod mpu6050;
pub mod oled;
pub mod serial;
pub mod syst;
pub mod w25q64;
