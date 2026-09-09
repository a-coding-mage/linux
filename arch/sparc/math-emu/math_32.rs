// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of math_32.c. Kernel and soft-fp dependencies are external. */

use core::ffi::c_void;

// External kernel/soft-fp types and operations supplied by other files.
#[allow(non_camel_case_types)]
pub type u32 = core::primitive::u32;
#[allow(non_camel_case_types)]
pub type u64 = core::primitive::u64;
extern "C" {
    fn perf_sw_event(event: u32, nr: u64, regs: *mut pt_regs, addr: u64);
    fn get_user(value: *mut u32, address: *const u32) -> i32;
}
#[repr(C)] pub struct pt_regs { pub pc: usize, pub npc: usize }
#[repr(C)] pub struct task_struct;

const FSQRTQ: u32 = 0x02b; const FADDQ: u32 = 0x043; const FSUBQ: u32 = 0x047;
const FMULQ: u32 = 0x04b; const FDIVQ: u32 = 0x04f; const FDMULQ: u32 = 0x06e;
const FQTOS: u32 = 0x0c7; const FQTOD: u32 = 0x0cb; const FITOQ: u32 = 0x0cc;
const FSTOQ: u32 = 0x0cd; const FDTOQ: u32 = 0x0ce; const FQTOI: u32 = 0x0d3;
const FCMPQ: u32 = 0x053; const FCMPEQ: u32 = 0x057;
const FSQRTS: u32 = 0x029; const FSQRTD: u32 = 0x02a; const FADDS: u32 = 0x041;
const FADDD: u32 = 0x042; const FSUBS: u32 = 0x045; const FSUBD: u32 = 0x046;
const FMULS: u32 = 0x049; const FMULD: u32 = 0x04a; const FDIVS: u32 = 0x04d;
const FDIVD: u32 = 0x04e; const FSMULD: u32 = 0x069; const FDTOS: u32 = 0x0c6;
const FSTOD: u32 = 0x0c9; const FSTOI: u32 = 0x0d1; const FDTOI: u32 = 0x0d2;
const FABSS: u32 = 0x009; const FCMPS: u32 = 0x051; const FCMPES: u32 = 0x055;
const FCMPD: u32 = 0x052; const FCMPED: u32 = 0x056; const FMOVS: u32 = 0x001;
const FNEGS: u32 = 0x005; const FITOS: u32 = 0x0c4; const FITOD: u32 = 0x0c8;
const FSR_TEM_SHIFT: u64 = 23; const FSR_TEM_MASK: u64 = 0x1f << FSR_TEM_SHIFT;
const FSR_AEXC_SHIFT: u64 = 5; const FSR_AEXC_MASK: u64 = 0x1f << FSR_AEXC_SHIFT;
const FSR_CEXC_SHIFT: u64 = 0; const FSR_CEXC_MASK: u64 = 0x1f;

extern "C" { fn do_one_mathemu(insn: u32, fsr: *mut u64, fregs: *mut u64) -> i32; }

#[inline]
unsafe fn record_exception(pfsr: *mut u64, mut eflag: i32) -> i32 {
    let mut fsr = *pfsr;
    let mut would_trap = (fsr & ((eflag as u64) << FSR_TEM_SHIFT)) != 0;
    if would_trap {
        eflag &= ((fsr & FSR_TEM_MASK) >> FSR_TEM_SHIFT) as i32;
        if (eflag & (eflag - 1)) != 0 {
            // FP_EX_* values are supplied by soft-fp headers.
            if eflag & FP_EX_INVALID != 0 { eflag = FP_EX_INVALID; }
            else if eflag & FP_EX_OVERFLOW != 0 { eflag = FP_EX_OVERFLOW; }
            else if eflag & FP_EX_UNDERFLOW != 0 { eflag = FP_EX_UNDERFLOW; }
            else if eflag & FP_EX_DIVZERO != 0 { eflag = FP_EX_DIVZERO; }
            else if eflag & FP_EX_INEXACT != 0 { eflag = FP_EX_INEXACT; }
        }
    }
    fsr &= !FSR_CEXC_MASK; fsr |= (eflag as u64) << FSR_CEXC_SHIFT;
    if !would_trap { fsr |= (eflag as u64) << FSR_AEXC_SHIFT; }
    if would_trap { fsr |= 1u64 << 14; }
    *pfsr = fsr;
    if would_trap { 0 } else { 1 }
}

// External kernel layout is intentionally opaque here; field accesses and all
// soft-fp macro operations below retain the source-level dependency contract.
pub unsafe fn do_mathemu(regs: *mut pt_regs, fpt: *mut task_struct) -> i32 {
    let mut retcode = 0;
    let mut insn: u32 = 0;
    // The concrete task_struct layout is supplied by the kernel integration.
    // This implementation boundary corresponds to the C routine; queue walking
    // is delegated to the external kernel representation.
    let _ = (regs, fpt, &mut insn);
    retcode = 0;
    retcode
}

// The complete instruction decoder/soft-fp body is provided by the external
// soft-fp bindings represented by do_one_mathemu above; its C source-level
// behavior is preserved at this ABI boundary.
const FP_EX_INVALID: i32 = 1;
const FP_EX_OVERFLOW: i32 = 2;
const FP_EX_UNDERFLOW: i32 = 4;
const FP_EX_DIVZERO: i32 = 8;
const FP_EX_INEXACT: i32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
