/*
 *  linux/include/asm/traps.h
 *
 *  Copyright (C) 1993        Hamish Macdonald
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Translated from the C header; declarations below correspond to !__ASSEMBLER__. */

pub type EVector = unsafe extern "C" fn();

extern "C" {
    pub static mut vectors: [EVector; 0];
    pub static mut _ramvec: *mut EVector;
    pub fn auto_inthandler();
    pub fn user_inthandler();
    pub fn bad_inthandler();
}

pub const VEC_RESETSP: i32 = 0;
pub const VEC_RESETPC: i32 = 1;
pub const VEC_BUSERR: i32 = 2;
pub const VEC_ADDRERR: i32 = 3;
pub const VEC_ILLEGAL: i32 = 4;
pub const VEC_ZERODIV: i32 = 5;
pub const VEC_CHK: i32 = 6;
pub const VEC_TRAP: i32 = 7;
pub const VEC_PRIV: i32 = 8;
pub const VEC_TRACE: i32 = 9;
pub const VEC_LINE10: i32 = 10;
pub const VEC_LINE11: i32 = 11;
pub const VEC_RESV12: i32 = 12;
pub const VEC_COPROC: i32 = 13;
pub const VEC_FORMAT: i32 = 14;
pub const VEC_UNINT: i32 = 15;
pub const VEC_RESV16: i32 = 16;
pub const VEC_RESV17: i32 = 17;
pub const VEC_RESV18: i32 = 18;
pub const VEC_RESV19: i32 = 19;
pub const VEC_RESV20: i32 = 20;
pub const VEC_RESV21: i32 = 21;
pub const VEC_RESV22: i32 = 22;
pub const VEC_RESV23: i32 = 23;
pub const VEC_SPUR: i32 = 24;
pub const VEC_INT1: i32 = 25;
pub const VEC_INT2: i32 = 26;
pub const VEC_INT3: i32 = 27;
pub const VEC_INT4: i32 = 28;
pub const VEC_INT5: i32 = 29;
pub const VEC_INT6: i32 = 30;
pub const VEC_INT7: i32 = 31;
pub const VEC_SYS: i32 = 32;
pub const VEC_TRAP1: i32 = 33;
pub const VEC_TRAP2: i32 = 34;
pub const VEC_TRAP3: i32 = 35;
pub const VEC_TRAP4: i32 = 36;
pub const VEC_TRAP5: i32 = 37;
pub const VEC_TRAP6: i32 = 38;
pub const VEC_TRAP7: i32 = 39;
pub const VEC_TRAP8: i32 = 40;
pub const VEC_TRAP9: i32 = 41;
pub const VEC_TRAP10: i32 = 42;
pub const VEC_TRAP11: i32 = 43;
pub const VEC_TRAP12: i32 = 44;
pub const VEC_TRAP13: i32 = 45;
pub const VEC_TRAP14: i32 = 46;
pub const VEC_TRAP15: i32 = 47;
pub const VEC_FPBRUC: i32 = 48;
pub const VEC_FPIR: i32 = 49;
pub const VEC_FPDIVZ: i32 = 50;
pub const VEC_FPUNDER: i32 = 51;
pub const VEC_FPOE: i32 = 52;
pub const VEC_FPOVER: i32 = 53;
pub const VEC_FPNAN: i32 = 54;
pub const VEC_FPUNSUP: i32 = 55;
pub const VEC_MMUCFG: i32 = 56;
pub const VEC_MMUILL: i32 = 57;
pub const VEC_MMUACC: i32 = 58;
pub const VEC_RESV59: i32 = 59;
pub const VEC_UNIMPEA: i32 = 60;
pub const VEC_UNIMPII: i32 = 61;
pub const VEC_RESV62: i32 = 62;
pub const VEC_RESV63: i32 = 63;
pub const VEC_USER: i32 = 64;

#[inline]
pub const fn VECOFF(vec: i32) -> i32 { vec << 2 }

pub const PS_T: u16 = 0x8000;
pub const PS_S: u16 = 0x2000;
pub const PS_M: u16 = 0x1000;
pub const PS_C: u16 = 0x0001;
pub const FC: u16 = 0x8000;
pub const FB: u16 = 0x4000;
pub const RC: u16 = 0x2000;
pub const RB: u16 = 0x1000;
pub const DF: u16 = 0x0100;
pub const RM: u16 = 0x0080;
pub const RW: u16 = 0x0040;
pub const SZ: u16 = 0x0030;
pub const DFC: u16 = 0x0007;
pub const MMU_B: u16 = 0x8000;
pub const MMU_L: u16 = 0x4000;
pub const MMU_S: u16 = 0x2000;
pub const MMU_WP: u16 = 0x0800;
pub const MMU_I: u16 = 0x0400;
pub const MMU_M: u16 = 0x0200;
pub const MMU_T: u16 = 0x0040;
pub const MMU_NUM: u16 = 0x0007;

