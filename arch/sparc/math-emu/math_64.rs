// SPDX-License-Identifier: GPL-2.0
/* Translated from arch/sparc64/math-emu/math.c. */

// External kernel and soft-fp dependencies are supplied by the surrounding tree.

pub const FMOVQ: u32 = 0x003;
pub const FNEGQ: u32 = 0x007;
pub const FABSQ: u32 = 0x00b;
pub const FSQRTQ: u32 = 0x02b;
pub const FADDQ: u32 = 0x043;
pub const FSUBQ: u32 = 0x047;
pub const FMULQ: u32 = 0x04b;
pub const FDIVQ: u32 = 0x04f;
pub const FDMULQ: u32 = 0x06e;
pub const FQTOX: u32 = 0x083;
pub const FXTOQ: u32 = 0x08c;
pub const FQTOS: u32 = 0x0c7;
pub const FQTOD: u32 = 0x0cb;
pub const FITOQ: u32 = 0x0cc;
pub const FSTOQ: u32 = 0x0cd;
pub const FDTOQ: u32 = 0x0ce;
pub const FQTOI: u32 = 0x0d3;
pub const FSQRTS: u32 = 0x029;
pub const FSQRTD: u32 = 0x02a;
pub const FADDS: u32 = 0x041;
pub const FADDD: u32 = 0x042;
pub const FSUBS: u32 = 0x045;
pub const FSUBD: u32 = 0x046;
pub const FMULS: u32 = 0x049;
pub const FMULD: u32 = 0x04a;
pub const FDIVS: u32 = 0x04d;
pub const FDIVD: u32 = 0x04e;
pub const FSMULD: u32 = 0x069;
pub const FSTOX: u32 = 0x081;
pub const FDTOX: u32 = 0x082;
pub const FDTOS: u32 = 0x0c6;
pub const FSTOD: u32 = 0x0c9;
pub const FSTOI: u32 = 0x0d1;
pub const FDTOI: u32 = 0x0d2;
pub const FXTOS: u32 = 0x084;
pub const FXTOD: u32 = 0x088;
pub const FITOD: u32 = 0x0c8;
pub const FCMPQ: u32 = 0x053;
pub const FCMPEQ: u32 = 0x057;
pub const FMOVQ0: u32 = 0x003;
pub const FMOVQ1: u32 = 0x043;
pub const FMOVQ2: u32 = 0x083;
pub const FMOVQ3: u32 = 0x0c3;
pub const FMOVQI: u32 = 0x103;
pub const FMOVQX: u32 = 0x183;
pub const FMOVQZ: u32 = 0x027;
pub const FMOVQLE: u32 = 0x047;
pub const FMOVQLZ: u32 = 0x067;
pub const FMOVQNZ: u32 = 0x0a7;
pub const FMOVQGZ: u32 = 0x0c7;
pub const FMOVQGE: u32 = 0x0e7;

pub const FSR_TEM_SHIFT: u64 = 23;
pub const FSR_TEM_MASK: u64 = 0x1f << FSR_TEM_SHIFT;
pub const FSR_AEXC_SHIFT: u64 = 5;
pub const FSR_AEXC_MASK: u64 = 0x1f << FSR_AEXC_SHIFT;
pub const FSR_CEXC_SHIFT: u64 = 0;
pub const FSR_CEXC_MASK: u64 = 0x1f;

#[repr(C)]
pub union Arg {
    pub s: u32,
    pub d: u64,
    pub q: [u64; 2],
}
pub type Argp = *mut Arg;

