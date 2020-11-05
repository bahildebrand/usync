#![no_std]
#![no_main]
#![feature(wake_trait)]
#![feature(default_alloc_error_handler)]

mod task;
mod timer;

extern crate alloc;

use stm32f4xx_hal as hal;
use crate::hal::{
    prelude::*,
    stm32::Peripherals
};

use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use alloc_cortex_m::CortexMHeap;
use task::executor::Executor;
use timer::{
    enable_timer,
    get_time_ms,
    sleep::sleep_ms
};
use stm32f4xx_hal::{stm32::USART2, serial::{Serial, config::Config, Tx}};
use core::fmt::Write; // for pretty formatting of the serial output

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    hprintln!("Memory start: {}", start).unwrap();
    let size = 50 * 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let tx = init_usart2();

    let mut executor = Executor::new();
    executor.spawn(sleep_task(tx));
    executor.run();
}

fn init_usart2() -> Tx<USART2> {
    let periphs = Peripherals::take().unwrap();
    let sys_clk_mhz = 168;

    let rcc = periphs.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(sys_clk_mhz.mhz()).freeze();

    enable_timer(sys_clk_mhz * 1000 * 1000);

    let gpioa = periphs.GPIOA.split();
    let tx_pin = gpioa.pa2.into_alternate_af7();
    let rx_pin = gpioa.pa3.into_alternate_af7();

    // configure serial
    let serial = Serial::usart2(
        periphs.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(115200.bps()),
        clocks,
    )
    .unwrap();

    let (tx, mut _rx) = serial.split();

    tx
}

async fn sleep_task(mut tx: Tx<USART2>) {
    writeln!(tx, "Task1 start: {}\r", get_time_ms()).unwrap();
    sleep_ms(50).await;
    writeln!(tx, "Task1 end: {}\r", get_time_ms()).unwrap();
    sleep_ms(100).await;
    writeln!(tx, "Task1 end: {}\r", get_time_ms()).unwrap();
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("Hardfault: {:?}", ef).unwrap();

    loop {}
}

#[exception]
fn DefaultHandler(irqn: i16) {
    hprintln!("Interrupt fired: {}", irqn).unwrap();
}
