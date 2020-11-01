use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc};
use core::{
    future::Future,
    task::{Context, Poll, Waker}
};
use crossbeam_queue::ArrayQueue;
use cortex_m::{interrupt, asm};
use super::waker::TaskWaker;

/// Exectuor for all spawned tasks.
///
/// # Examples
///
/// Creating an executor and running a simple task:
/// ```
/// use task::executor::Executor;
///
/// let mut executor = Executor::new();
/// executor.run();
/// ```
pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    /// Creates a new executor. Currently there is only support for single
    /// threaded environments.
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Spawns a task from a provided future. The future must not have an output
    /// type as this is intended to be a top level task.
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(future);
        let task_id = task.get_id();
        if self.tasks.insert(task_id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    /// Executes all tasks previously spawned. This function will loop through
    /// all tasks and attempt to sleep if there is no work to be done.
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    #[doc(hidden)]
    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    #[doc(hidden)]
    fn sleep_if_idle(&self) {
        interrupt::disable();
        if self.task_queue.is_empty() {
            asm::wfe();
            unsafe { interrupt::enable(); }
        } else {
            unsafe { interrupt::enable(); }
        }
    }
}
