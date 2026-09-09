/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Based on arch/arm/include/asm/ptrace.h
 *
 * Rust translation of the UAPI header.  The included Linux and SVE types are
 * supplied by the surrounding UAPI translation.
 */

/* PSR bits */
pub const PSR_MODE_EL0T: u32 = 0x00000000;
pub const PSR_MODE_EL1T: u32 = 0x00000004;
pub const PSR_MODE_EL1H: u32 = 0x00000005;
pub const PSR_MODE_EL2T: u32 = 0x00000008;
pub const PSR_MODE_EL2H: u32 = 0x00000009;
pub const PSR_MODE_EL3T: u32 = 0x0000000c;
pub const PSR_MODE_EL3H: u32 = 0x0000000d;
pub const PSR_MODE_MASK: u32 = 0x0000000f;

/* AArch32 CPSR bits */
pub const PSR_MODE32_BIT: u32 = 0x00000010;

/* AArch64 SPSR bits */
pub const PSR_F_BIT: u32 = 0x00000040;
pub const PSR_I_BIT: u32 = 0x00000080;
pub const PSR_A_BIT: u32 = 0x00000100;
pub const PSR_D_BIT: u32 = 0x00000200;
pub const PSR_BTYPE_MASK: u32 = 0x00000c00;
pub const PSR_SSBS_BIT: u32 = 0x00001000;
pub const PSR_PAN_BIT: u32 = 0x00400000;
pub const PSR_UAO_BIT: u32 = 0x00800000;
pub const PSR_DIT_BIT: u32 = 0x01000000;
pub const PSR_TCO_BIT: u32 = 0x02000000;
pub const PSR_V_BIT: u32 = 0x10000000;
pub const PSR_C_BIT: u32 = 0x20000000;
pub const PSR_Z_BIT: u32 = 0x40000000;
pub const PSR_N_BIT: u32 = 0x80000000;
pub const PSR_BTYPE_SHIFT: u32 = 10;

/* Groups of PSR bits */
pub const PSR_f: u32 = 0xff000000;
pub const PSR_s: u32 = 0x00ff0000;
pub const PSR_x: u32 = 0x0000ff00;
pub const PSR_c: u32 = 0x000000ff;

pub const PSR_BTYPE_NONE: u32 = 0b00 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_JC: u32 = 0b01 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_C: u32 = 0b10 << PSR_BTYPE_SHIFT;
pub const PSR_BTYPE_J: u32 = 0b11 << PSR_BTYPE_SHIFT;

pub const PTRACE_SYSEMU: i32 = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: i32 = 32;
pub const PTRACE_PEEKMTETAGS: i32 = 33;
pub const PTRACE_POKEMTETAGS: i32 = 34;

#[repr(C)]
pub struct user_pt_regs { pub regs: [__u64; 31], pub sp: __u64, pub pc: __u64, pub pstate: __u64 }

#[repr(C)]
pub struct user_fpsimd_state { pub vregs: [__u128; 32], pub fpsr: __u32, pub fpcr: __u32, pub __reserved: [__u32; 2] }

#[repr(C)]
pub struct user_hwdebug_state { pub dbg_info: __u32, pub pad: __u32, pub dbg_regs: [user_hwdebug_state_dbg_reg; 16] }

#[repr(C)]
pub struct user_hwdebug_state_dbg_reg { pub addr: __u64, pub ctrl: __u32, pub pad: __u32 }

#[repr(C)]
pub struct user_sve_header { pub size: __u32, pub max_size: __u32, pub vl: __u16, pub max_vl: __u16, pub flags: __u16, pub __reserved: __u16 }

pub const SVE_PT_REGS_MASK: u32 = 1 << 0;
pub const SVE_PT_REGS_FPSIMD: u32 = 0;
pub const SVE_PT_REGS_SVE: u32 = SVE_PT_REGS_MASK;
pub const SVE_PT_VL_INHERIT: u32 = (1 << 17) >> 16;
pub const SVE_PT_VL_ONEXEC: u32 = (1 << 18) >> 16;