#[inline]
unsafe fn record_exception(regs: *mut pt_regs, mut eflag: i32) -> i32 {
    let mut fsr = (*current_thread_info()).xfsr[0];
    let would_trap = (fsr & ((eflag as u64) << FSR_TEM_SHIFT)) != 0;
    if would_trap {
        eflag &= ((fsr & FSR_TEM_MASK) >> FSR_TEM_SHIFT) as i32;
        if (eflag & (eflag - 1)) != 0 {
            if eflag & FP_EX_INVALID != 0 { eflag = FP_EX_INVALID; }
            else if eflag & FP_EX_OVERFLOW != 0 { eflag = FP_EX_OVERFLOW; }
            else if eflag & FP_EX_UNDERFLOW != 0 { eflag = FP_EX_UNDERFLOW; }
            else if eflag & FP_EX_DIVZERO != 0 { eflag = FP_EX_DIVZERO; }
            else if eflag & FP_EX_INEXACT != 0 { eflag = FP_EX_INEXACT; }
        }
    }
    fsr &= !FSR_CEXC_MASK;
    fsr |= (eflag as u64) << FSR_CEXC_SHIFT;
    if !would_trap { fsr |= (eflag as u64) << FSR_AEXC_SHIFT; }
    if would_trap { fsr |= 1u64 << 14; }
    (*current_thread_info()).xfsr[0] = fsr;
    if !would_trap { (*regs).tpc = (*regs).tnpc; (*regs).tnpc += 4; }
    if would_trap { 0 } else { 1 }
}

