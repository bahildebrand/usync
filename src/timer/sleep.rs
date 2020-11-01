use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};
use super::{timer_queue_push, get_time_ms};
use crate::task::waker::TimerWaker;

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

            timer_queue_push(timer);
            Poll::Pending
        }
    }
}

pub fn sleep_ms(ms: u64) -> Sleep {
    Sleep::new(ms)
}