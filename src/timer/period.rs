use core::{
    pin::Pin,
    task::{Context, Poll}
};
use futures_util::stream::StreamExt;
use futures_core::stream::Stream;
use super::{timer_queue_push, get_time_ms};
use crate::task::waker::TimerWaker;


pub struct Period {
    period_ms: u64,
    next_start_ms: u64
}

impl Period {
    pub fn new(period_ms: u64) -> Period {
        Period {
            period_ms: period_ms,
            next_start_ms: get_time_ms()
        }
    }

    pub async fn next_period(&mut self) {
        self.next().await;
    }
}

impl Stream for Period {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<Option<Self::Item>> {
        if get_time_ms() >= self.next_start_ms {
            self.next_start_ms += self.period_ms;

            Poll::Ready(Some(get_time_ms()))
        } else {
            let timer = TimerWaker::new(self.next_start_ms);
            timer.register_waker(&cx.waker());

            timer_queue_push(timer);
            Poll::Pending
        }
    }
}
