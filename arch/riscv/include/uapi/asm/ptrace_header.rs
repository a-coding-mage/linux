/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

/* Translated from the non-assembler portion of riscv/include/uapi/asm/ptrace.h. */

pub const PTRACE_GETFDPIC: u32 = 33;

pub const PTRACE_GETFDPIC_EXEC: u32 = 0;
pub const PTRACE_GETFDPIC_INTERP: u32 = 1;

/*
 * User-mode register state for core dumps, ptrace, sigcontext
 *
 * This decouples struct pt_regs from the userspace ABI.
 * struct user_regs_struct must form a prefix of struct pt_regs.
 */
#[repr(C)]
pub struct user_regs_struct {
    pub pc: kernel::ffi::c_ulong,
    pub ra: kernel::ffi::c_ulong,
    pub sp: kernel::ffi::c_ulong,
    pub gp: kernel::ffi::c_ulong,
    pub tp: kernel::ffi::c_ulong,
    pub t0: kernel::ffi::c_ulong,
    pub t1: kernel::ffi::c_ulong,
    pub t2: kernel::ffi::c_ulong,
    pub s0: kernel::ffi::c_ulong,
    pub s1: kernel::ffi::c_ulong,
    pub a0: kernel::ffi::c_ulong,
    pub a1: kernel::ffi::c_ulong,
    pub a2: kernel::ffi::c_ulong,
    pub a3: kernel::ffi::c_ulong,
    pub a4: kernel::ffi::c_ulong,
    pub a5: kernel::ffi::c_ulong,
    pub a6: kernel::ffi::c_ulong,
    pub a7: kernel::ffi::c_ulong,
    pub s2: kernel::ffi::c_ulong,
    pub s3: kernel::ffi::c_ulong,
    pub s4: kernel::ffi::c_ulong,
    pub s5: kernel::ffi::c_ulong,
    pub s6: kernel::ffi::c_ulong,
    pub s7: kernel::ffi::c_ulong,
    pub s8: kernel::ffi::c_ulong,
    pub s9: kernel::ffi::c_ulong,
    pub s10: kernel::ffi::c_ulong,
    pub s11: kernel::ffi::c_ulong,
    pub t3: kernel::ffi::c_ulong,
    pub t4: kernel::ffi::c_ulong,
    pub t5: kernel::ffi::c_ulong,
    pub t6: kernel::ffi::c_ulong,
}

#[repr(C)]
pub struct __riscv_f_ext_state {
    pub f: [u32; 32],
    pub fcsr: u32,
}

#[repr(C)]
pub struct __riscv_d_ext_state {
    pub f: [u64; 32],
    pub fcsr: u32,
}

#[repr(C, align(16))]
pub struct __riscv_q_ext_state {
    pub f: [u64; 64],
    pub fcsr: u32,
    /* Reserved for expansion of sigcontext structure. Currently zeroed
     * upon signal, and must be zero upon sigreturn. */
    pub reserved: [u32; 3],
}

#[repr(C)]
pub struct __riscv_ctx_hdr {
    pub magic: u32,
    pub size: u32,
}

#[repr(C, align(16))]
pub struct __riscv_extra_ext_header {
    pub __padding: [u32; 129],
    /* Reserved for expansion of sigcontext structure. Currently zeroed
     * upon signal, and must be zero upon sigreturn. */
    pub reserved: u32,
    pub hdr: __riscv_ctx_hdr,
}

#[repr(C)]
pub union __riscv_fp_state {
    pub f: __riscv_f_ext_state,
    pub d: __riscv_d_ext_state,
    pub q: __riscv_q_ext_state,
}

#[repr(C)]
pub struct __riscv_v_ext_state {
    pub vstart: kernel::ffi::c_ulong,
    pub vl: kernel::ffi::c_ulong,
    pub vtype: kernel::ffi::c_ulong,
    pub vcsr: kernel::ffi::c_ulong,
    pub vlenb: kernel::ffi::c_ulong,
    pub datap: *mut kernel::ffi::c_void,
    /* In signal handler, datap will be set a correct user stack offset
     * and vector registers will be copied to the address of datap pointer. */
}

#[repr(C)]
pub struct __riscv_v_regset_state {
    pub vstart: kernel::ffi::c_ulong,
    pub vl: kernel::ffi::c_ulong,
    pub vtype: kernel::ffi::c_ulong,
    pub vcsr: kernel::ffi::c_ulong,
    pub vlenb: kernel::ffi::c_ulong,
    pub vreg: [kernel::ffi::c_char; 0],
}

/* VLEN >= ELEN, a power of 2, and no greater than 2^16 bits = 8192 bytes. */
pub const RISCV_MAX_VLENB: usize = 8192;

#[repr(C)]
pub struct __sc_riscv_cfi_state {
    pub ss_ptr: kernel::ffi::c_ulong, /* shadow stack pointer */
}

pub const PTRACE_CFI_BRANCH_LANDING_PAD_EN_BIT: usize = 0;
pub const PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_BIT: usize = 1;
pub const PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_BIT: usize = 2;
pub const PTRACE_CFI_SHADOW_STACK_EN_BIT: usize = 3;
pub const PTRACE_CFI_SHADOW_STACK_LOCK_BIT: usize = 4;
pub const PTRACE_CFI_SHADOW_STACK_PTR_BIT: usize = 5;

pub const PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_BRANCH_LANDING_PAD_EN_BIT;
pub const PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_BIT;
pub const PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_BIT;
pub const PTRACE_CFI_SHADOW_STACK_EN_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_SHADOW_STACK_EN_BIT;
pub const PTRACE_CFI_SHADOW_STACK_LOCK_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_SHADOW_STACK_LOCK_BIT;
pub const PTRACE_CFI_SHADOW_STACK_PTR_STATE: kernel::ffi::c_ulong = 1 << PTRACE_CFI_SHADOW_STACK_PTR_BIT;

pub const PTRACE_CFI_STATE_INVALID_MASK: kernel::ffi::c_ulong =
    !(PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE
        | PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_STATE
        | PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE
        | PTRACE_CFI_SHADOW_STACK_EN_STATE
        | PTRACE_CFI_SHADOW_STACK_LOCK_STATE
        | PTRACE_CFI_SHADOW_STACK_PTR_STATE);

#[repr(C)]
pub struct __cfi_status {
    pub cfi_state: u64,
}

#[repr(C)]
pub struct user_cfi_state {
    pub cfi_status: __cfi_status,
    pub shstk_ptr: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
