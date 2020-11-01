use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll},
};

/// Future that allows the executor to keep track of root level tasks.
pub(crate) struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Creates a new task with a unique task ID.
    pub(crate) fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    /// Polls the future to make progress on the overall task.
    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    /// Returns the unique ID for the given task.
    pub(crate) fn get_id(&self) -> TaskId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Unsigned 32-bit integer that that represenets a unique task ID.
pub(crate) struct TaskId(u32);

impl TaskId {
    /// Creates a new TaskID. All TaskIDs should be unique, as they are created
    /// from an atomic integer increment.
    fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}