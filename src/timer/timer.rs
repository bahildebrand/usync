use conquer_once::spin::OnceCell;
use cortex_m::{
    Peripherals,
    peripheral::syst::SystClkSource
};

use cortex_m_rt::exception;
use crossbeam_queue::ArrayQueue;
use heapless::binary_heap::{BinaryHeap, Min};
use heapless::consts::*;

use crate::task::waker::TimerWaker;

static mut COUNT: u64 = 0;
static mut TIMER_HEAP: BinaryHeap<TimerWaker, U8, Min> = BinaryHeap(
        heapless::i::BinaryHeap::new()
    );
static mut TIMER_QUEUE: OnceCell<ArrayQueue<TimerWaker>> = OnceCell::uninit();

pub fn enable_timer(hertz: u32) {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;
    unsafe {
        TIMER_QUEUE
                .try_init_once(|| ArrayQueue::new(20))
                .expect("Timer should only be initialized once");
    }

    syst.set_clock_source(SystClkSource::Core);

    let ticks_per_ms = (hertz / 1000) - 1;
    syst.set_reload(ticks_per_ms);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

pub(crate) fn timer_queue_push(timer: TimerWaker) {
    unsafe {
        let queue = TIMER_QUEUE
                .try_get()
                .expect("Queue not initialized");
        let _ = queue.push(timer);
    }
}

pub fn get_time_ms() -> u64 {
    unsafe { COUNT.clone() }
}

#[exception]
fn SysTick() {
    unsafe {
        COUNT += 1;

        let queue = TIMER_QUEUE
                    .try_get()
                    .expect("Queue not initialized");
        while let Some(timer) = queue.pop() {
            let _ = TIMER_HEAP.push(timer);
        }

        while let Some(timer) = TIMER_HEAP.peek() {
            if timer.get_time() >= COUNT {
                timer.wake();
                let _ = TIMER_HEAP.pop();
            }
        }
    }
}