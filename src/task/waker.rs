use alloc::{
    sync::Arc,
    task::Wake
};
use futures_util::task::AtomicWaker;
use core::{
    cmp::Ordering,
    task::Waker
};
use crossbeam_queue::ArrayQueue;
use super::TaskId;

pub(crate) struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    pub(crate) fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    pub(crate) fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

pub(crate) struct TimerWaker {
    pub(crate) ms: u64,
    waker: AtomicWaker
}

impl TimerWaker {
    pub(crate) fn new(ms: u64) -> TimerWaker {
        TimerWaker {
            ms: ms,
            waker: AtomicWaker::new()
        }
    }

    pub(crate) fn register_waker(&self, waker: &Waker) {
        self.waker.register(waker);
    }

    pub(crate) fn wake(&self) {
        self.waker.wake();
    }

    pub(crate) fn get_time(&self) -> u64 {
        self.ms
    }
}

impl Eq for TimerWaker {}

impl PartialEq for TimerWaker {
    fn eq(&self, other: &Self) -> bool {
        self.ms == other.ms
    }
}

impl Ord for TimerWaker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ms.cmp(&other.ms)
    }
}

impl PartialOrd for TimerWaker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}