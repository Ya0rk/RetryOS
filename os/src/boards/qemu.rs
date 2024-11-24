//! Constants used in rCore for qemu
use crate::config::mm::KERNEL_ADDR_OFFSET;

// 时钟设置
pub const CLOCK_FREQ: usize = 12500000;

// 内核设置
pub const MEMORY_END: usize = 0x8800_0000 + KERNEL_ADDR_OFFSET; // 内核物理空间结束地址

// 外设
pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
    (0x1000_1000, 0x00_1000), // Virtio Block in virt machine
];
