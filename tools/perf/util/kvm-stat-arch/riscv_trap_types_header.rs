// SPDX-License-Identifier: GPL-2.0

/* Exception cause high bit - is an interrupt if set */
pub const fn CAUSE_IRQ_FLAG(xlen: usize) -> u64 {
    1_u64 << (xlen - 1)
}

/* Interrupt causes (minus the high bit) */
pub const IRQ_S_SOFT: u32 = 1;
pub const IRQ_VS_SOFT: u32 = 2;
pub const IRQ_M_SOFT: u32 = 3;
pub const IRQ_S_TIMER: u32 = 5;
pub const IRQ_VS_TIMER: u32 = 6;
pub const IRQ_M_TIMER: u32 = 7;
pub const IRQ_S_EXT: u32 = 9;
pub const IRQ_VS_EXT: u32 = 10;
pub const IRQ_M_EXT: u32 = 11;
pub const IRQ_S_GEXT: u32 = 12;
pub const IRQ_PMU_OVF: u32 = 13;

/* Exception causes */
pub const EXC_INST_MISALIGNED: u32 = 0;
pub const EXC_INST_ACCESS: u32 = 1;
pub const EXC_INST_ILLEGAL: u32 = 2;
pub const EXC_BREAKPOINT: u32 = 3;
pub const EXC_LOAD_MISALIGNED: u32 = 4;
pub const EXC_LOAD_ACCESS: u32 = 5;
pub const EXC_STORE_MISALIGNED: u32 = 6;
pub const EXC_STORE_ACCESS: u32 = 7;
pub const EXC_SYSCALL: u32 = 8;
pub const EXC_HYPERVISOR_SYSCALL: u32 = 9;
pub const EXC_SUPERVISOR_SYSCALL: u32 = 10;
pub const EXC_INST_PAGE_FAULT: u32 = 12;
pub const EXC_LOAD_PAGE_FAULT: u32 = 13;
pub const EXC_STORE_PAGE_FAULT: u32 = 15;
pub const EXC_INST_GUEST_PAGE_FAULT: u32 = 20;
pub const EXC_LOAD_GUEST_PAGE_FAULT: u32 = 21;
pub const EXC_VIRTUAL_INST_FAULT: u32 = 22;
pub const EXC_STORE_GUEST_PAGE_FAULT: u32 = 23;

macro_rules! TRAP {
    ($x:ident) => {
        ($x, stringify!($x))
    };
}

macro_rules! kvm_riscv_trap_class {
    () => {
        TRAP!(IRQ_S_SOFT),
        TRAP!(IRQ_VS_SOFT),
        TRAP!(IRQ_M_SOFT),
        TRAP!(IRQ_S_TIMER),
        TRAP!(IRQ_VS_TIMER),
        TRAP!(IRQ_M_TIMER),
        TRAP!(IRQ_S_EXT),
        TRAP!(IRQ_VS_EXT),
        TRAP!(IRQ_M_EXT),
        TRAP!(IRQ_S_GEXT),
        TRAP!(IRQ_PMU_OVF),
        TRAP!(EXC_INST_MISALIGNED),
        TRAP!(EXC_INST_ACCESS),
        TRAP!(EXC_INST_ILLEGAL),
        TRAP!(EXC_BREAKPOINT),
        TRAP!(EXC_LOAD_MISALIGNED),
        TRAP!(EXC_LOAD_ACCESS),
        TRAP!(EXC_STORE_MISALIGNED),
        TRAP!(EXC_STORE_ACCESS),
        TRAP!(EXC_SYSCALL),
        TRAP!(EXC_HYPERVISOR_SYSCALL),
        TRAP!(EXC_SUPERVISOR_SYSCALL),
        TRAP!(EXC_INST_PAGE_FAULT),
        TRAP!(EXC_LOAD_PAGE_FAULT),
        TRAP!(EXC_STORE_PAGE_FAULT),
        TRAP!(EXC_INST_GUEST_PAGE_FAULT),
        TRAP!(EXC_LOAD_GUEST_PAGE_FAULT),
        TRAP!(EXC_VIRTUAL_INST_FAULT),
        TRAP!(EXC_STORE_GUEST_PAGE_FAULT)
    };
}

pub(crate) use kvm_riscv_trap_class;
pub(crate) use TRAP;
