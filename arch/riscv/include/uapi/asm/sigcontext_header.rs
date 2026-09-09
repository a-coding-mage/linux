/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependency supplied by <asm/ptrace.h>.

/* The Magic number for signal context frame header. */
pub const RISCV_V_MAGIC: u32 = 0x5346_5457;
pub const RISCV_ZICFISS_MAGIC: u32 = 0x9487;
pub const END_MAGIC: u32 = 0x0;

/* The size of END signal context header. */
pub const END_HDR_SIZE: u32 = 0x0;

#[repr(C, align(16))]
pub struct __sc_riscv_v_state {
    pub v_state: __riscv_v_ext_state,
}

/*
 * Signal context structure
 *
 * This contains the context saved before a signal handler is invoked;
 * it is restored by sys_rt_sigreturn.
 */
#[repr(C)]
pub struct sigcontext {
    pub sc_regs: user_regs_struct,
    pub _anon: sigcontext__bindgen_ty_1,
}

#[repr(C)]
pub union sigcontext__bindgen_ty_1 {
    pub sc_fpregs: ::core::mem::ManuallyDrop<__riscv_fp_state>,
    pub sc_extdesc: ::core::mem::ManuallyDrop<__riscv_extra_ext_header>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
