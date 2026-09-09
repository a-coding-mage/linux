/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/math-emu.h.  C/assembler-only dependencies remain external. */

/* Status Register bits */
pub const FPSR_AEXC_INEX: u32 = 3;
pub const FPSR_AEXC_DZ: u32 = 4;
pub const FPSR_AEXC_UNFL: u32 = 5;
pub const FPSR_AEXC_OVFL: u32 = 6;
pub const FPSR_AEXC_IOP: u32 = 7;

pub const FPSR_EXC_INEX1: u32 = 8;
pub const FPSR_EXC_INEX2: u32 = 9;
pub const FPSR_EXC_DZ: u32 = 10;
pub const FPSR_EXC_UNFL: u32 = 11;
pub const FPSR_EXC_OVFL: u32 = 12;
pub const FPSR_EXC_OPERR: u32 = 13;
pub const FPSR_EXC_SNAN: u32 = 14;
pub const FPSR_EXC_BSUN: u32 = 15;

/* Big-endian quotient byte. */
#[inline]
pub unsafe fn fpsr_quotient(fpsr: *mut u32) -> *mut i8 {
    (fpsr as *mut u8).add(1) as *mut i8
}

pub const FPSR_CC_NAN: u32 = 24;
pub const FPSR_CC_INF: u32 = 25;
pub const FPSR_CC_Z: u32 = 26;
pub const FPSR_CC_NEG: u32 = 27;

pub const FPCR_ROUND_RN: u32 = 0; /* round to nearest/even */
pub const FPCR_ROUND_RZ: u32 = 1; /* round to zero */
pub const FPCR_ROUND_RM: u32 = 2; /* minus infinity */
pub const FPCR_ROUND_RP: u32 = 3; /* plus infinity */

pub const FPCR_PRECISION_X: u32 = 0; /* long double */
pub const FPCR_PRECISION_S: u32 = 1; /* double */
pub const FPCR_PRECISION_D: u32 = 2; /* float */

pub const PDECODE: u32 = 0;
pub const PEXECUTE: u32 = 1;
pub const PCONV: u32 = 2;
pub const PNORM: u32 = 3;
pub const PREGISTER: u32 = 4;
pub const PINSTR: u32 = 5;
pub const PUNIMPL: u32 = 6;
pub const PMOVEM: u32 = 7;

pub const PMDECODE: u32 = 1 << PDECODE;
pub const PMEXECUTE: u32 = 1 << PEXECUTE;
pub const PMCONV: u32 = 1 << PCONV;
pub const PMNORM: u32 = 1 << PNORM;
pub const PMREGISTER: u32 = 1 << PREGISTER;
pub const PMINSTR: u32 = 1 << PINSTR;
pub const PMUNIMPL: u32 = 1 << PUNIMPL;
pub const PMMOVEM: u32 = 1 << PMOVEM;

#[repr(C)]
pub union FpMant64 {
    pub m64: u64,
    pub m32: [usize; 2],
}

#[repr(C)]
pub union FpMant128 {
    pub m64: [u64; 2],
    pub m32: [usize; 4],
}

#[repr(C)]
pub struct FpExt {
    pub lowmant: u8,
    pub sign: u8,
    pub exp: u16,
    pub mant: FpMant64,
}

/* C representation of FPU registers. */
#[repr(C)]
pub struct FpData {
    pub fpreg: [FpExt; 8],
    pub fpcr: u32,
    pub fpsr: u32,
    pub fpiar: u32,
    pub prec: u16,
    pub rnd: u16,
    pub temp: [FpExt; 2],
}

#[cfg(feature = "FPU_EMU_DEBUG")]
extern "C" {
    pub static mut fp_debugprint: u32;
}

/* Kernel logging symbols supplied by the surrounding environment. */
extern "C" {
    pub fn pr_info(fmt: *const core::ffi::c_char, ...);
    pub fn pr_err(fmt: *const core::ffi::c_char, ...);
    pub fn no_printk(fmt: *const core::ffi::c_char, ...);
}

#[cfg(feature = "FPU_EMU_DEBUG")]
#[macro_export]
macro_rules! dprint {
    ($bit:expr, $($arg:tt)*) => {{
        if unsafe { $crate::fp_debugprint & (1u32 << ($bit)) } != 0 {
            unsafe { $crate::pr_info($($arg)*); }
        }
    }};
}

#[cfg(not(feature = "FPU_EMU_DEBUG"))]
#[macro_export]
macro_rules! dprint {
    ($bit:expr, $($arg:tt)*) => {{ let _ = ($bit, stringify!($($arg)*)); }};
}

/* The original uprint macro limits its diagnostic to three occurrences. */
#[macro_export]
macro_rules! uprint {
    ($str:expr) => {{
        static mut COUNT: i32 = 3;
        unsafe {
            if COUNT > 0 {
                $crate::pr_err(c"You just hit an unimplemented fpu instruction (%s)\n".as_ptr(), $str);
                $crate::pr_err(c"Please report this to ....\n".as_ptr());
                COUNT -= 1;
            }
        }
    }};
}

/* FPDATA is ((struct fp_data *)current->thread.fp) in the C kernel context. */
#[macro_export]
macro_rules! fpdata {
    ($current:expr) => {{
        ($current.thread.fp as *mut $crate::FpData)
    }};
}

/* The __ASSEMBLER__ branch consists solely of m68k assembler macros and offsets.
 * It has no executable Rust equivalent; the original assembler interface is
 * intentionally preserved here as documentation for the assembly consumer.
 * FPDATA=%a2; FPD_FPREG=TASK_THREAD+THREAD_FPREG+0; FPD_FPCR=...+96;
 * FPD_FPSR=...+100; FPD_FPIAR=...+104; FPD_PREC=...+108;
 * FPD_RND=...+110; FPD_TEMPFP1=...+112; FPD_TEMPFP2=...+124;
 * FPD_SIZEOF=...+136.  FPS_DO..FPS_PC map to PT_OFF_D0..PT_OFF_PC,
 * with FPS_EA=PT_OFF_PC+6 and FPS_PC2=PT_OFF_PC+10.
 * The fp_get_fp_reg, fp_get_pc, fp_put_pc, fp_get_instr_data,
 * fp_get_instr_word, fp_get_instr_long, getuser, putuser, movestack,
 * printf, printx, and debug macros retain their original assembler semantics.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