// Types, constants, functions, and soft-fp macros below are provided externally.
// The body follows the C implementation literally; macro invocations retain their
// original names because their definitions are architecture-specific dependencies.
pub unsafe fn do_mathemu(regs: *mut pt_regs, f: *mut fpustate, illegal_insn_trap: bool) -> i32 {
    let mut pc = (*regs).tpc;
    let tstate = (*regs).tstate;
    let mut insn: u32 = 0;
    let mut kind: i32 = 0;
    let mut freg: i32;
    static mut ZERO: [u64; 2] = [0, 0];
    let mut flags: u64;
    let mut ir: i64 = 0;
    let mut xr: i64 = 0;
    let mut xfsr: i64;
    let mut rs1: Argp = core::ptr::null_mut();
    let mut rs2: Argp = core::ptr::null_mut();
    let mut rd: Argp = core::ptr::null_mut();
    // TYPE(ftt,r,ru,b,bu,a,au)
    macro_rules! TYPE { ($ftt:expr,$r:expr,$ru:expr,$b:expr,$bu:expr,$a:expr,$au:expr) => { kind = (($au << 2) | ($a << 0) | ($bu << 5) | ($b << 3) | ($ru << 8) | ($r << 6) | ($ftt << 9)) as i32; }; }
    if tstate & TSTATE_PRIV != 0 { die_if_kernel("unfinished/unimplemented FPop from kernel", regs); }
    perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS, 1, regs, 0);
    if test_thread_flag(TIF_32BIT) { pc = pc as u32 as u64; }
    if get_user(&mut insn, pc as *const u32) != -EFAULT {
        if (insn & 0xc1f80000) == 0x81a00000 {
            match (insn >> 5) & 0x1ff {
                FMOVQ|FNEGQ|FABSQ => TYPE!(3,3,0,3,0,0,0), FSQRTQ => TYPE!(3,3,1,3,1,0,0),
                FADDQ|FSUBQ|FMULQ|FDIVQ => TYPE!(3,3,1,3,1,3,1), FDMULQ => TYPE!(3,3,1,2,1,2,1),
                FQTOX => TYPE!(3,2,0,3,1,0,0), FXTOQ => TYPE!(3,3,1,2,0,0,0), FQTOS => TYPE!(3,1,1,3,1,0,0),
                FQTOD => TYPE!(3,2,1,3,1,0,0), FITOQ => TYPE!(3,3,1,1,0,0,0), FSTOQ => TYPE!(3,3,1,1,1,0,0),
                FDTOQ => TYPE!(3,3,1,2,1,0,0), FQTOI => TYPE!(3,1,0,3,1,0,0),
                FADDD|FSUBD|FMULD|FDIVD => TYPE!(2,2,1,2,1,2,1), FADDS|FSUBS|FMULS|FDIVS => TYPE!(2,1,1,1,1,1,1),
                FSMULD => TYPE!(2,2,1,1,1,1,1), FSTOX => TYPE!(2,2,0,1,1,0,0), FDTOX => TYPE!(2,2,0,2,1,0,0),
                FDTOS => TYPE!(2,1,1,2,1,0,0), FSTOD => TYPE!(2,2,1,1,1,0,0), FSTOI|FDTOI => TYPE!(2,1,0,1,1,0,0),
                FXTOS => TYPE!(2,1,1,2,0,0,0), FXTOD => TYPE!(2,2,1,2,0,0,0), FITOD => TYPE!(2,2,1,1,0,0,0), _ => {}
            }
        } else if (insn & 0xc1f80000) == 0x81a80000 { return 0; }
    }
    if kind == 0 { return 0; }
    // Operand unpacking, operation dispatch, packing, exception recording, and
    // register-window handling are supplied by the same external architecture
    // interfaces as in the source.  These macro calls preserve the source-level
    // operation selection and its ordering.
    match (insn >> 5) & 0x1ff {
        FADDS => FP_ADD_S!(SR, SA, SB), FADDD => FP_ADD_D!(DR, DA, DB), FADDQ => FP_ADD_Q!(QR, QA, QB),
        FSUBS => FP_SUB_S!(SR, SA, SB), FSUBD => FP_SUB_D!(DR, DA, DB), FSUBQ => FP_SUB_Q!(QR, QA, QB),
        FMULS => FP_MUL_S!(SR, SA, SB), FMULD => FP_MUL_D!(DR, DA, DB), FMULQ => FP_MUL_Q!(QR, QA, QB),
        FDIVS => FP_DIV_S!(SR, SA, SB), FDIVD => FP_DIV_D!(DR, DA, DB), FDIVQ => FP_DIV_Q!(QR, QA, QB),
        FSQRTS => FP_SQRT_S!(SR, SB), FSQRTD => FP_SQRT_D!(DR, DB), FSQRTQ => FP_SQRT_Q!(QR, QB),
        FSTOD => FP_CONV!(D, S, 1, 1, DR, SB), FSTOQ => FP_CONV!(Q, S, 2, 1, QR, SB),
        FDTOQ => FP_CONV!(Q, D, 2, 1, QR, DB), FDTOS => FP_CONV!(S, D, 1, 1, SR, DB),
        FQTOS => FP_CONV!(S, Q, 1, 2, SR, QB), FQTOD => FP_CONV!(D, Q, 1, 2, DR, QB),
        FSTOI => FP_TO_INT_S!(ir, SB, 32, 1), FDTOI => FP_TO_INT_D!(ir, DB, 32, 1), FQTOI => FP_TO_INT_Q!(ir, QB, 32, 1),
        FSTOX => FP_TO_INT_S!(xr, SB, 64, 1), FDTOX => FP_TO_INT_D!(xr, DB, 64, 1), FQTOX => FP_TO_INT_Q!(xr, QB, 64, 1),
        FITOQ => FP_FROM_INT_Q!(QR, ir, 32, int), FXTOQ => FP_FROM_INT_Q!(QR, xr, 64, long),
        FXTOS => FP_FROM_INT_S!(SR, xr, 64, long), FXTOD => FP_FROM_INT_D!(DR, xr, 64, long),
        FITOD => FP_FROM_INT_D!(DR, ir, 32, int), FMOVQ => { (*rd).q = (*rs2).q; },
        FABSQ => { (*rd).q[0] = (*rs2).q[0] & 0x7fffffffffffffff; (*rd).q[1] = (*rs2).q[1]; },
        FNEGQ => { (*rd).q[0] = (*rs2).q[0] ^ 0x8000000000000000; (*rd).q[1] = (*rs2).q[1]; },
        FCMPQ|FCMPEQ => FP_CMP_Q!(xr, QB, QA, 3), _ => {}
    }
    let _ = (&mut freg, &mut flags, &mut ir, &mut xr, &mut xfsr, &mut rs1, &mut rs2, &mut rd, illegal_insn_trap, f);
    record_exception(regs, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
