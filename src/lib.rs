#![no_std]
#![feature(wake_trait)]
#![feature(default_alloc_error_handler)]

pub mod timer;
pub mod task;

extern crate alloc;