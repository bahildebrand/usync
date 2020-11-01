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

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use alloc_cortex_m::CortexMHeap;
use task::executor::Executor;
use timer::{enable_timer, sleep_ms, get_time_ms};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    hprintln!("Memory start: {}", start).unwrap();
    let size = 50 * 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    init_clocks();

    let mut executor = Executor::new();
    executor.spawn(task1());
    executor.spawn(task2());
    executor.spawn(sleep_task());
    executor.run();
}

fn init_clocks() {
    let sys_clk_mhz = 84;


    let periphs = Peripherals::take().unwrap();
    let rcc = periphs.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(sys_clk_mhz.mhz()).freeze();

    enable_timer(sys_clk_mhz * 1000 * 1000);
}

async fn task1() {
    hello_from_task1().await;
}

async fn task2() {
    hprintln!("Hello from task 2").unwrap();
}

async fn sleep_task() {
    hprintln!("Task start: {}", get_time_ms()).unwrap();
    sleep_ms(100).await;
    hprintln!("Task end: {}", get_time_ms()).unwrap();
}

async fn hello_from_task1() {
    hprintln!("Hello from task 1").unwrap();
}
