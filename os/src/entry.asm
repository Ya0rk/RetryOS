    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top

    # since the base addr is 0xffff_ffc0_8020_0000
    # we need to activate pagetable here in case of absolute addressing
    # satp: 8 << 60 | boot_pagetable
    la t0, boot_pagetable
    li t1, 8 << 60
    srli t0, t0, 12
    or t0, t0, t1
    csrw satp, t0 # 开启页表机制，现在开始都是虚拟地址
    sfence.vma    # clear TLB

    # 在跳板函数里call rust_main
    call jump_helper

    # call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    
    .space 4096 * 16 # 此时只有一个cpu，todo(在多核阶段修改)

    .globl boot_stack_top
boot_stack_top:

# 实现的大页表
    .section .data
    .align 12
boot_pagetable:
    # we need 2 pte here
    # 0x0000_0000_8000_0000 -> 0x0000_0000_8000_0000
    # 0xffff_fc00_8000_0000 -> 0x0000_0000_8000_0000
    .quad 0
    .quad 0
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 255
    .quad (0x80000 << 10) | 0xcf # VRWXAD
    .zero 8 * 253