#![no_std]
#![no_main]
#![feature(wake_trait)]
#![feature(default_alloc_error_handler)]

mod task;
extern crate alloc;

use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use alloc_cortex_m::CortexMHeap;
use task::executor::Executor;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    hprintln!("Memory start: {}", start).unwrap();
    let size = 10 * 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let mut executor = Executor::new();
    executor.spawn(task1());
    executor.spawn(task2());
    executor.run();
}

async fn task1() {
    hprintln!("task1").unwrap();
    hello_from_task1().await;
}

async fn task2() {
    hprintln!("task2").unwrap();
}

async fn hello_from_task1() {
    hprintln!("Hello from task 1").unwrap();
}
