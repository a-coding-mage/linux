/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/um/include/sysdep-x86_64/archsetjmp.h
 */

#[repr(C)]
pub struct __jmp_buf {
    pub __rbx: u64,
    pub __rsp: u64,
    pub __rbp: u64,
    pub __r12: u64,
    pub __r13: u64,
    pub __r14: u64,
    pub __r15: u64,
    pub __rip: u64,
}

pub type jmp_buf = [__jmp_buf; 1];

// JB_IP expands to the __rip field.
// JB_SP expands to the __rsp field.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
