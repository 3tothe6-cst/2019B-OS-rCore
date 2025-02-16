use crate::consts::*;

global_asm!(include_str!("boot/entry64.asm"));
global_asm!(include_str!("link_user.S"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12,
    );
    crate::interrupt::init();
    crate::fs::init();
    crate::process::init();
    crate::timer::init();
    crate::process::run();
    loop {}
}
