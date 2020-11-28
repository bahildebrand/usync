#![no_std]
#![feature(wake_trait)]
#![feature(default_alloc_error_handler)]

pub mod task;
pub mod timer;

extern crate alloc;