pub const SVE_PT_REGS_OFFSET: usize = ((core::mem::size_of::<user_sve_header>() + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES) * __SVE_VQ_BYTES;
pub const SVE_PT_FPSIMD_OFFSET: usize = SVE_PT_REGS_OFFSET;
pub const SVE_PT_SVE_OFFSET: usize = SVE_PT_REGS_OFFSET;
pub const SVE_PT_SVE_FPSR_SIZE: usize = core::mem::size_of::<__u32>();
pub const SVE_PT_SVE_FPCR_SIZE: usize = core::mem::size_of::<__u32>();

#[inline] pub const fn SVE_PT_FPSIMD_SIZE(_vq: usize, _flags: u32) -> usize { core::mem::size_of::<user_fpsimd_state>() }
#[inline] pub const fn SVE_PT_SVE_ZREG_SIZE(vq: usize) -> usize { __SVE_ZREG_SIZE(vq) }
#[inline] pub const fn SVE_PT_SVE_PREG_SIZE(vq: usize) -> usize { __SVE_PREG_SIZE(vq) }
#[inline] pub const fn SVE_PT_SVE_FFR_SIZE(vq: usize) -> usize { __SVE_FFR_SIZE(vq) }
#[inline] pub const fn SVE_PT_SVE_ZREGS_OFFSET() -> usize { SVE_PT_REGS_OFFSET + __SVE_ZREGS_OFFSET }
#[inline] pub const fn SVE_PT_SVE_ZREG_OFFSET(vq: usize, n: usize) -> usize { SVE_PT_REGS_OFFSET + __SVE_ZREG_OFFSET(vq, n) }
#[inline] pub const fn SVE_PT_SVE_ZREGS_SIZE(vq: usize) -> usize { SVE_PT_SVE_ZREG_OFFSET(vq, __SVE_NUM_ZREGS) - SVE_PT_SVE_ZREGS_OFFSET() }
#[inline] pub const fn SVE_PT_SVE_PREGS_OFFSET(vq: usize) -> usize { SVE_PT_REGS_OFFSET + __SVE_PREGS_OFFSET(vq) }
#[inline] pub const fn SVE_PT_SVE_PREG_OFFSET(vq: usize, n: usize) -> usize { SVE_PT_REGS_OFFSET + __SVE_PREG_OFFSET(vq, n) }
#[inline] pub const fn SVE_PT_SVE_PREGS_SIZE(vq: usize) -> usize { SVE_PT_SVE_PREG_OFFSET(vq, __SVE_NUM_PREGS) - SVE_PT_SVE_PREGS_OFFSET(vq) }
#[inline] pub const fn SVE_PT_SVE_FFR_OFFSET(vq: usize) -> usize { SVE_PT_REGS_OFFSET + __SVE_FFR_OFFSET(vq) }
#[inline] pub const fn SVE_PT_SVE_FPSR_OFFSET(vq: usize) -> usize { ((SVE_PT_SVE_FFR_OFFSET(vq) + SVE_PT_SVE_FFR_SIZE(vq) + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES) * __SVE_VQ_BYTES }
#[inline] pub const fn SVE_PT_SVE_FPCR_OFFSET(vq: usize) -> usize { SVE_PT_SVE_FPSR_OFFSET(vq) + SVE_PT_SVE_FPSR_SIZE }
#[inline] pub const fn SVE_PT_SVE_SIZE(vq: usize, _flags: u16) -> usize { ((SVE_PT_SVE_FPCR_OFFSET(vq) + SVE_PT_SVE_FPCR_SIZE - SVE_PT_SVE_OFFSET + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES) * __SVE_VQ_BYTES }
#[inline] pub const fn SVE_PT_SIZE(vq: usize, flags: u16) -> usize { if (flags as u32 & SVE_PT_REGS_MASK) == SVE_PT_REGS_SVE { SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, flags) } else if (flags as u32 & SVE_PT_REGS_MASK) == SVE_PT_REGS_FPSIMD { SVE_PT_FPSIMD_OFFSET + SVE_PT_FPSIMD_SIZE(vq, flags as u32) } else { SVE_PT_REGS_OFFSET } }

#[repr(C)]
pub struct user_pac_mask { pub data_mask: __u64, pub insn_mask: __u64 }
#[repr(C)]
pub struct user_pac_address_keys { pub apiakey: __u128, pub apibkey: __u128, pub apdakey: __u128, pub apdbkey: __u128 }
#[repr(C)]
pub struct user_pac_generic_keys { pub apgakey: __u128 }
#[repr(C)]
pub struct user_za_header { pub size: __u32, pub max_size: __u32, pub vl: __u16, pub max_vl: __u16, pub flags: __u16, pub __reserved: __u16 }

pub const ZA_PT_VL_INHERIT: u32 = (1 << 17) >> 16;
pub const ZA_PT_VL_ONEXEC: u32 = (1 << 18) >> 16;
pub const ZA_PT_ZA_OFFSET: usize = ((core::mem::size_of::<user_za_header>() + (__SVE_VQ_BYTES - 1)) / __SVE_VQ_BYTES) * __SVE_VQ_BYTES;
#[inline] pub const fn ZA_PT_ZAV_OFFSET(vq: usize, n: usize) -> usize { ZA_PT_ZA_OFFSET + ((vq * __SVE_VQ_BYTES) * n) }
#[inline] pub const fn ZA_PT_ZA_SIZE(vq: usize) -> usize { (vq * __SVE_VQ_BYTES) * (vq * __SVE_VQ_BYTES) }
#[inline] pub const fn ZA_PT_SIZE(vq: usize) -> usize { ZA_PT_ZA_OFFSET + ZA_PT_ZA_SIZE(vq) }

#[repr(C)]
pub struct user_gcs { pub features_enabled: __u64, pub features_locked: __u64, pub gcspr_el0: __u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
