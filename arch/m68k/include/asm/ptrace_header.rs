/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <uapi/asm/ptrace.h>
// This header is not intended for assembler consumers.

// #ifndef PS_S
pub const PS_S: u32 = 0x2000;
pub const PS_M: u32 = 0x1000;
// #endif

// #define user_mode(regs) (!((regs)->sr & PS_S))
#[macro_export]
macro_rules! user_mode {
    ($regs:expr) => {
        !(($regs).sr & $crate::PS_S != 0)
    };
}

// #define instruction_pointer(regs) ((regs)->pc)
#[macro_export]
macro_rules! instruction_pointer {
    ($regs:expr) => {
        ($regs).pc
    };
}

// #define profile_pc(regs) instruction_pointer(regs)
#[macro_export]
macro_rules! profile_pc {
    ($regs:expr) => {
        $crate::instruction_pointer!($regs)
    };
}

// #define current_pt_regs() \
//     (struct pt_regs *)((char *)current_thread_info() + THREAD_SIZE) - 1
#[macro_export]
macro_rules! current_pt_regs {
    () => {{
        (($crate::current_thread_info() as *mut u8)
            .wrapping_add($crate::THREAD_SIZE)
            as *mut $crate::pt_regs)
            .wrapping_sub(1)
    }};
}

// #define current_user_stack_pointer() rdusp()
#[macro_export]
macro_rules! current_user_stack_pointer {
    () => {
        $crate::rdusp()
    };
}

// #define arch_has_single_step() (1)
#[macro_export]
macro_rules! arch_has_single_step {
    () => {
        1
    };
}

// #ifdef CONFIG_MMU
// #define arch_has_block_step() (1)
#[macro_export]
macro_rules! arch_has_block_step {
    () => {
        1
    };
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
