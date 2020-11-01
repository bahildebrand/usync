mod sleep;

use conquer_once::spin::OnceCell;
use cortex_m::{
    Peripherals,
    peripheral::syst::SystClkSource
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
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

pub struct Sleep {
    time_ms: u64
}

impl Sleep {
    pub fn new(duration_ms: u64) -> Sleep{
        Sleep {
            time_ms: get_time_ms() + duration_ms
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if get_time_ms() >= self.time_ms {
            Poll::Ready(())
        } else {
            let timer = TimerWaker::new(self.time_ms);
            timer.register_waker(&cx.waker());

            unsafe {
                let queue = TIMER_QUEUE
                        .try_get()
                        .expect("Queue not initialized");
                let _ = queue.push(timer);
            }
            Poll::Pending
        }
    }
}

pub fn get_time_ms() -> u64 {
    unsafe { COUNT.clone() }
}

pub fn sleep_ms(ms: u64) -> Sleep {
    Sleep::new(ms)
}

#[exception]
fn SysTick() {
    unsafe {
        COUNT += 1;

        let queue = TIMER_QUEUE
                    .try_get()
                    .expect("Queue not initialized");
        match queue.pop() {
            Some(timer) => {
                let _ = TIMER_HEAP.push(timer);
            },
            None => {}
        }

        while let Some(timer) = TIMER_HEAP.peek() {
            if timer.get_time() >= COUNT {
                timer.wake();
                let _ = TIMER_HEAP.pop();
            }
        }
    }
}
