/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* Translated from the Linux UAPI header; linux/types.h supplies the original
 * architecture-dependent integer types. */

/*
 * For PTRACE_{POKE,PEEK}USR. 0 - 31 are GPRs,
 * 32 is syscall's original ARG0, 33 is PC, 34 is BADVADDR.
 */
pub const GPR_BASE: usize = 0;
pub const GPR_NUM: usize = 32;
pub const GPR_END: usize = GPR_BASE + GPR_NUM - 1;
pub const ARG0: usize = GPR_END + 1;
pub const PC: usize = GPR_END + 2;
pub const BADVADDR: usize = GPR_END + 3;

pub const NUM_FPU_REGS: usize = 32;

#[repr(C, align(8))]
pub struct user_pt_regs {
    /* Main processor registers. */
    pub regs: [core::ffi::c_ulong; 32],

    /* Original syscall arg0. */
    pub orig_a0: core::ffi::c_ulong,

    /* Special CSR registers. */
    pub csr_era: core::ffi::c_ulong,
    pub csr_badv: core::ffi::c_ulong,
    pub reserved: [core::ffi::c_ulong; 10],
}

#[repr(C)]
pub struct user_fp_state {
    pub fpr: [u64; 32],
    pub fcc: u64,
    pub fcsr: u32,
}

#[repr(C)]
pub struct user_lsx_state {
    /* 32 registers, 128 bits width per register. */
    pub vregs: [u64; 32 * 2],
}

#[repr(C)]
pub struct user_lasx_state {
    /* 32 registers, 256 bits width per register. */
    pub vregs: [u64; 32 * 4],
}

#[repr(C)]
pub struct user_lbt_state {
    pub scr: [u64; 4],
    pub eflags: u32,
    pub ftop: u32,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct user_watch_dbg_reg {
    pub addr: u32,
    pub mask: u32,
    pub ctrl: u32,
    pub pad: u32,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct user_watch_dbg_reg {
    pub addr: u64,
    pub mask: u64,
    pub ctrl: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct user_watch_state {
    pub dbg_info: u64,
    pub dbg_regs: [user_watch_dbg_reg; 8],
}

#[repr(C)]
pub struct user_watch_state_v2 {
    pub dbg_info: u64,
    pub dbg_regs: [user_watch_dbg_reg; 14],
}

pub const PTRACE_SYSEMU: u32 = 0x1f;
pub const PTRACE_SYSEMU_SINGLESTEP: u32 = 0x20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
