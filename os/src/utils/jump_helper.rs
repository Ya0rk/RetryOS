/// 跳板函数
/// ADD KERNEL_ADDR_OFFSET and jump to rust_main
use crate::config::mm::KERNEL_ADDR_OFFSET;
use core::arch::asm;

/// 间接跳转到rust_main
#[no_mangle]
pub fn jump_helper() {
    unsafe {
        asm!("add sp, sp, {}", in(reg) KERNEL_ADDR_OFFSET);
        asm!("la t0, rust_main");
        asm!("add t0, t0, {}", in(reg) KERNEL_ADDR_OFFSET);
        // asm!("mv a0, {}", in(reg) hart_id);
        asm!("jalr zero, 0(t0)");
    }
}