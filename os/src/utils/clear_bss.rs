/// clear bss before boot
use core::arch::global_asm;

global_asm!(include_str!("../entry.asm"));
global_asm!(include_str!("../link_app.S"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}