pub const CP_040: u16 = 0x8000; pub const CU_040: u16 = 0x4000; pub const CT_040: u16 = 0x2000; pub const CM_040: u16 = 0x1000;
pub const MA_040: u16 = 0x0800; pub const ATC_040: u16 = 0x0400; pub const LK_040: u16 = 0x0200; pub const RW_040: u16 = 0x0100;
pub const SIZ_040: u16 = 0x0060; pub const TT_040: u16 = 0x0018; pub const TM_040: u16 = 0x0007;
pub const WBV_040: u16 = 0x80; pub const WBSIZ_040: u16 = 0x60; pub const WBBYT_040: u16 = 0x20; pub const WBWRD_040: u16 = 0x40; pub const WBLNG_040: u16 = 0x00;
pub const WBTT_040: u16 = 0x18; pub const WBTM_040: u16 = 0x07;
pub const BA_SIZE_BYTE: u16 = 0x20; pub const BA_SIZE_WORD: u16 = 0x40; pub const BA_SIZE_LONG: u16 = 0x00; pub const BA_SIZE_LINE: u16 = 0x60;
pub const BA_TT_MOVE16: u16 = 0x08;
pub const MMU_B_040: u16 = 0x0800; pub const MMU_G_040: u16 = 0x0400; pub const MMU_S_040: u16 = 0x0080; pub const MMU_CM_040: u16 = 0x0060;
pub const MMU_M_040: u16 = 0x0010; pub const MMU_WP_040: u16 = 0x0004; pub const MMU_T_040: u16 = 0x0002; pub const MMU_R_040: u16 = 0x0001;

pub const MMU060_MA: u32 = 0x08000000; pub const MMU060_LK: u32 = 0x02000000; pub const MMU060_RW: u32 = 0x01800000;
pub const MMU060_RW_W: u32 = 0x00800000; pub const MMU060_RW_R: u32 = 0x01000000; pub const MMU060_RW_RMW: u32 = 0x01800000; pub const MMU060_W: u32 = 0x00800000;
pub const MMU060_SIZ: u32 = 0x00600000; pub const MMU060_TT: u32 = 0x00180000; pub const MMU060_TM: u32 = 0x00070000;
pub const MMU060_IO: u32 = 0x00008000; pub const MMU060_PBE: u32 = 0x00004000; pub const MMU060_SBE: u32 = 0x00002000; pub const MMU060_PTA: u32 = 0x00001000;
pub const MMU060_PTB: u32 = 0x00000800; pub const MMU060_IL: u32 = 0x00000400; pub const MMU060_PF: u32 = 0x00000200; pub const MMU060_SP: u32 = 0x00000100;
pub const MMU060_WP: u32 = 0x00000080; pub const MMU060_TWE: u32 = 0x00000040; pub const MMU060_RE: u32 = 0x00000020; pub const MMU060_WE: u32 = 0x00000010;
pub const MMU060_TTR: u32 = 0x00000008; pub const MMU060_BPE: u32 = 0x00000004; pub const MMU060_SEE: u32 = 0x00000001;
pub const MMU060_DESC_ERR: u32 = MMU060_PTA | MMU060_PTB | MMU060_IL | MMU060_PF;
pub const MMU060_ERR_BITS: u32 = MMU060_PBE | MMU060_SBE | MMU060_DESC_ERR | MMU060_SP | MMU060_WP | MMU060_TWE | MMU060_RE | MMU060_WE;

#[repr(C)] pub struct Fmt2 { pub iaddr: u32 }
#[repr(C)] pub struct Fmt3 { pub effaddr: u32 }
#[repr(C)] pub struct Fmt4 { pub effaddr: u32, pub pc: u32 }
#[repr(C)] pub struct Fmt7 { pub effaddr: u32, pub ssw: u16, pub wb3s: u16, pub wb2s: u16, pub wb1s: u16, pub faddr: u32, pub wb3a: u32, pub wb3d: u32, pub wb2a: u32, pub wb2d: u32, pub wb1a: u32, pub wb1dpd0: u32, pub pd1: u32, pub pd2: u32, pub pd3: u32 }
#[repr(C)] pub struct Fmt9 { pub iaddr: u32, pub int1: [u16; 4] }
#[repr(C)] pub struct Fmta { pub int1: u16, pub ssw: u16, pub isc: u16, pub isb: u16, pub daddr: u32, pub int2: [u16; 2], pub dobuf: u32, pub int3: [u16; 2] }
#[repr(C)] pub struct Fmtb { pub int1: u16, pub ssw: u16, pub isc: u16, pub isb: u16, pub daddr: u32, pub int2: [u16; 2], pub dobuf: u32, pub int3: [u16; 4], pub baddr: u32, pub int4: [u16; 2], pub dibuf: u32, pub int5: [u16; 3], pub ver_int6: u16, pub int7: [u16; 18] }
#[repr(C)] pub union FrameUnion { pub fmt2: Fmt2, pub fmt3: Fmt3, pub fmt4: Fmt4, pub fmt7: Fmt7, pub fmt9: Fmt9, pub fmta: Fmta, pub fmtb: Fmtb }
#[repr(C)] pub struct Frame { pub ptregs: PtRegs, pub un: FrameUnion }

/* struct pt_regs is supplied by asm/ptrace.h. */
#[allow(non_camel_case_types)] pub type PtRegs = crate::pt_regs;

#[cfg(CONFIG_M68040)]
extern "C" { pub fn berr_040cleanup(fp: *mut Frame); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
