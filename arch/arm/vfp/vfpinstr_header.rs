/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/vfp/vfpinstr.h
 *
 *  Copyright (C) 2004 ARM Limited.
 *  Written by Deep Blue Solutions Limited.
 *
 * VFP instruction masks.
 */

#[inline]
pub const fn INST_CPRTDO(inst: u32) -> bool { (inst & 0x0f000000) == 0x0e000000 }
#[inline]
pub const fn INST_CPRT(inst: u32) -> u32 { inst & (1 << 4) }
#[inline]
pub const fn INST_CPRT_L(inst: u32) -> u32 { inst & (1 << 20) }
#[inline]
pub const fn INST_CPRT_Rd(inst: u32) -> u32 { (inst & (15 << 12)) >> 12 }
#[inline]
pub const fn INST_CPRT_OP(inst: u32) -> u32 { (inst >> 21) & 7 }
#[inline]
pub const fn INST_CPNUM(inst: u32) -> u32 { inst & 0xf00 }
#[inline]
pub const fn CPNUM(cp: u32) -> u32 { cp << 8 }

pub const FOP_MASK: u32 = 0x00b00040;
pub const FOP_FMAC: u32 = 0x00000000;
pub const FOP_FNMAC: u32 = 0x00000040;
pub const FOP_FMSC: u32 = 0x00100000;
pub const FOP_FNMSC: u32 = 0x00100040;
pub const FOP_FMUL: u32 = 0x00200000;
pub const FOP_FNMUL: u32 = 0x00200040;
pub const FOP_FADD: u32 = 0x00300000;
pub const FOP_FSUB: u32 = 0x00300040;
pub const FOP_FDIV: u32 = 0x00800000;
pub const FOP_EXT: u32 = 0x00b00040;

#[inline]
pub const fn FOP_TO_IDX(inst: u32) -> u32 { ((inst & 0x00b00000) >> 20) | ((inst & (1 << 6)) >> 4) }

pub const FEXT_MASK: u32 = 0x000f0080;
pub const FEXT_FCPY: u32 = 0x00000000;
pub const FEXT_FABS: u32 = 0x00000080;
pub const FEXT_FNEG: u32 = 0x00010000;
pub const FEXT_FSQRT: u32 = 0x00010080;
pub const FEXT_FCMP: u32 = 0x00040000;
pub const FEXT_FCMPE: u32 = 0x00040080;
pub const FEXT_FCMPZ: u32 = 0x00050000;
pub const FEXT_FCMPEZ: u32 = 0x00050080;
pub const FEXT_FCVT: u32 = 0x00070080;
pub const FEXT_FUITO: u32 = 0x00080000;
pub const FEXT_FSITO: u32 = 0x00080080;
pub const FEXT_FTOUI: u32 = 0x000c0000;
pub const FEXT_FTOUIZ: u32 = 0x000c0080;
pub const FEXT_FTOSI: u32 = 0x000d0000;
pub const FEXT_FTOSIZ: u32 = 0x000d0080;

#[inline]
pub const fn FEXT_TO_IDX(inst: u32) -> u32 { ((inst & 0x000f0000) >> 15) | ((inst & (1 << 7)) >> 7) }

#[inline]
pub const fn vfp_get_sd(inst: u32) -> u32 { ((inst & 0x0000f000) >> 11) | ((inst & (1 << 22)) >> 22) }
#[inline]
pub const fn vfp_get_dd(inst: u32) -> u32 { ((inst & 0x0000f000) >> 12) | ((inst & (1 << 22)) >> 18) }
#[inline]
pub const fn vfp_get_sm(inst: u32) -> u32 { ((inst & 0x0000000f) << 1) | ((inst & (1 << 5)) >> 5) }
#[inline]
pub const fn vfp_get_dm(inst: u32) -> u32 { (inst & 0x0000000f) | ((inst & (1 << 5)) >> 1) }
#[inline]
pub const fn vfp_get_sn(inst: u32) -> u32 { ((inst & 0x000f0000) >> 15) | ((inst & (1 << 7)) >> 7) }
#[inline]
pub const fn vfp_get_dn(inst: u32) -> u32 { ((inst & 0x000f0000) >> 16) | ((inst & (1 << 7)) >> 3) }

#[inline]
pub const fn vfp_single(inst: u32) -> bool { (inst & 0x0000f00) == 0xa00 }

pub const FPSCR_N: u32 = 1 << 31;
pub const FPSCR_Z: u32 = 1 << 30;
pub const FPSCR_C: u32 = 1 << 29;
pub const FPSCR_V: u32 = 1 << 28;

#[macro_export]
macro_rules! fmrx {
    ($vfp:ident) => {{
        let mut __v: u32;
        unsafe {
            core::arch::asm!(concat!(".fpu\tvfpv2\n", "vmrs\t{0}, ", stringify!($vfp)), out(reg) __v, options(nostack, preserves_flags));
        }
        __v
    }};
}

#[macro_export]
macro_rules! fmxr {
    ($vfp:ident, $var:expr) => {{
        unsafe {
            core::arch::asm!(concat!(".fpu\tvfpv2\n", "vmsr\t", stringify!($vfp), ", {0}"), in(reg) $var, options(nostack, preserves_flags));
        }
    }};
}

extern "C" {
    pub fn vfp_single_cpdo(inst: u32, fpscr: u32) -> u32;
    pub fn vfp_single_cprt(inst: u32, fpscr: u32, regs: *mut crate::pt_regs) -> u32;
    pub fn vfp_double_cpdo(inst: u32, fpscr: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
