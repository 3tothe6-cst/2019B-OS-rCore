#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(const_in_array_repeat_expressions)]

extern crate alloc;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod io;

mod consts;
mod context;
mod fs;
mod init;
mod interrupt;
mod lang_items;
mod memory;
mod process;
mod sbi;
mod sync;
mod syscall;
mod timer;
