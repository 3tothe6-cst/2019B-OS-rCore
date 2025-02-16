use riscv::register::{sie, time};

use crate::sbi::set_timer;

pub static mut TICKS: usize = 0;

pub(crate) static TIMEBASE: u64 = 100000;
pub fn init() {
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}

pub fn get_cycle() -> u64 {
    time::read() as u64
}
