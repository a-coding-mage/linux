/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux AArch64 UAPI sigcontext header. */

/*
 * Signal context structure - contains all info to do with the state
 * before the signal handler was invoked.
 */
#[repr(C)]
pub struct sigcontext {
    pub fault_address: u64,
    /* AArch64 registers */
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
    /* 4K reserved for FP/SIMD state and future expansion */
    pub __reserved: [u8; 4096], // __attribute__((aligned(16)))
}

/*
 * Header to be used at the beginning of structures extending the user
 * context. Such structures must be placed after the rt_sigframe on the stack
 * and be 16-byte aligned. The last structure must be a dummy one with the
 * magic and size set to 0.
 */
#[repr(C)]
pub struct _aarch64_ctx {
    pub magic: u32,
    pub size: u32,
}

pub const FPSIMD_MAGIC: u32 = 0x46508001;

#[repr(C)]
pub struct fpsimd_context {
    pub head: _aarch64_ctx,
    pub fpsr: u32,
    pub fpcr: u32,
    pub vregs: [u128; 32],
}

/* ESR_EL1 context */
pub const ESR_MAGIC: u32 = 0x45535201;

#[repr(C)]
pub struct esr_context {
    pub head: _aarch64_ctx,
    pub esr: u64,
}

pub const POE_MAGIC: u32 = 0x504f4530;

#[repr(C)]
pub struct poe_context {
    pub head: _aarch64_ctx,
    pub por_el0: u64,
}

pub const EXTRA_MAGIC: u32 = 0x45585401;

#[repr(C)]
pub struct extra_context {
    pub head: _aarch64_ctx,
    pub datap: u64, /* 16-byte aligned pointer to extra space cast to __u64 */
    pub size: u32, /* size in bytes of the extra space */
    pub __reserved: [u32; 3],
}

pub const SVE_MAGIC: u32 = 0x53564501;

#[repr(C)]
pub struct sve_context {
    pub head: _aarch64_ctx,
    pub vl: u16,
    pub flags: u16,
    pub __reserved: [u16; 2],
}

pub const SVE_SIG_FLAG_SM: u32 = 0x1; /* Context describes streaming mode */

/* TPIDR2_EL0 context */
pub const TPIDR2_MAGIC: u32 = 0x54504902;

#[repr(C)]
pub struct tpidr2_context {
    pub head: _aarch64_ctx,
    pub tpidr2: u64,
}

/* FPMR context */
pub const FPMR_MAGIC: u32 = 0x46504d52;

#[repr(C)]
pub struct fpmr_context {
    pub head: _aarch64_ctx,
    pub fpmr: u64,
}

pub const ZA_MAGIC: u32 = 0x54366345;

#[repr(C)]
pub struct za_context {
    pub head: _aarch64_ctx,
    pub vl: u16,
    pub __reserved: [u16; 3],
}

pub const ZT_MAGIC: u32 = 0x5a544e01;

#[repr(C)]
pub struct zt_context {
    pub head: _aarch64_ctx,
    pub nregs: u16,
    pub __reserved: [u16; 3],
}

pub const GCS_MAGIC: u32 = 0x47435300;

#[repr(C)]
pub struct gcs_context {
    pub head: _aarch64_ctx,
    pub gcspr: u64,
    pub features_enabled: u64,
    pub reserved: u64,
}

/* SVE architecture constants and helpers are supplied by asm/sve_context.h. */
pub const SVE_VQ_BYTES: usize = __SVE_VQ_BYTES;
pub const SVE_VQ_MIN: usize = __SVE_VQ_MIN;
pub const SVE_VQ_MAX: usize = __SVE_VQ_MAX;
pub const SVE_VL_MIN: usize = __SVE_VL_MIN;
pub const SVE_VL_MAX: usize = __SVE_VL_MAX;
pub const SVE_NUM_ZREGS: usize = __SVE_NUM_ZREGS;
pub const SVE_NUM_PREGS: usize = __SVE_NUM_PREGS;

#[macro_export]
macro_rules! sve_vl_valid { ($vl:expr) => { __sve_vl_valid($vl) }; }
#[macro_export]
macro_rules! sve_vq_from_vl { ($vl:expr) => { __sve_vq_from_vl($vl) }; }
#[macro_export]
macro_rules! sve_vl_from_vq { ($vq:expr) => { __sve_vl_from_vq($vq) }; }

