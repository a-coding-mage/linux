/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency: __u32 and __u64 are supplied by the Linux types translation.

/* FP context was used */
pub const SC_USED_FP: __u32 = 1 << 0;
/* Address error was due to memory load */
pub const SC_ADDRERR_RD: __u32 = 1 << 30;
/* Address error was due to memory store */
pub const SC_ADDRERR_WR: __u32 = 1 << 31;

#[repr(C, align(16))]
pub struct SigcontextExtcontext([__u64; 0]);

#[repr(C)]
pub struct sigcontext {
    pub sc_pc: __u64,
    pub sc_regs: [__u64; 32],
    pub sc_flags: __u32,
    pub sc_extcontext: SigcontextExtcontext,
}

pub const CONTEXT_INFO_ALIGN: usize = 16;

#[repr(C)]
pub struct sctx_info {
    pub magic: __u32,
    pub size: __u32,
    pub padding: __u64, /* padding to 16 bytes */
}

/* FPU context */
pub const FPU_CTX_MAGIC: __u32 = 0x4650_5501;
pub const FPU_CTX_ALIGN: usize = 8;

#[repr(C)]
pub struct fpu_context {
    pub regs: [__u64; 32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

/* LSX context */
pub const LSX_CTX_MAGIC: __u32 = 0x5358_0001;
pub const LSX_CTX_ALIGN: usize = 16;

#[repr(C)]
pub struct lsx_context {
    pub regs: [__u64; 2 * 32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

/* LASX context */
pub const LASX_CTX_MAGIC: __u32 = 0x4153_5801;
pub const LASX_CTX_ALIGN: usize = 32;

#[repr(C)]
pub struct lasx_context {
    pub regs: [__u64; 4 * 32],
    pub fcc: __u64,
    pub fcsr: __u32,
}

/* LBT context */
pub const LBT_CTX_MAGIC: __u32 = 0x4254_0001;
pub const LBT_CTX_ALIGN: usize = 8;

#[repr(C)]
pub struct lbt_context {
    pub regs: [__u64; 4],
    pub eflags: __u32,
    pub ftop: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
