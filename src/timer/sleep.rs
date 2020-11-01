use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};
use super::{timer_queue_push, get_time_ms};
use crate::task::waker::TimerWaker;

/// Future that allows a task to yield without busy waiting.
pub struct Sleep {
    time_ms: u64
}

impl Sleep {
    /// Creates a new sleep future that sleeps for the given amount of time.
    ///
    /// # Arguements
    ///
    /// * `duration_ms` - The time in ms that the task needs to yield.
    pub fn new(duration_ms: u64) -> Sleep{
        Sleep {
            time_ms: get_time_ms() + duration_ms
        }
    }
}

impl Future for Sleep {
    type Output = ();

    /// Advances the sleep future by checking the current time in ms, and
    /// registering a waker that will be checked by the systick exception.
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

/// Sleeps for the given amount of time in ms._
///
/// # Arguments
///
/// * `ms` - Amount of time to sleep in ms.
pub fn sleep_ms(ms: u64) -> Sleep {
    Sleep::new(ms)
}