#[macro_export]
macro_rules! SVE_SIG_ZREG_SIZE { ($vq:expr) => { __SVE_ZREG_SIZE($vq) }; }
#[macro_export]
macro_rules! SVE_SIG_PREG_SIZE { ($vq:expr) => { __SVE_PREG_SIZE($vq) }; }
#[macro_export]
macro_rules! SVE_SIG_FFR_SIZE { ($vq:expr) => { __SVE_FFR_SIZE($vq) }; }

#[macro_export]
macro_rules! SVE_SIG_REGS_OFFSET { () => { ( (core::mem::size_of::<$crate::sve_context>() + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES * __SVE_VQ_BYTES ) }; }
#[macro_export]
macro_rules! SVE_SIG_ZREGS_OFFSET { () => { (SVE_SIG_REGS_OFFSET!() + __SVE_ZREGS_OFFSET) }; }
#[macro_export]
macro_rules! SVE_SIG_ZREG_OFFSET { ($vq:expr, $n:expr) => { (SVE_SIG_REGS_OFFSET!() + __SVE_ZREG_OFFSET($vq, $n)) }; }
#[macro_export]
macro_rules! SVE_SIG_ZREGS_SIZE { ($vq:expr) => { __SVE_ZREGS_SIZE($vq) }; }
#[macro_export]
macro_rules! SVE_SIG_PREGS_OFFSET { ($vq:expr) => { (SVE_SIG_REGS_OFFSET!() + __SVE_PREGS_OFFSET($vq)) }; }
#[macro_export]
macro_rules! SVE_SIG_PREG_OFFSET { ($vq:expr, $n:expr) => { (SVE_SIG_REGS_OFFSET!() + __SVE_PREG_OFFSET($vq, $n)) }; }
#[macro_export]
macro_rules! SVE_SIG_PREGS_SIZE { ($vq:expr) => { __SVE_PREGS_SIZE($vq) }; }
#[macro_export]
macro_rules! SVE_SIG_FFR_OFFSET { ($vq:expr) => { (SVE_SIG_REGS_OFFSET!() + __SVE_FFR_OFFSET($vq)) }; }
#[macro_export]
macro_rules! SVE_SIG_REGS_SIZE { ($vq:expr) => { (__SVE_FFR_OFFSET($vq) + __SVE_FFR_SIZE($vq)) }; }
#[macro_export]
macro_rules! SVE_SIG_CONTEXT_SIZE { ($vq:expr) => { (SVE_SIG_REGS_OFFSET!() + SVE_SIG_REGS_SIZE!($vq)) }; }

#[macro_export]
macro_rules! ZA_SIG_REGS_OFFSET { () => { ((core::mem::size_of::<$crate::za_context>() + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES * __SVE_VQ_BYTES) }; }
#[macro_export]
macro_rules! ZA_SIG_REGS_SIZE { ($vq:expr) => { (($vq * __SVE_VQ_BYTES) * ($vq * __SVE_VQ_BYTES)) }; }
#[macro_export]
macro_rules! ZA_SIG_ZAV_OFFSET { ($vq:expr, $n:expr) => { (ZA_SIG_REGS_OFFSET!() + (SVE_SIG_ZREG_SIZE!($vq) * $n)) }; }
#[macro_export]
macro_rules! ZA_SIG_CONTEXT_SIZE { ($vq:expr) => { (ZA_SIG_REGS_OFFSET!() + ZA_SIG_REGS_SIZE!($vq)) }; }

pub const ZT_SIG_REG_SIZE: usize = 512;
pub const ZT_SIG_REG_BYTES: usize = ZT_SIG_REG_SIZE / 8;
pub const ZT_SIG_REGS_OFFSET: usize = core::mem::size_of::<zt_context>();
#[macro_export]
macro_rules! ZT_SIG_REGS_SIZE { ($n:expr) => { (ZT_SIG_REG_BYTES * $n) }; }
#[macro_export]
macro_rules! ZT_SIG_CONTEXT_SIZE { ($n:expr) => { (core::mem::size_of::<$crate::zt_context>() + ZT_SIG_REGS_SIZE!($n)) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
