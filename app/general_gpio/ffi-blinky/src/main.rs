#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal as _;

use stm32f10x_rs::{
    BitAction, Delay_ms, FunctionalState, GPIOMode_TypeDef, GPIOSpeed_TypeDef, GPIO_Init,
    GPIO_InitTypeDef, GPIO_ResetBits, GPIO_SetBits, GPIO_TypeDef, GPIO_WriteBit,
    RCC_APB2PeriphClockCmd,
};

// #define GPIO_Pin_0                 ((uint16_t)0x0001)  /*!< Pin 0 selected */
#[allow(non_upper_case_globals)]
const GPIO_Pin_0: u16 = 0x0001;

// #define RCC_APB2Periph_GPIOA             ((uint32_t)0x00000004)
#[allow(non_upper_case_globals)]
const RCC_APB2Periph_GPIOA: u32 = 0x00000004;

// #define PERIPH_BASE           ((uint32_t)0x40000000) /*!< Peripheral base address in the alias region */
// #define APB2PERIPH_BASE       (PERIPH_BASE + 0x10000)
// #define GPIOA_BASE            (APB2PERIPH_BASE + 0x0800)
// #define GPIOA               ((GPIO_TypeDef *) GPIOA_BASE)
const PERIPH_BASE: u32 = 0x40000000; // 1073741824 -> 0x40000000
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000; // 1073807360 -> 0x40010000
const GPIOA_BASE: u32 = APB2PERIPH_BASE + 0x0800; // 1073809408 -> 0x40010800
const GPIOA: *mut GPIO_TypeDef = (GPIOA_BASE as *mut u32).cast(); // 1073809408 -> 0x40010800

#[entry]
fn main() -> ! {
    unsafe {
        println!("APB2PERIPH_BASE {:?}", APB2PERIPH_BASE); // 1073807360 -> 0x40010000
        println!("GPIOA_BASE {:?}", GPIOA_BASE); // 1073809408 -> 0x40010800
        println!("GPIOA {:?}", GPIOA as *const _ as u32); // 1073809408 -> 0x8002ef4

        println!("RCC_APB2PeriphClockCmd");
        RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, FunctionalState::ENABLE);

        println!("GPIO_Init");
        let mut gpio_init_structure = GPIO_InitTypeDef {
            GPIO_Mode: GPIOMode_TypeDef::GPIO_Mode_Out_PP,
            GPIO_Pin: GPIO_Pin_0,
            GPIO_Speed: GPIOSpeed_TypeDef::GPIO_Speed_50MHz,
        };
        GPIO_Init(GPIOA, &mut gpio_init_structure);

        println!("loop");
        loop {
            println!("GPIO_ResetBits");
            GPIO_ResetBits(GPIOA, GPIO_Pin_0);
            Delay_ms(500);
            println!("GPIO_SetBits");
            GPIO_SetBits(GPIOA, GPIO_Pin_0);
            Delay_ms(500);

            println!("GPIO_WriteBit Bit_RESET");
            GPIO_WriteBit(GPIOA, GPIO_Pin_0, BitAction::Bit_RESET);
            Delay_ms(500);
            println!("GPIO_WriteBit Bit_SET");
            GPIO_WriteBit(GPIOA, GPIO_Pin_0, BitAction::Bit_SET);
            Delay_ms(500);
        }
    }
}
